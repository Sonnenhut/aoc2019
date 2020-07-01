use std::collections::HashMap;
use aoc2019::read_lines;
use aoc2019::intcode::{IntCode, IntCodeClient};
use std::sync::mpsc::{Sender, Receiver};

fn main() {
    let mem: Vec<i64> = read_lines(15)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", pt1(&mem)); // 336
    println!("pt2: {}", pt2(&mem)); // 360
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Coord,
    instr: Vec<i64> // instructions to get there from 0
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
struct Coord {
    x: isize,
    y: isize
}

impl Coord {
    fn shift(&self, d: i64) -> Coord {
        match d {
            1 => Coord{x:self.x+1, y: self.y},
            2 => Coord{x:self.x-1, y: self.y},
            3 => Coord{x:self.x, y: self.y-1},
            4 => Coord{x:self.x, y: self.y+1},
            _ => panic!("cannot go in given direction (direction unknown)")
        }
    }
}
fn pt1(mem: &Vec<i64>) -> usize {
    shortest_path(&vec![], &mem).0.unwrap().len()
}

fn pt2(mem: &Vec<i64>) -> usize {
    let to_oxy = shortest_path(&vec![], &mem).0.unwrap();
    shortest_path(&to_oxy, &mem).1
}

fn shortest_path(initial_position: &Vec<i64>, mem: &Vec<i64>) -> (Option<Vec<i64>>,usize) {
    let start = Coord{x:0, y:0};
    let max : usize = 999999999999999999;
    let mut dist: HashMap<Coord, usize> = HashMap::new();
    let mut stack = vec![];

    dist.insert(start.clone(), 0);
    stack.push(State { cost: 0, position: start.clone(), instr: vec![] });

    let mut robot = Robot::new(&initial_position, &mem);
    let mut oxy_system : Option<Vec<i64>> = None;

    while let Some(State { cost, position, instr }) = stack.pop() {
        // Already got a better way, ignore
        if cost > *dist.get(&position).unwrap_or(&max) { continue; }

        let robot_res = robot.run(&instr);
        let res = robot_res;
        if res == 0 {
            dist.remove(&position); // forget distances to walls
            continue;
        } else if res == 2 {
            oxy_system = Some(instr.clone());// found the oxygen system!
        }

        // Check all neighbors from the current cursor
        for dir in 1..=4 {
            let next = State { cost: cost + 1, position:  position.shift(dir), instr: [instr.clone(), vec![dir]].concat().to_vec()};

            if next.cost < *dist.get(&next.position).unwrap_or(&max) {
                dist.insert(next.position.clone(), next.cost);
                stack.push(next);
            }
        }
    }
    (oxy_system, *dist.values().max().unwrap())
}

struct Robot {
    instr: Vec<i64>,
    send: Sender<i64>,
    recv: Receiver<i64>
}

impl Robot { // utility to reuse IntCode and not rerun every time...
    fn new(initial_location: &Vec<i64>, mem: &Vec<i64>) -> Robot {
        let IntCodeClient {snd, rcv, idle: _} = IntCode::run_async(&mem);
        let mut robot = Robot { send: snd, recv: rcv, instr: vec![]};
        robot.run(initial_location);
        robot.instr = vec![];
        robot
    }
    fn run(&mut self, new_instr: &Vec<i64>) -> i64 {
        if new_instr.len() == 0 {
            return 1; // all good
        }
        // maybe we don't have to go all the way, lets see how far we already are!
        let prefix_shared  = new_instr.iter().zip(self.instr.iter()).take_while(|(a,b)| a == b).map(|(a,_)|*a).count();
        let to_backtrace = Robot::reverse_instr(&self.instr[prefix_shared..].to_vec());
        let new_subpath = new_instr[prefix_shared..].to_vec();
        let diff = [to_backtrace, new_subpath].concat();
        for d in diff.iter() {
            let _ = self.send.send(*d);
        }
        let res = self.recv.iter().take(diff.len()).last().unwrap();
        self.instr = if res == 0 { // was a wall, don't store that as our current location
             new_instr[..new_instr.len()-1].to_vec()
        } else {
            new_instr.clone()
        };
        res
    }
    fn reverse_instr(to_reverse : &Vec<i64>) -> Vec<i64> {
        to_reverse.iter().rev().map(|dir| {
            match dir {
                1 => 2,
                2 => 1,
                3 => 4,
                4 => 3,
                _ => panic ! ("unknown direction to inverse")
            }
        }).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn regression() {
        let mem: Vec<i64> = read_lines(15)[0].split(',').map(|s| s.parse().unwrap()).collect();
        assert_eq!(pt1(&mem), 336);
        assert_eq!(pt2(&mem), 360);
    }

    #[test]
    fn test_run_instr() {
        let mem: Vec<i64> = read_lines(15)[0].split(',').map(|s| s.parse().unwrap()).collect();
        let mut robot = Robot::new(&vec![],&mem);
        assert_eq!(robot.run(&vec![1]), 1);
        assert_eq!(robot.run(&vec![2]), 0);
        assert_eq!(robot.run(&vec![3]), 0);
        assert_eq!(robot.run(&vec![4]), 0);
        assert_eq!(robot.run(&vec![1,1,1]), 0);
    }
    #[test]
    fn test_slice() {
        assert_eq!(vec![1,1][1..], [1]);
    }
}
