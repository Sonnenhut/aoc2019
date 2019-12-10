use aoc2019::read_lines;
use std::iter::successors;
use std::cmp::Ordering::Equal;

fn main() {
    let input = parse_asteroids(&read_lines(10).join("\n"));
    println!("pt1: {}", pt1(&input).1); // 214
    println!("pt2: {}", pt2(&input)); // 502
}

fn pt1(asteroids: &Vec<Point>) -> (Point, usize) {
    asteroids.iter()
        .map(|p| (*p, cnt_in_sight(p,asteroids)))
        .max_by(|a,b| a.1.cmp(&b.1))
        .unwrap()
}

fn pt2(asteroids: &Vec<Point>) -> isize {
    in_kill_order(&asteroids)
        .iter().skip(199).next().map(|p| p.x * 100 + p.y).unwrap()
}

fn in_kill_order(asteroids: &Vec<Point>) -> Vec<Point> {
    let lazer = pt1(&asteroids).0;
    let mut remainder : Vec<Point> = asteroids.to_vec();
    let mut res : Vec<Point> = vec![];

    loop {
        let removed : Vec<Point> = one_360(&lazer, &remainder);
        remainder = remainder.iter().filter(|other| !removed.contains(other)).cloned().collect();
        res = [res, removed.to_vec()].concat();
        if removed.len() == 0 {
            break;
        }
    }
    res
}

fn one_360(lazer: &Point, asteroids: &Vec<Point>) -> Vec<Point> {
    let cmp_deg = |a,b| lazer.degrees_360(&a).partial_cmp(&lazer.degrees_360(&b)).unwrap();
    let mut remainder : Vec<Point> = asteroids.iter().cloned().filter(|other| *other != *lazer).collect();
    remainder.sort_by(|a,b| cmp_deg(*a,*b) // first by degrees
        .then_with(|| lazer.distance(a).partial_cmp(&lazer.distance(b)).unwrap()) // then by distance
    );

    let mut res: Vec<Point> = vec![];
    while remainder.len() > 0 {
        let next : Vec<Point> = successors(Some(remainder.remove(0)), |last| {
            if remainder.get(0).filter(|&succ| cmp_deg(*last, *succ) == Equal).is_some() {
                Some(remainder.remove(0))
            } else {None}
        }).collect(); // block of next values
        res.push(next[0]);
    }
    res
}

fn cnt_in_sight(loc: &Point, asteroids: &Vec<Point>) -> usize {
    let mut in_sight : Vec<f64> = asteroids.iter()
        .filter(|other| *other != loc)
        .map(|other| loc.degrees_360(other))
        .collect();
    in_sight.sort_by(|a,b|a.partial_cmp(b).unwrap());
    in_sight.dedup_by(|a,b|a.partial_cmp(&b).unwrap() == Equal);
    in_sight.len()
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
    fn degrees_360(&self, other: &Point) -> f64 {
        let x_relate_to_self = other.x as f64 - self.x as f64;
        let y_relate_to_self = other.y as f64 - self.y as f64;
        let mut degrees = y_relate_to_self.atan2(x_relate_to_self).to_degrees();
        degrees = degrees + 90.0; // shift 0° to be the y axis
        degrees = if degrees >= 0.0 { degrees } else { 360.0 + degrees }; // turn into 360° instead of 90, -90 etc
        degrees
    }
}

#[cfg(test)]
mod test {
    use super::*;

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

    #[test]
    fn test_atan2() {
        assert_eq!(Point::new(0,0).degrees_360(&Point::new(0,-1)),0.0);
        assert_eq!(Point::new(0,0).degrees_360(&Point::new(0,1)),180.0);
        assert_eq!(Point::new(0,0).degrees_360(&Point::new(1,0)),90.0);
        assert_eq!(Point::new(0,0).degrees_360(&Point::new(-1,0)),270.0);

        assert_eq!(Point::new(8,3).degrees_360(&Point::new(9,3)),90.0);
        assert_eq!(Point::new(8,3).degrees_360(&Point::new(7,3)),270.0);
        assert_eq!(Point::new(8,3).degrees_360(&Point::new(8,4)),180.0);
        assert_eq!(Point::new(8,3).degrees_360(&Point::new(8,2)),0.0);

        assert_eq!(Point::new(8,3).degrees_360(&Point::new(8,0)),0.0);
    }
    #[test]
    fn test_one_360_ex() {
        let ex =
            ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";
        let mut asteroids = parse_asteroids(&String::from(ex));
        let (lazer,_) = pt1(&asteroids);
        let mut removed = one_360(&lazer, &asteroids);
        assert_eq!(removed.len(), 30);

        asteroids = asteroids.iter().filter(|other| !removed.contains(&other)).cloned().collect();
        removed = one_360(&lazer, &asteroids);
        assert_eq!(removed.len(), 5);

        asteroids = asteroids.iter().filter(|other| !removed.contains(&other)).cloned().collect();
        removed = one_360(&lazer, &asteroids);
        assert_eq!(removed.len(), 1);
    }

    #[test]
    fn test_turn_360_pt2_ex() {
        let mut ex =
            ".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";
        let mut asteroids = parse_asteroids(&String::from(ex));
        assert_eq!(in_kill_order(&asteroids).len(), 36);
ex =
".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";
        asteroids = parse_asteroids(&String::from(ex));
        let remove_order = in_kill_order(&asteroids);
        assert_eq!(remove_order.len(), 299);
        assert_eq!(remove_order.iter().skip(199).next().unwrap().clone(), Point::new(8,2));
        assert_eq!(pt2(&asteroids), 802);
    }
}