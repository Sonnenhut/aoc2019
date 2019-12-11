use std::collections::{HashSet, HashMap};

use aoc2019::read_lines;
use std::time::Instant;

fn main() {
    let nums: Vec<Vec<String>> = read_lines(3).into_iter().map(|l| parse_wire(&l)).collect();
    println!("pt1: {}", pt1(&nums)); // 375
    println!("pt2: {}", pt2(&nums)); // 14746
}
fn pt1(wire_definitions: &Vec<Vec<String>>) -> u32 {
    let all_points : Vec<Point> = wire_definitions.iter().flat_map(|wire| {
        let p = path(wire);
        unique(&p)
    }).collect();
    duplicates(&all_points).iter().map(|p| p.distance_to_center()).min().unwrap_or_default()
}

fn pt2(wire_definitions: &Vec<Vec<String>>) -> u32 {
    let wires : Vec<Vec<Point>> = wire_definitions.iter().map(|wire| path(wire)).collect();
    let wires_without_dupes : Vec<Point> = wires.iter().cloned().flat_map(|wire| unique(&wire)).collect();
    duplicates(&wires_without_dupes).iter().filter_map(|p| {
        let lengths : Vec<u32> = wires.iter()
            .filter_map(|wire| wire.iter().position(|other|*other == *p).map(|u| u as u32))
            .collect();
        if lengths.len() == wires.len() {Some(lengths.iter().sum()) } else{None} // ignore where the point is tripping over itself
    }).min().unwrap()
}

fn path(wire: &Vec<String>) -> Vec<Point> {
    let mut csr = Point::center();
    let mut res :Vec<Point> = wire.iter().flat_map(|direction| {
        let inner = csr.in_direction(direction);
        csr = *inner.last().unwrap();
        inner
    }).collect();
    res.insert(0, csr);
    res
}

fn unique(v: &Vec<Point>) -> Vec<Point> {
    let mut res = v.to_vec();
    res.sort();
    res.dedup();
    res
}

fn duplicates(v: &Vec<Point>) -> Vec<Point> {
    let mut unique = HashSet::new();
    unique.reserve(v.len()); // this halves the runtime!
    v.iter().cloned()
        .filter(|x| !unique.insert(*x))
        .collect()
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq, PartialOrd, Ord)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn center() -> Point{
        Point {x: 0,y: 0}
    }
    fn in_direction(&self, other: &str) -> Vec<Point> {
        let dir = other.chars().next().unwrap();
        let amount  = other[1..].parse().unwrap();
        (1..=amount).map(|n|
            match dir {
                'U' => Point {y: self.y + n, ..*self},
                'D' => Point {y: self.y - n, ..*self},
                'L' => Point {x: self.x - n, ..*self},
                'R' => Point {x: self.x + n, ..*self},
                _ => panic!("No valid direction given")
            }
        ).collect()
    }
    fn distance(&self, other: &Point) -> u32{
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as u32
    }
    fn distance_to_center(&self) -> u32{
        self.distance(&Point::center())
    }
}

fn parse_wire(s: &str) -> Vec<String> {
    s.clone().split(',').into_iter().map(|x| String::from(x)).collect()
}

#[cfg(test)]
mod test {
    use super::*;


    #[test]
    fn regression() {
        let nums: Vec<Vec<String>> = read_lines(3).into_iter().map(|l| parse_wire(&l)).collect();
        assert_eq!(pt1(&nums), 375);
        assert_eq!(pt2(&nums), 14746);
    }

    #[test]
    fn test_pt1() {
        assert_eq!(pt1(&vec![parse_wire("U3,R3,D1,L4")]), 0); // wire tripping over itself is not valid
        assert_eq!(pt1(&vec![parse_wire("R8,U5,L5,D3"),parse_wire("U7,R6,D4,L4")]), 6);
        assert_eq!(pt1(&vec![parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")]), 135);
        assert_eq!(pt1(&vec![parse_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72"),parse_wire("U62,R66,U55,R34,D71,R55,D58,R83")]), 159);
    }
    #[test]
    fn test_pt2() {
        assert_eq!(pt2(&vec![parse_wire("R8,U5,L5,D3"),parse_wire("U7,R6,D4,L4")]), 30);
        assert_eq!(pt2(&vec![parse_wire("R75,D30,R83,U83,L12,D49,R71,U7,L72"),parse_wire("U62,R66,U55,R34,D71,R55,D58,R83")]), 610);
        assert_eq!(pt2(&vec![parse_wire("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51"),parse_wire("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")]), 410);
    }
    #[test]
    fn test_distance() {
        assert_eq!(Point::center().distance(&Point{x:-2,y:-3}), 5);
        assert_eq!(Point::center().distance(&Point{x:2,y:-3}), 5);
    }
    #[test]
    fn test_duplicates() {
        assert_eq!(duplicates(&vec![Point::center(),Point::center(), Point{x:1,y:1}]), vec![Point::center()]);
    }
    #[test]
    fn test_direction() {
        assert_eq!(Point::center().in_direction("U1"), vec![Point{x:0, y:1}]);
        assert_eq!(Point::center().in_direction("D1"), vec![Point{x:0, y:-1}]);
        assert_eq!(Point::center().in_direction("L1"), vec![Point{x:-1, y:0}]);
        assert_eq!(Point::center().in_direction("R1"), vec![Point{x:1, y:0}]);
        assert_eq!(Point::center().in_direction("R2"), vec![Point{x:1, y:0},Point{x:2, y:0}]);
        assert_eq!(Point::center().in_direction("U3"), vec![Point{x:0, y:1},Point{x:0, y:2},Point{x:0, y:3}]);
    }
}