use std::collections::{HashMap, BinaryHeap};
use aoc2019::read_lines;
use std::iter::successors;
use std::cmp::Ordering;
use std::ops::RangeInclusive;
use aoc2019::intcode::IntCode;

fn main() {
    let mem: Vec<i64> = read_lines(15)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", shortest_path(&mem).len()); // 336
    //println!("pt2: {}", pt2(&reactions)); //
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

fn shortest_path(mem: &Vec<i64>) -> Vec<i64>{
    let start = Coord{x:0, y:0};
    let max : usize = 999999999999999999;
    let mut dist: HashMap<Coord, usize> = HashMap::new();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist.insert(start.clone(), 0);
    heap.push(State { cost: 0, position: start.clone(), instr: vec![] });

    let mut i = 0;
    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position, instr }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        //if position == goal { return Some(cost); }
        // TODO short circuit when wall is found, dont proceed

        //println!("running instr: {:?}", instr);
        let instr_res = run_instr(&instr, &mem);
        if instr_res == 0 {
            println!("ignoring wall");
            continue; // hit a wall, ignore it
        } else if instr_res == 1 {
            // all good!
        } else if instr_res == 2 {
            // winner! found the oxygen system!
            return instr.clone();
        }

        // Ignore, there is a better way
        if cost > *dist.get(&position).unwrap_or(&max) { continue; }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for dir in 1_i64..=4_i64 {
            let next = State { cost: cost + 1, position: position.shift(dir), instr: [instr.clone(), vec![dir]].concat().to_vec()};

            if next.cost < *dist.get(&next.position).unwrap_or(&max) {
                heap.push(next.clone());
                // faster path found
                dist.insert(next.position.clone(), next.cost);
            }
        }
        i +=1;
        if i == 1000 {
            break;
        }
    }
    panic!("Unable to find shortest path!")
}


fn run_instr(v: &Vec<i64>, pgm: &Vec<i64>) -> i64 {
    if v.len() == 0 {
        return 1; // all good, initial spot can be reached!
    }
    let (send_in, recv_out) = IntCode::run_async(&pgm);
    for instr in v {
        send_in.send(*instr);
    }
    for (i, out) in recv_out.iter().enumerate() {
        if i == v.len() -1 {
            //println!("{:?} => {}", v, out);
            return out
        } else if out == 0 {
            panic!("dude theres a wall in your way!");
        }
    }
    panic!("dude he just hang up man!");
    -1
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
        assert_eq!(run_instr(&vec![1], &mem), 1);
        assert_eq!(run_instr(&vec![2], &mem), 0);
        assert_eq!(run_instr(&vec![3], &mem), 0);
        assert_eq!(run_instr(&vec![4], &mem), 0);
        assert_eq!(run_instr(&vec![1,1,1], &mem), 0);
    }
    #[test]
    fn test_find() {
        let mem: Vec<i64> = read_lines(15)[0].split(',').map(|s| s.parse().unwrap()).collect();
        assert_eq!(shortest_path(&mem).len(), 0);
    }
}