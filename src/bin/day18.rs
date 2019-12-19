use aoc2019::read_lines;
use std::collections::{HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::slice::SliceIndex;
use std::char::ToLowercase;
use std::time::Instant;

fn main() {
    //let mem: Vec<i64> = read_lines(15)[0].split(',').map(|s| s.parse().unwrap()).collect();
    //println!("pt1: {}", pt1(&mem)); // 336
    //println!("pt2: {}", pt2(&mem)); // 360

    let mut ex =
        "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

    let mut maze = ex.split('\n').map(String::from).collect();
    let start = Instant::now();
    let res = shortest_path_all_keys(&maze);
    println!("found {} after {:?}", res, start.elapsed());
    assert_eq!(res, 132);
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    steps: usize,
    position: Coord,
//    collected_keys: Vec<char>
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.steps.cmp(&self.steps)
            .then_with(|| self.position.cmp(&other.position))
//        other.collected_keys.len().cmp(&self.collected_keys.len())
//            .then_with(|| other.steps.cmp(&self.steps))
//            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}


#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize
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
        [self.shift(1), self.shift(2), self.shift(3), self.shift(4)].to_vec()
    }
}

fn at_coord(coord: &Coord, maze: &Vec<String>) -> char{
    maze[coord.y].chars().nth(coord.x).unwrap()
}

fn walkables(maze: &Vec<String>) -> HashMap<char, Coord> {
    (0..maze.len())
        .flat_map(|y|
            (0..maze[y].len())
                .filter_map(|x| {
                    let coord = Coord{x,y};
                    let c = at_coord(&coord, &maze);
                    if c.is_lowercase() || c.is_uppercase() || c == '@' { Some((c, coord))} else { None }
                }).collect::<Vec<(char, Coord)>>()
        )
        .collect()
}

fn keys(maze: &Vec<String>) -> HashMap<char, Coord> {
    walkables(&maze).into_iter().filter(|(ch, coord)| ch.is_lowercase()).collect()
}

fn path_to(start: &Coord, goal: &Coord, maze : &Vec<String>) -> Vec<Coord> {
    let max : usize = 999999999999999999;
    // build up distances in different subsets, based on what key is already collected
    let mut dist : HashMap<Coord, usize> = HashMap::new();
    let mut prev : HashMap<Coord, Coord> = HashMap::new();
    let mut heap : BinaryHeap<State> = BinaryHeap::new();


    dist.insert(start.clone(), 0);
    heap.push(State { steps: 0, position: start.clone() /*, collected_keys: vec![] */ });

    let mut last = Some(0);
    //println!("before loop");
    while let Some(State { steps: steps, position /*, collected_keys */ }) = heap.pop() {
        let char_at = at_coord(&position, &maze);
        if char_at == '#' {
            prev.remove(&position);
            continue;
        }
        // Skip if whe have a better way with the same key combination
        if steps > *dist.get(&position).unwrap_or(&max) { continue; }

        if position == *goal {
            println!("found goal!");
            break; }
        // Check all neighbors from the current cursor
        for next_position in position.around() {
            let next = State { steps: steps + 1, position: next_position.clone()/*, collected_keys: collected_keys.clone()*/};
            if next.steps < *dist.get(&next_position).unwrap_or(&max) {
                println!("updating position prev {:?} {:?}", next_position, position);
                dist.insert(next_position.clone(), next.steps);
                prev.insert(next_position.clone(), position.clone());
                heap.push(next);
            }
        }
    }

    let mut path : Vec<Coord> = vec![];
    path.push(goal.clone());
    let mut csr = goal.clone();
    let mut i = 0;
    while let Some(path_part) = prev.get(&csr) {
        csr = path_part.clone();
        println!("{:?}", csr);
        path.push(path_part.clone());

        if i == 20 {
            break;
        }
        i += 1;
    }
    path.reverse();

    path
}

fn shortest_path_all_keys(maze: &Vec<String>) -> usize {/*
    let mut keys = keys(&maze);
    let start = keys.remove(&'@').unwrap();
    println!("{:?}", start);
    println!("{:?}", keys);

    let max : usize = 999999999999999999;
    // build up distances in different subsets, based on what key is already collected
    let mut dist : HashMap<Coord, usize> = HashMap::new();
    let mut prev : HashMap<Coord, Coord> = HashMap::new();
    let mut heap : BinaryHeap<State> = BinaryHeap::new();

    heap.push(State { steps: 0, position: start.clone(), collected_keys: vec![] });

    let mut last = Some(0);
    //println!("before loop");
    while let Some(State { steps: steps, position, collected_keys: prev_collected_key }) = heap.pop() {
        //println!("looking at {:?}, {:?}, {:?}", position, prev_collected_key, steps);

        let mut collected_keys = prev_collected_key;
        let char_at = at_coord(&position, &maze);
        if char_at == '#' {
            continue;
        }
        // Skip if whe have a better way with the same key combination
        if steps > dist[&position] { continue; }

        // Check all neighbors from the current cursor
        for next_position in position.around() {
            let next = State { steps: steps + 1, position: next_position.clone(), collected_keys: collected_keys.clone()};
            if next.steps < *dist.get(&next_position).unwrap_or(&max) {
                dist.insert(next_position.clone(), next.steps);
                prev.insert(next_position.clone(), position.clone());
                heap.push(next);
            }
        }
    }

    last.unwrap()*/
    0
}

#[cfg(test)]
mod test {
    use super::*;
    /*
    #[test]
    fn regression() {
        let mem: Vec<i64> = read_lines(15)[0].split(',').map(|s| s.parse().unwrap()).collect();
        assert_eq!(pt1(&mem), 336);
        assert_eq!(pt2(&mem), 360);
    }*/

    #[test]
    fn test_something() {
        let mut ex =
"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";

        let mut maze = ex.split('\n').map(String::from).collect();
        let start = walkables(&maze)[&'@'].clone();
        let end = walkables(&maze)[&'f'].clone();
        //println!("{:?}", );
        assert_eq!(path_to(&start, &end, &maze), vec![]);
    }

    #[test]
    fn test() {
        let mut ex =
"########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";

        let mut maze = ex.split('\n').map(String::from).collect();
        assert_eq!(shortest_path_all_keys(&maze),86);

        ex =
"#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

        maze = ex.split('\n').map(String::from).collect();
        assert_eq!(shortest_path_all_keys(&maze),132);

    }

    #[test]
    fn test_eq_vec() {
        let mut hm : HashMap<Vec<char>, usize> = HashMap::new();
        hm.insert(vec!['a','b'],0);

        println!("{}", hm[&vec!['a','b']]);
        assert_eq!(vec![1,2], vec![1,2]);
        assert_ne!(vec![1,2], vec![2,1]);
    }
}