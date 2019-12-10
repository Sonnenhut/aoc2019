use aoc2019::read_lines;
use aoc2019::intcode::IntCode;
use std::collections::HashSet;
use std::iter::successors;
use std::pin::Pin;

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

fn turn_360(asteroids: &Vec<Point>) -> Vec<Point> {
    let res : Vec<Point> = vec![];
    let mut remainder : Vec<Point> = asteroids.iter().cloned().collect();
    let lazer = pt1(&asteroids).0;

    let first = start_360(&lazer, &asteroids);

    remainder = remainder.into_iter().filter(|other| *other != lazer && *other != first).collect();

    successors(Some(first), |last| {
        if remainder.is_empty() {
            None
        } else {
            let next = next_clockwise(&lazer, &last, &remainder);
            remainder = remainder.iter().cloned().filter(|other| *other != next).collect();

            println!("{:?}", next);
            Some(next)
        }
    })
        .collect()
}

fn start_360(lazer : &Point, asteroids: &Vec<Point>) -> Point {
    let initial_lazer_end = Point::new(lazer.x, lazer.y - 999); // upwards from lazer
    let at_start_line : Vec<Point> = asteroids.iter().cloned()
        .filter(|possible| *possible != *lazer)
        .filter(|possible| possible.on_line(&lazer, &initial_lazer_end))
        .collect();
    sorted_by_distance(&lazer, &initial_lazer_end, &at_start_line)[0]
}

fn next_clockwise(line_start : &Point, line_end : &Point, others: &Vec<Point>) -> Point {
    let cw : Vec<Point> = others.iter()
        .filter(|possible| possible.is_clockwise_to(&line_start, &line_end))
        .cloned().collect();
    sorted_by_distance(&line_start, &line_end, &cw)[0]
}

fn sorted_by_distance(line_start : &Point, line_end : &Point, others: &Vec<Point>) -> Vec<Point> {
    let mut res : Vec<Point> = others.iter().cloned().collect();
    res.sort_by(|a,b| a.distance_to_line(&line_start, &line_end).partial_cmp(&b.distance_to_line(&line_start, &line_end)).unwrap());
    res
}

fn cnt_in_sight(loc: &Point, asteroids: &Vec<Point>) -> usize {
    // count the ones that are obstructed by another
    let not_in_sight : HashSet<Point> = asteroids.iter()
        .flat_map(|line_end| {
            let res = asteroids.iter()
                .filter(|other| *other != line_end && *other != loc)
                .filter(|maybe_between| maybe_between.on_line(loc, line_end))
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
    fn distance_to_line(&self, p1: &Point, p2: &Point) -> f64 {
        distance_to_line(self.x as f64, self.y as f64,p1.x as f64, p2.y as f64,p2.x as f64, p2.y as f64)
    }
    fn on_line(&self, line_start: &Point, line_end: &Point) -> bool {
        let d1 = line_start.distance(self);
        let d2 = line_end.distance(self);
        let d3 = line_start.distance(line_end);

        let res = approx_eq((d1 + d2), d3, 0.0000001);
        //println!("{:?} -- {:?} -- {:?} = {}; ({}) + ({}) = ({})", line_start, self, line_end, res,d1,d2,d3);
        //println!("{:?} -- {:?} -- {:?} = {}", line_start, self, line_end, res);
        res
    }
    fn is_clockwise_to(&self, p1: &Point, p2: &Point) -> bool {
        let p3 = self.clone();
        let val = (p2.y - p1.y) * (p3.x - p2.x) -
                    (p2.x - p1.x) * (p3.y - p2.y);
        return val > 0;
    }
}

//whereas x,y is the desired point, and 1,2 are the points on a line
fn distance_to_line(x: f64, y: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let a = x - x1;
    let b = y - y1;
    let c = x2 - x1;
    let d= y2 - y1;

    let dot = a * c + b * d;
    let len_sq = c * c + d * d;
    let mut param = -1.0;
    if len_sq != 0.0 {
        param = dot/ len_sq
    }
    let mut xx = 0.0;
    let mut yy = 0.0;

    if param < 0.0{
        xx = x1;
        yy = y1;
    } else if param > 1.0 {
        xx = x2;
        yy = y2;
    } else {
        xx = x1 + param * c;
        yy = y1 + param * d;
    }

    let dx = x - xx;
    let dy = y - yy;

    (dx * dx + dy * dy).sqrt()
}

fn distance_f64(x: f64, y: f64,x1: f64, y1: f64,) -> f64 {
    ((x - x1).powf(2.0) as f64 + (y - y1).powf(2.0) as f64).sqrt()
}

fn approx_eq(a: f64, b: f64, delta: f64) -> bool{
    (a-b).abs() < delta
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_on_line() {
        assert_eq!(Point::new(2,2).on_line(&Point::new(3,4), &Point::new(1,0)), true);
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

    #[test]
    fn test_clockwise() {
        assert_eq!(Point::new(1,0).is_clockwise_to(&Point::new(0,0), &Point::new(4,4)), true);
        assert_eq!(Point::new(2,1).is_clockwise_to(&Point::new(0,0), &Point::new(4,4)), true);
        assert_eq!(Point::new(3,1).is_clockwise_to(&Point::new(0,0), &Point::new(4,4)), true);

        assert_eq!(Point::new(0,1).is_clockwise_to(&Point::new(0,0), &Point::new(4,4)), false);
        assert_eq!(Point::new(1,2).is_clockwise_to(&Point::new(0,0), &Point::new(4,4)), false);
        assert_eq!(Point::new(1,3).is_clockwise_to(&Point::new(0,0), &Point::new(4,4)), false);

        assert_eq!(Point::new(3,3).is_clockwise_to(&Point::new(0,0), &Point::new(4,4)), false); // on same line (colinear)
        assert_eq!(Point::new(2,2).is_clockwise_to(&Point::new(0,0), &Point::new(4,4)), false);
        assert_eq!(Point::new(1,1).is_clockwise_to(&Point::new(0,0), &Point::new(4,4)), false);
        assert_eq!(Point::new(0,0).is_clockwise_to(&Point::new(0,0), &Point::new(4,4)), false);

        // flipping the line around, gives the opposite results
        assert_eq!(Point::new(1,0).is_clockwise_to(&Point::new(4,4), &Point::new(0,0)), false);
        assert_eq!(Point::new(2,1).is_clockwise_to(&Point::new(4,4), &Point::new(0,0)), false);
        assert_eq!(Point::new(3,1).is_clockwise_to(&Point::new(4,4), &Point::new(0,0)), false);

        assert_eq!(Point::new(0,1).is_clockwise_to(&Point::new(4,4), &Point::new(0,0)), true);
        assert_eq!(Point::new(1,2).is_clockwise_to(&Point::new(4,4), &Point::new(0,0)), true);
        assert_eq!(Point::new(1,3).is_clockwise_to(&Point::new(4,4), &Point::new(0,0)), true);
    }

    #[test]
    fn test_distance() {
        assert_eq!(distance_to_line(2.0,2.0, 0.0,0.0,0.0,4.0), 2.0);
        assert_eq!(distance_to_line(3.0,3.0, 0.0,0.0,0.0,4.0), 3.0);
        assert_eq!(distance_to_line(1.0,1.0, 0.0,0.0,1.0,10.0), 0.8955334711889903);
    }
    #[test]
    fn test_next_clockwise() {
        let others  = vec![Point::new(1,1),Point::new(1,2),Point::new(5,1),Point::new(5,2),Point::new(3,5),Point::new(1,0)];
        assert_eq!(next_clockwise(&Point::new(1,0), &Point::new(3,5), &others), Point::new(5,2));
        assert_eq!(next_clockwise(&Point::new(3,5), &Point::new(1,0), &others), Point::new(1,1));

        let others  = vec![Point::new(1,1)];
        assert_eq!(next_clockwise(&Point::new(1,0), &Point::new(3,5), &others), Point::new(5,2));
        assert_eq!(next_clockwise(&Point::new(3,5), &Point::new(1,0), &others), Point::new(1,1));
    }
    #[test]
    fn test_360() {
        let ex =
".#....#####...#..
##...##.#####..##
##...#...#.#####.
..#.....#...###..
..#.#.....#....##";
        let asteroids = parse_asteroids(&String::from(ex));
        assert_eq!(turn_360(&asteroids), vec![]);
    }
}