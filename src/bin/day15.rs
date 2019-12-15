use std::collections::{HashMap, BinaryHeap, HashSet};
use aoc2019::read_lines;
use std::iter::successors;
use std::cmp::Ordering;
use std::ops::RangeInclusive;
use aoc2019::intcode::IntCode;
use std::time::{Duration, Instant};
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

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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
    fn around(&self) -> Vec<Coord> {
        vec![self.shift(1),self.shift(2),self.shift(3),self.shift(4)]
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
    let mut heap = BinaryHeap::new();
    let mut wall : HashSet<Coord> = HashSet::new(); //TODO i dont think this is a performance increase...

    // We're at `start`, with a zero cost
    dist.insert(start.clone(), 0);
    heap.push(State { cost: 0, position: start.clone(), instr: vec![] });

    let mut i = 0;
    let mut robot = Robot::new(&initial_position, &mem);
    //TODO maybe just look at the last pushed elements (stack.pop()) without binaryHeap
    let mut oxy_system : Option<Vec<i64>> = None;
    while let Some(State { cost, position, instr }) = heap.pop() {
        // Ignore, there is a better way
        if cost > *dist.get(&position).unwrap_or(&max) { continue; }

        let robot_res = robot.run(&instr);
        let res = robot_res;
        if res == 0 {
            dist.remove(&position); // forget about walls!
            continue; // hit a wall, ignore it
        } else if res == 1 {
            // all good!
        } else if res == 2 {
            oxy_system = Some(instr.clone());// found the oxygen system!
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for dir in 1_i64..=4_i64 {
            let next_coord = position.shift(dir);
            let next = State { cost: cost + 1, position: next_coord, instr: [instr.clone(), vec![dir]].concat().to_vec()};

            if next.cost < *dist.get(&next.position).unwrap_or(&max) {
                heap.push(next.clone());
                // faster path found
                dist.insert(next.position.clone(), next.cost);
            }
        }
    }
    let furthest_distance = dist.values().max().unwrap();
    (oxy_system, *furthest_distance)
}


struct Robot {
    instr: Vec<i64>,
    send: Sender<i64>,
    recv: Receiver<i64>
}
impl Robot { // utility to reuse IntCode and not rerun every time...
    fn new(initial_location: &Vec<i64>, mem: &Vec<i64>) -> Robot {
        let (send,recv) = IntCode::run_async(&mem);
        let mut robot = Robot { send, recv, instr: vec![]};
        robot.run(initial_location);
        robot.instr = vec![];
        robot
    }
    fn run(&mut self, new_instr: &Vec<i64>) -> i64 {
        if new_instr.len() == 0 {
            return 1; // all good
        }
        // maybe we don't have to go all the way, lets see how far we already are!
        let shared_path : Vec<i64> = new_instr.iter().zip(self.instr.iter()).take_while(|(a,b)| a == b).map(|(a,_)|*a).collect();
        let to_backtrace = Robot::reverse_instr(self.instr[shared_path.len()..].to_vec());
        for back in to_backtrace.iter() {
            self.send.send(*back);
        }
        let new_subpath = new_instr[shared_path.len()..].to_vec();
        for new in new_subpath.iter() {
            self.send.send(*new);
        }
        self.instr = new_instr.clone();
        let res = self.recv.iter().take(to_backtrace.len() + new_subpath.len()).last().unwrap();
        if res == 0 {
            self.instr = new_instr[..new_instr.len()-1].to_vec();
        }
        res
    }
    fn reverse_instr(mut to_reverse : Vec<i64>) -> Vec<i64> {
        to_reverse.reverse();
        to_reverse.iter().map(|dir| {
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

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
struct Chem {amount: i64, name: String}

impl Chem {
    fn new(amount: i64, name: &str) -> Chem { Chem {amount, name: name.to_string()} }
}

#[cfg(test)]
mod test {
    use super::*;

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
    fn test_find() {
        let mem: Vec<i64> = read_lines(15)[0].split(',').map(|s| s.parse().unwrap()).collect();
        assert_eq!(pt1(&mem), 336);
    }
    #[test]
    fn test_slice() {
        assert_eq!(vec![1,1][1..], [1]);
    }
}