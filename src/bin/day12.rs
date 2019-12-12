use std::collections::{HashMap, HashSet};

use aoc2019::intcode::IntCode;
use aoc2019::read_lines;
use std::hash::Hash;
use std::cmp::Ordering;
use std::cell::RefCell;
use std::borrow::{Borrow, BorrowMut};

fn main() {
    let mem: Vec<Planet> = parse(&read_lines(12));
    println!("pt1: {}", pt1(&mem,1000)); // 6423
    //println!("pt1: {}", pt2(&mem,100)); // 6423
    //println!("pt2: ", ); //
}

fn pt1(planets: &Vec<Planet>, steps: isize) -> isize{
    let mut  p_mut = planets.to_vec();
    for _ in 0..steps {
        step(&mut p_mut)
    }
    p_mut.iter().map(Planet::energy).sum::<isize>()
}

fn pt2(planets: &Vec<Planet>) -> isize{
    let mut p_mut = planets.to_vec();
    let mut cycle_time_x : Vec<isize> = vec![0; p_mut.len()];
    let mut cycle_time_y : Vec<isize> = vec![0; p_mut.len()];
    let mut cycle_time_z : Vec<isize> = vec![0; p_mut.len()];
    let mut cycle_times : Vec<Vec<isize>> = vec![vec![0,0,0]; p_mut.len()];
    let mut cycle_cnt = 1;
    while cycle_times.iter().flatten().any(|e| *e == 0) {
    //while cycle_time_x.contains(&0) || cycle_time_y.contains(&0) || cycle_time_z.contains(&0) {
        step(&mut p_mut);

        for(i,(original, modified)) in planets.iter().zip(p_mut.iter()).enumerate() {
            if original.pos.x == modified.pos.x && original.vel.x == modified.vel.x && cycle_times[i][0] == 0 {
                cycle_times[i][0] = cycle_cnt;
                //cycle_time_x[i] = cycle_cnt;
                println!("[{}] returned initial x state after {}", i, cycle_cnt);
            }
            if original.pos.y == modified.pos.y && original.vel.y == modified.vel.y && cycle_times[i][1] == 0 {
                cycle_times[i][1] = cycle_cnt;
                //cycle_time_y[i] = cycle_cnt;
                println!("[{}] returned initial y state after {}", i, cycle_cnt);
            }
            if original.pos.z == modified.pos.z && original.vel.z == modified.vel.z && cycle_times[i][2] == 0{
                cycle_times[i][2] = cycle_cnt;
                //cycle_time_z[i] = cycle_cnt;
                println!("[{}] returned initial z state after {}", i, cycle_cnt);
            }
        }
        cycle_cnt += 1;

        if cycle_cnt % 1000 == 0{
            //println!("{:?}", [cycle_time_z.clone(), cycle_time_y.clone(), cycle_time_x.clone()].concat());
            //break;
        }
        if cycle_cnt == 100000 {
            println!("halting after 100000 iterations");
            break;
        }
    }
/*

    let x_repeat = lcm_vec(&cycle_time_x);
    let y_repeat = lcm_vec(&cycle_time_y);
    let z_repeat = lcm_vec(&cycle_time_z);
    let res = lcm_vec(&vec![x_repeat,y_repeat,z_repeat]);

    let possible_cycle_times: Vec<isize> = [cycle_time_x, cycle_time_y, cycle_time_z].concat();
*/

    let mut possible_cycle_times : HashSet<isize> = cycle_times.iter().clone().flatten().map(|n|*n).collect();
    possible_cycle_times.insert(36);
    possible_cycle_times.insert(12);
    println!("found all cycle values {:?};", possible_cycle_times);
    let res = lcm_vec(&possible_cycle_times.iter().cloned().collect::<Vec<isize>>());


    println!("found all cycle values {:?}; lcd : {}", possible_cycle_times, res);
    p_mut.iter().map(Planet::energy).sum::<isize>()
}

fn step(planets: &mut Vec<Planet>) {
    apply_gravity(planets);
    apply_velocity(planets);
}

fn apply_gravity(planets : &mut Vec<Planet>) {
    for i in 1..planets.len() {
        let (left, right) = planets.split_at_mut(i);
        let p1 = left.last_mut().unwrap();
        for p2 in right {
            p1.mod_gravity_from(p2);
            p2.mod_gravity_from(p1);
        }
    }
}

fn apply_velocity(planets : &mut Vec<Planet>) {
    for p in planets {
        p.apply_velocity()
    }
}

fn parse(input: &Vec<String>) -> Vec<Planet> {
    input.iter()
        .map(|line| Planet::new(&Coord::new(line), &Coord::at(0,0,0)))
        .collect()
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize,
    z: isize
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
struct Planet {
    pos: Coord,
    vel: Coord
}

impl Coord {
    fn new(s: &String) -> Coord {
        let parsed : Vec<isize> = s.split(['=',',','>'].as_ref())
            .map(|s| s.trim())
            .filter_map(|part| part.parse().ok())
            .collect();
        //println!("parsed {:?} from {}", parsed, s);
        Coord::at(parsed[0],parsed[1],parsed[2])
    }
    fn at(x: isize, y: isize, z: isize) -> Coord {
        Coord{x,y,z}
    }
    fn energy(&self) -> isize {
        vec![self.x,self.y,self.z].iter().map(|n|n.abs()).sum()
    }
}

impl Planet {
    fn from_str(s: &String) -> Planet{
        let coords : Vec<Coord> = s.replace("pos=","S").replace(", vel=", "S")
            .split('S')
            .filter(|s| s.trim().len() > 0)
            .map(|c| Coord::new(&String::from(c)))
            .collect();
        Planet {pos: coords[0].clone(), vel: coords[1].clone()}
    }
    fn new(pos: &Coord, vel: &Coord) -> Planet {
        Planet{pos: pos.clone(), vel: vel.clone()}
    }
    fn mod_gravity_from(&mut self, other: &Planet) {
        self.vel.x += gravity(self.pos.x, other.pos.x);
        self.vel.y += gravity(self.pos.y, other.pos.y);
        self.vel.z += gravity(self.pos.z, other.pos.z);
    }
    fn apply_velocity(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }
    fn energy(&self) -> isize {
        self.pos.energy() * self.vel.energy()
    }
}

fn gravity(from: isize, to: isize) -> isize {
    match from.cmp(&to) {
        Ordering::Equal => 0,
        Ordering::Greater => -1,
        Ordering::Less => 1
    }
}

fn gcd(mut m: isize, mut n: isize) -> isize {
    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
    }
    n.abs()
}

fn lcm(a: isize, b: isize) -> isize {
    let res = a * b / gcd(a, b);

    println!("lcm {} {} => {}",a,b , res);
    res
}

fn lcm_vec(v: &Vec<isize>) -> isize {
    v.iter().fold(-1,|acc, n| {
        if acc == -1 {
            *n
        } else {
            lcm(*n, acc)
        }
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_coord() {
        assert_eq!(Coord::new(&String::from("<x=14, y=4, z=5>")), Coord::at(14,4,5));
        assert_eq!(Coord::new(&String::from("<x=1, y=-2, z=-3>")), Coord::at(1,-2,-3));
    }

    #[test]
    fn test_gravity() {
        let ex =
            "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        let mut  planets: Vec<Planet> = parse(&ex.split('\n').map(String::from).collect());
        let mut expected : Vec<Planet>= vec![
            "pos=<x= 2, y=-1, z= 1>, vel=<x=3, y=-1, z=-1>",
            "pos=<x= 3, y=-7, z=-4>, vel=<x=1, y=3, z= 3>",
            "pos=<x= 1, y=-7, z= 5>, vel=<x=-3, y= 1, z=-3>",
            "pos=<x= 2, y= 2, z= 0>, vel=<x=-1, y=-3, z= 1>"
        ].iter().map(|s| Planet::from_str(&String::from(*s))).collect();
        step(&mut planets);
        assert_eq!(planets, expected);

        expected = vec![
            "pos=<x= 5, y=-3, z=-1>, vel=<x= 3, y=-2, z=-2>",
            "pos=<x= 1, y=-2, z= 2>, vel=<x=-2, y= 5, z= 6>",
            "pos=<x= 1, y=-4, z=-1>, vel=<x= 0, y= 3, z=-6>",
            "pos=<x= 1, y=-4, z= 2>, vel=<x=-1, y=-6, z= 2>"
        ].iter().map(|s| Planet::from_str(&String::from(*s))).collect();
        step(&mut planets);
        assert_eq!(planets, expected);

        expected = vec![
            "pos=<x= 5, y=-6, z=-1>, vel=<x= 0, y=-3, z= 0>",
            "pos=<x= 0, y= 0, z= 6>, vel=<x=-1, y= 2, z= 4>",
            "pos=<x= 2, y= 1, z=-5>, vel=<x= 1, y= 5, z=-4>",
            "pos=<x= 1, y=-8, z= 2>, vel=<x= 0, y=-4, z= 0>"
        ].iter().map(|s| Planet::from_str(&String::from(*s))).collect();
        step(&mut planets); // 3rd step
        assert_eq!(planets, expected);

        step(&mut planets);
        step(&mut planets);
        step(&mut planets);
        step(&mut planets);
        step(&mut planets);
        step(&mut planets); // 9th step

        expected = vec![
            "pos=<x= 2, y= 1, z=-3>, vel=<x=-3, y=-2, z= 1>",
            "pos=<x= 1, y=-8, z= 0>, vel=<x=-1, y= 1, z= 3>",
            "pos=<x= 3, y=-6, z= 1>, vel=<x= 3, y= 2, z=-3>",
            "pos=<x= 2, y= 0, z= 4>, vel=<x= 1, y=-1, z=-1>"
        ].iter().map(|s| Planet::from_str(&String::from(*s))).collect();
        step(&mut planets); // 10th step
        assert_eq!(planets, expected);
        assert_eq!(planets.iter().map(|p|p.energy()).sum::<isize>(), 179);
    }
    #[test]
    fn test_pt1() {
        let ex =
            "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        let mut planets: Vec<Planet> = parse(&ex.split('\n').map(String::from).collect());
        assert_eq!(pt1(&planets,10), 179);
    }

    #[test]
    fn test_pt2() {
        let mut ex =
            "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        let mut planets: Vec<Planet> = parse(&ex.split('\n').map(String::from).collect());
        pt2(&planets);


        ex =
"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

        planets = parse(&ex.split('\n').map(String::from).collect());
        pt2(&planets);

    }
}