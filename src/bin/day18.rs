use aoc2019::read_lines;
use std::collections::{HashMap, BinaryHeap, HashSet};
use std::cmp::Ordering;
use std::slice::SliceIndex;
use std::char::ToLowercase;
use std::time::Instant;
use std::iter::FromIterator;

fn main() {
    let maze1: Vec<String> = read_lines(18).iter().map(String::from).collect();
    println!("pt1: {}", solve(&maze1)); // 2684

    let maze2: Vec<String> = transform_pt2_maze(&maze1);
    println!("pt2: {}", solve(&maze2)); // 1886
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    steps: usize,
    position: Coord
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.steps.cmp(&self.steps)
            .then_with(|| *&self.position.collected_keys.len().cmp(&other.position.collected_keys.len()))
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

fn entries(maze: &Vec<String>) -> Vec<Coord> {
    (0..maze.len())
        .flat_map(|y|
            (0..maze[y].len())
                .filter_map(|x| {
                    let coord = Coord{x,y, collected_keys: vec![]};
                    let c = at_coord(&coord, &maze);
                    if c == '@' { Some(coord)} else { None }
                }).collect::<Vec<Coord>>()
        )
        .collect()
}

fn keys(maze: &Vec<String>) -> HashMap<char, Coord> {
    walkables(&maze).into_iter().filter(|(ch, coord)| ch.is_lowercase()).collect()
}


// parse coming from a location what are the (reached keys, reached doors)
fn parse(maze: &Vec<String>, coord: &Coord) -> (Vec<char>, Vec<char>) {
    let mut keys = keys(&maze);
    let start = coord.clone();
    println!("{:?}", start);
    println!("{:?}", keys);

    let max : usize = 999999999999999999;
    // build up distances in different subsets, based on what key is already collected
    let mut dist : HashMap<Coord, usize> = HashMap::new();
    let mut heap : BinaryHeap<State> = BinaryHeap::new();

    heap.push(State { steps: 0, position: start.clone()});
    dist.insert(start.clone(), 0);

    let mut reached_doors = vec![];
    let mut reached_keys = vec![];
    while let Some(State { steps: steps, position: initial_position}) = heap.pop() {
        let mut position = initial_position.clone();

        // Skip if whe have a shorter way
        if steps > *dist.get(&position).unwrap_or(&max) {
            dist.remove(&position);
            continue;
        }

        // Check all neighbors from the current cursor
        for neighbour in position.around() {
            let char_at = at_coord(&neighbour, &maze);
            if char_at == '#' {
                continue;
            }

            if char_at.is_lowercase() { reached_keys.push(char_at.clone()); }
            if char_at.is_uppercase() { reached_doors.push(char_at.clone()); }

            let mut next = State { steps: steps + 1, position: neighbour.clone()};
            if next.steps < *dist.get(&neighbour).unwrap_or(&max) {
                dist.insert(neighbour.clone(), next.steps);
                heap.push(next);
            }
        }
    }

    reached_doors.sort();
    reached_doors.dedup_by_key(|c| c.clone());
    reached_keys.sort();
    reached_keys.dedup_by_key(|c| c.clone());
    (reached_keys, reached_doors)
}

fn shortest_path(maze: &Vec<String>, start: &Coord, wanted_keys: &HashSet<char>) -> usize {
    println!("start {:?}", start);
    println!("wanted keys{:?}", wanted_keys);

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

        // Skip if whe have a better way with the same key combination
        if steps > *dist.get(&position).unwrap_or(&max) {
            dist.remove(&position);
            prev.remove(&position);
            continue;
        }

        if wanted_keys.iter().all(|k| position.collected_keys.contains(k)) {
            res = dist[&initial_position];
            println!("FOUND ALL KEYS YEAH!");
            break;
        }
        // Check all neighbors from the current cursor
        for neighbour_coord in position.around() {
            let char_at = at_coord(&neighbour_coord, &maze);
            if char_at == '#' {
                continue;
            }
            let neighbour = if char_at.is_lowercase() && !neighbour_coord.collected_keys.contains(&char_at) {
                let mut collected_keys = neighbour_coord.collected_keys.clone();
                collected_keys.push(char_at);
                collected_keys.sort();
                Coord {collected_keys, ..neighbour_coord}
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

// for pt2: assume that the optimal path for each quadrant if all the other quadrants were already solved
fn solve(maze: &Vec<String>) -> usize {
    let startpoints = entries(maze);

    let mut res = 0;
    for mut startpoint in startpoints {
        let (reachable_keys, reachable_doors) = parse(&maze, &startpoint);
        let unreachable_keys : HashSet<char> = reachable_doors.iter().map(|c| c.to_lowercase().nth(0).unwrap()).filter(|c| !reachable_keys.contains(c)).collect();

        startpoint.collected_keys = unreachable_keys.clone().into_iter().collect(); // assume the fastest way when all keys are collected
        res += shortest_path(&maze, &startpoint, &reachable_keys.clone().into_iter().collect());
    }
    res
}

fn transform_pt2_maze(maze: &Vec<String>) -> Vec<String> {
    let original_start = walkables(&maze).remove(&'@').unwrap();
    let walls = [original_start.around(), vec![original_start.clone()]].concat();
    let portals = [
        Coord{x : original_start.x - 1,y: original_start.y - 1, collected_keys: vec![]},
        Coord{x : original_start.x - 1,y: original_start.y + 1, collected_keys: vec![]},
        Coord{x : original_start.x + 1,y: original_start.y - 1, collected_keys: vec![]},
        Coord{x : original_start.x + 1,y: original_start.y + 1, collected_keys: vec![]}
    ];
    (0..maze.len())
        .map(|y|
            (0..maze[y].len())
                .map(|x| {
                    let coord = Coord{x,y, collected_keys: vec![]};
                    if walls.contains(&coord) { '#' }
                    else if portals.contains(&coord) { '@' }
                    else {at_coord(&coord, &maze)}
                }).collect::<String>()
        )
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn regression() {
        let maze1: Vec<String> = read_lines(18).iter().map(String::from).collect();
        assert_eq!(solve(&maze1), 2684); // 2684

        let maze2: Vec<String> = transform_pt2_maze(&maze1);
        assert_eq!(solve(&maze2), 1886); // 1886
    }


    #[test]
    fn test_vector_equality() {
        assert_ne!(vec![1,2,3,], vec![3,2,1]); // yup, thats an issue for my implementation
        let mut sorted = vec![3,2,1];
        sorted.sort();
        assert_eq!(vec![1,2,3,], sorted);
        assert_eq!(vec![1,2,3,], vec![1,2,3]);
    }

    #[test]
    fn test_patch_pt2() {

        let maze_before =
            "#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#Ab#
#######".split('\n').map(String::from).collect();
        let maze_after : Vec<String> =
"#######
#a.#Cd#
##@#@##
#######
##@#@##
#cB#Ab#
#######".split('\n').map(String::from).collect();
        assert_eq!(transform_pt2_maze(&maze_before), maze_after);
    }

    #[test]
    fn test_86() {
        let mut ex =
            "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";

        let mut maze = ex.split('\n').map(String::from).collect();
        assert_eq!(solve(&maze), 86);
    }

        #[test]
        fn test_132() {
            let ex =
                "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

            let maze = ex.split('\n').map(String::from).collect();
            assert_eq!(solve(&maze), 132);
        }

    #[test]
    fn test_136() {
        let ex =
            "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

        let maze = ex.split('\n').map(String::from).collect();
        assert_eq!(solve(&maze),136);
    }

    #[test]
    fn test_136_unreachable_keys() {
        let ex =
"#################
#Z.a.fc...e..Hxp#
########@########
#################";

        let maze = ex.split('\n').map(String::from).collect();
        let start = walkables(&maze).get(&'@').unwrap().clone();
        assert_eq!(parse(&maze, &start),(vec!['a','c','e','f','p','x'],vec!['H','Z']));
    }
    #[test]
    fn test2_8() {
        let maze =
            "#######
#a.#Cd#
##@#@##
#######
##@#@##
#cB#Ab#
#######".split('\n').map(String::from).collect();
        assert_eq!(solve(&maze),8);
    }
    #[test]
    fn test2_72() {
        let maze =

            "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba@#@BcIJ#
#############
#nK.L@#@G...#
#M###N#H###.#
#o#m..#i#jk.#
#############"
                .split('\n').map(String::from).collect();
        assert_eq!(solve(&maze),72);
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
