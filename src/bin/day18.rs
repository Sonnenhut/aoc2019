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

#[derive(Clone, Eq, PartialEq)]
struct State {
    steps: usize,
    position: Coord
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.steps.cmp(&self.steps)
            .then_with(|| other.position.collected_keys.len().cmp(&self.position.collected_keys.len()))
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
    x: usize,
    y: usize,
    collected_keys: Vec<char>
}

impl Coord {
    fn shift(&self, d: i64) -> Coord {
        match d {
            1 => Coord{x:self.x+1, y: self.y, collected_keys: self.collected_keys.clone()},
            2 => Coord{x:self.x-1, y: self.y, collected_keys: self.collected_keys.clone()},
            3 => Coord{x:self.x, y: self.y-1, collected_keys: self.collected_keys.clone()},
            4 => Coord{x:self.x, y: self.y+1, collected_keys: self.collected_keys.clone()},
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
                    let coord = Coord{x,y, collected_keys: vec![]};
                    let c = at_coord(&coord, &maze);
                    if c.is_lowercase() || c.is_uppercase() || c == '@' { Some((c, coord))} else { None }
                }).collect::<Vec<(char, Coord)>>()
        )
        .collect()
}

fn keys(maze: &Vec<String>) -> HashMap<char, Coord> {
    walkables(&maze).into_iter().filter(|(ch, coord)| ch.is_lowercase()).collect()
}

fn distance(start: &Coord, goal: &Coord, maze: &Vec<String>) -> usize {
    let mut keys = keys(&maze);
    let start = walkables(maze).remove(&'@').unwrap();
    println!("{:?}", start);
    println!("{:?}", keys);

    let max : usize = 999999999999999999;
    // build up distances in different subsets, based on what key is already collected
    let mut dist : HashMap<Coord, usize> = HashMap::new();
    let mut prev : HashMap<Coord, Coord> = HashMap::new();
    let mut heap : BinaryHeap<State> = BinaryHeap::new();

    heap.push(State { steps: 0, position: start.clone()});
    dist.insert(start.clone(), 0);

    let mut res = 0;
    while let Some(State { steps: steps, position: initial_position}) = heap.pop() {
        let mut position = initial_position.clone();
        //println!("looking at: {:?}", position);

        // Skip if whe have a better way with the same key combination
        if steps > *dist.get(&position).unwrap_or(&max) { continue; }

        if position.collected_keys.len() == keys.len() {
            res = dist[&initial_position];
            break;
        }

        // Check all neighbors from the current cursor
        for neighbour_coord in position.around() {
            let char_at = at_coord(&neighbour_coord, &maze);
            if char_at == '#' {
                continue;
            }
            let neighbour = if char_at.is_lowercase() && !neighbour_coord.collected_keys.contains(&char_at) {
                Coord {collected_keys: [neighbour_coord.collected_keys, vec![char_at]].concat(), ..neighbour_coord}
            } else { neighbour_coord };

            if char_at.is_uppercase() && !neighbour.collected_keys.contains(&char_at.to_lowercase().nth(0).unwrap()){
                continue; // cannot pass through door yet
            }

            let mut next = State { steps: steps + 1, position: neighbour.clone()};
            if next.steps < *dist.get(&neighbour).unwrap_or(&max) {
                dist.insert(neighbour.clone(), next.steps);
                prev.insert(neighbour.clone(), position.clone());
                heap.push(next);
            }
        }
    }

    res
}


fn shortest_path_all_keys(maze: &Vec<String>) -> usize {
    let mut keys = keys(&maze);
    let start = walkables(maze).remove(&'@').unwrap();
    println!("{:?}", start);
    println!("{:?}", keys);

    let max : usize = 999999999999999999;
    // build up distances in different subsets, based on what key is already collected
    let mut dist : HashMap<Coord, usize> = HashMap::new();
    let mut prev : HashMap<Coord, Coord> = HashMap::new();
    let mut heap : BinaryHeap<State> = BinaryHeap::new();

    heap.push(State { steps: 0, position: start.clone()});
    dist.insert(start.clone(), 0);

    let mut res = 0;
    while let Some(State { steps: steps, position: initial_position}) = heap.pop() {
        let mut position = initial_position.clone();
        //println!("looking at: {:?}", position);

        // Skip if whe have a better way with the same key combination
        if steps > *dist.get(&position).unwrap_or(&max) { continue; }

        if position.collected_keys.len() == keys.len() {
            res = dist[&initial_position];
            break;
        }

        // Check all neighbors from the current cursor
        for neighbour_coord in position.around() {
            let char_at = at_coord(&neighbour_coord, &maze);
            if char_at == '#' {
                continue;
            }
            let neighbour = if char_at.is_lowercase() && !neighbour_coord.collected_keys.contains(&char_at) {
                Coord {collected_keys: [neighbour_coord.collected_keys, vec![char_at]].concat(), ..neighbour_coord}
            } else { neighbour_coord };

            if char_at.is_uppercase() && !neighbour.collected_keys.contains(&char_at.to_lowercase().nth(0).unwrap()){
                continue; // cannot pass through door yet
            }

            let mut next = State { steps: steps + 1, position: neighbour.clone()};
            if next.steps < *dist.get(&neighbour).unwrap_or(&max) {
                dist.insert(neighbour.clone(), next.steps);
                prev.insert(neighbour.clone(), position.clone());
                heap.push(next);
            }
        }
    }

    res
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