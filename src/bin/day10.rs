use aoc2019::read_lines;
use aoc2019::intcode::IntCode;
use std::collections::HashSet;

fn main() {
    let input = parse_asteroids(&read_lines(10).join("\n"));
    println!("pt1: {}", pt1(&input).1); // 214
    //println!("pt2: {}", IntCode::resolve(&vec![2], &nums)); //
}

fn pt1(asteroids: &Vec<Point>) -> (Point, usize) {
    asteroids.iter()
        .map(|p| (p.clone(), cnt_in_sight(p,asteroids)))
        .max_by(|a,b| a.1.cmp(&b.1))
        .unwrap()
}
/*
fn eradicate_360_with_lazer(asteroids: &Vec<Point>) -> Vec<Point>{
    let mut res = vec![];
    let lazer = Point::new(8,3);
    let mut lazer_pointing_up = Point {y: lazer.y-25, ..lazer};
    let mut remaining : Vec<Point> = asteroids.clone().iter().cloned().filter(|other| *other != lazer).collect();

    for deg in 0..360*10 {
        let lazer_end = lazer_pointing_up.rotate_around(&lazer,((deg/10) as f64).to_radians());
        let mut possibles : Vec<Point> = remaining.iter().cloned()
            .filter(|possible| possible.on_line(&lazer, &lazer_end, 0.001))
            .collect();

        if possibles.len() > 0 {
            possibles.sort_by(|a,b| a.distance(&lazer).partial_cmp(&b.distance(&lazer)).unwrap());
            remaining = remaining.iter().cloned().filter(|other| !possibles.contains(other)).collect();
            println!("{:?}", possibles);

            res = [res, possibles].concat();
        }
    }
    println!("{}", res.len());
    res
}
*/

fn cnt_in_sight(loc: &Point, asteroids: &Vec<Point>) -> usize {
    // count the ones that are obstructed by another
    let not_in_sight : HashSet<Point> = asteroids.iter()
        .flat_map(|line_end| {
            let res = asteroids.iter()
                .filter(|other| *other != line_end && *other != loc)
                .filter(|maybe_between| maybe_between.on_line(loc, line_end, 0.0000001))
                .cloned()
                .collect::<HashSet<Point>>();
            //println!("{:?}", res);
            res
        })
        .collect();
    //println!("{:?}", not_in_sight);
    asteroids.len() - not_in_sight.len() - 1
}

fn parse_asteroids(s: &String) -> Vec<Point>{
    s.lines()
        .enumerate()
        .flat_map(|(y,line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x,c)| if c == '#' { Some(Point::new(x as isize,y as isize)) } else { None })
        }).collect()
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn new(x: isize, y: isize) -> Point {
        Point{x, y}
    }
    fn distance(&self,other: &Point) -> f64 {
        let res = ((self.x - other.x).pow(2) as f64 + (self.y - other.y).pow(2) as f64).sqrt();
        res
    }
    fn on_line(&self, line_start: &Point, line_end: &Point, accuracy: f64) -> bool {
        let d1 = line_start.distance(self);
        let d2 = line_end.distance(self);
        let d3 = line_start.distance(line_end);

        let res = approx_eq((d1 + d2), d3, accuracy);
        //println!("{:?} -- {:?} -- {:?} = {}; ({}) + ({}) = ({})", line_start, self, line_end, res,d1,d2,d3);
        //println!("{:?} -- {:?} -- {:?} = {}", line_start, self, line_end, res);
        res
    }
    /*
    fn rotate_around(&self, pivot: &Point, angle_in_radian: f64) -> Point {
        let ox = pivot.x.clone() as f64;
        let oy = pivot.y.clone() as f64;
        let px = self.x.clone() as f64;
        let py = self.y.clone() as f64;
        let sin = angle_in_radian.sin();
        let cos = angle_in_radian.cos();

        let qx = ox + cos * (px - ox) - sin * (py - oy);
        let qy = oy + sin * (px - ox) + cos * (py - oy);

        Point::new(qx.round() as isize, qy.round() as isize)
    }
    */
}

fn approx_eq(a: f64, b: f64, delta: f64) -> bool{
    (a-b).abs() < delta
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_on_line() {
        assert_eq!(Point::new(2,2).on_line(&Point::new(3,4), &Point::new(1,0),0.001), true);
    }
    #[test]
    fn parse_test() {
let ex =
".#..#
.....
#####
....#
...##";
        assert_eq!(parse_asteroids(&String::from(ex)), vec![(1,0),(4,0),(0,2),(1,2),(2,2),(3,2),(4,2),(4,3),(3,4),(4,4)].iter().map(|t|Point::new(t.0, t.1)).collect::<Vec<Point>>());
    }

    #[test]
    fn test_pt1_ex() {
        let ex =
            ".#..#
.....
#####
....#
...##";
        let ast = parse_asteroids(&String::from(ex));
        assert_eq!(cnt_in_sight(&Point::new(3,4),&ast), 8);
        assert_eq!(cnt_in_sight(&Point::new(1,0),&ast), 7); // <- this one has floating point issues by 0.000000xx1
        assert_eq!(cnt_in_sight(&Point::new(4,0),&ast), 7);
        assert_eq!(cnt_in_sight(&Point::new(0,2),&ast), 6);
        assert_eq!(cnt_in_sight(&Point::new(1,2),&ast), 7);
        assert_eq!(pt1(&parse_asteroids(&String::from(ex))).1, 8);
    }
    /*
    #[test]
    fn test_rotate() {
        assert_eq!(Point::new(21,20).rotate_around(&Point::new(20,20),90_f64.to_radians()), Point::new(20,21));
        let ex =
".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....X...###..
..#.#.....#....##";
        eradicate_360_with_lazer(&parse_asteroids(&String::from(ex)));
    }
    */
}