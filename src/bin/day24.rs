use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, LinkedList};
use std::convert::TryInto;
use std::fmt::DebugTuple;
use std::iter;
use std::ops::{RemAssign, Sub};
use std::path::Iter;
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use std::thread::Thread;
use std::time::Duration;

use aoc2019::intcode::{IntCode, IntCodeClient};
use aoc2019::read_lines;
use std::borrow::Borrow;


fn main() {
    let bugs = parse_bugs(read_lines(24));
    let bug_tiles = parse_bug_tiles(read_lines(24));
    println!("pt1: {}", pt1(&bugs));
    println!("pt2: {}", pt2(&bug_tiles,200));
}

fn pt1(bugs: &Vec<Vec<bool>>) -> usize {
    let mut prev = HashSet::new();
    let mut last_bug = bugs.clone();
    loop {
        prev.insert(last_bug.clone());
        last_bug = evolve(&last_bug);
        if prev.contains(&last_bug) { break;}
    }
    rate(&last_bug)
}

fn rate(bugs: &Vec<Vec<bool>>) -> usize {
    bugs.iter()
        .flat_map(|row| row.iter())
        .enumerate()
        .filter(|(_,is_a_bug)|**is_a_bug)
        .map(|(idx, bol)| 2_usize.pow(idx as u32))
        .sum()
}

fn evolve(bugs: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let mut res = bugs.clone();
    for row_id in 0..bugs.len() {
        for col_id in 0..bugs.len() {
            let is_a_bug = is_bug(row_id,col_id,&bugs);
            let around_cnt = cnt_adjacent(row_id,col_id,&bugs);
            let will_be_bug = if is_a_bug { around_cnt == 1} else { around_cnt == 1 || around_cnt == 2 };
            res[row_id][col_id] = will_be_bug;
        }
    }
    res
}

fn cnt_adjacent(row: usize, col: usize, bugs: &Vec<Vec<bool>>) -> i64 {
    let mut res = 0;
    // up, down, left, right
    if let Some(row_adj) = row.checked_sub(1) { if is_bug(row_adj,col,bugs) {res += 1} }
    if let Some(row_adj) = row.checked_add(1) { if is_bug(row_adj,col,bugs) {res += 1} }
    if let Some(col_adj) = col.checked_sub(1) { if is_bug(row,col_adj,bugs) {res += 1} }
    if let Some(col_adj) = col.checked_add(1) { if is_bug(row,col_adj,bugs) {res += 1} }
    res
}

fn is_bug(row: usize, col: usize, bugs: &Vec<Vec<bool>>) -> bool {
    *bugs.get(row).and_then(|r|r.get(col)).unwrap_or(&false)
}

fn parse_bugs(lines: Vec<String>) -> Vec<Vec<bool>>{
    lines.iter().map(|l| l.chars().map(|c| c == '#').collect()).collect()
}

fn pt2(bugs: &HashSet<Tile>, time: usize) -> usize {
    let mut res = bugs.clone();
    for _ in 0..time {
        res = evolve2(res);
    }
    res.len()
}

fn evolve2(bugs: HashSet<Tile>) -> HashSet<Tile> {
    let mut bugs_around_map : HashMap<Tile,usize> = HashMap::new();

    bugs.iter().for_each(|bug| {
        // tile is around a bug
        around(&bug).iter().for_each(|empty| {
            let new_cnt = *bugs_around_map.get(&empty).get_or_insert(&0_usize) + 1_usize;
            bugs_around_map.insert(empty.clone(),new_cnt);
        })
    });

    // A bug dies (becoming an empty space) unless there is exactly one bug adjacent to it.
    let stays_bug : HashSet<Tile> = bugs.iter()
        .filter(|bug| **bugs_around_map.get(&bug).get_or_insert(&0_usize) == 1_usize)
        .cloned()
        .collect();
    // An empty space becomes infested with a bug if exactly one or two bugs are adjacent to it.
    let empty_evolves : HashSet<Tile> = bugs_around_map.iter()
        .filter(|(tile, around_cnt)| !bugs.contains(tile))// empty tile
        .filter(|(tile, around_cnt)| **around_cnt == 1_usize || **around_cnt == 2_usize)
        .map(|(tile,_)|tile.clone())
        .collect();

    let mut res = HashSet::new();
    stays_bug.into_iter().for_each(|bug| {res.insert(bug);});
    empty_evolves.into_iter().for_each(|bug| {res.insert(bug);});
    res
}

fn around(tile: &Tile) -> HashSet<Tile> {
    let Tile(row, col, depth) = tile.clone();
    let mut res = HashSet::new();
    // -- tiles touching center
    if row == 2 && col == 1 { // left of center
        (0..5).map(|row_adj| Tile(row_adj,0,depth - 1)).for_each(|t| {res.insert(t);});
    }
    if row == 2 && col == 3 { // right of center
        (0..5).map(|row_adj| Tile(row_adj,4,depth - 1)).for_each(|t| {res.insert(t);});
    }
    if row == 1 && col == 2 { // above center
        (0..5).map(|col_adj| Tile(0,col_adj,depth - 1)).for_each(|t| {res.insert(t);});
    }
    if row == 3 && col == 2 { // below center
        (0..5).map(|col_adj| Tile(4,col_adj,depth - 1)).for_each(|t| {res.insert(t);});
    }

    // on outer border (depth +1)
    // upper edge
    println!("#{:?}", res);
    if row == 0 { res.insert(Tile(1,2,depth+1)); }
    println!("{:?}", res);
    // lower edge
    if row == 4 { res.insert(Tile(3,2,depth+1)); }
    println!("{:?}", res);
    // left edge
    if col == 0 { res.insert(Tile(2,1,depth+1)); }
    println!("{:?}", res);
    // right edge
    if col == 4 { res.insert(Tile(2,3,depth+1)); }
    println!("{:?}", res);

    if row != 0 { res.insert(Tile(row - 1,col,depth)); }
    if col != 0 { res.insert(Tile(row,col - 1,depth)); }
    if row != 4 { res.insert(Tile(row + 1,col,depth)); }
    if col != 4 { res.insert(Tile(row,col + 1,depth)); }

    res.remove(&Tile(2,2,depth)); // center element does not exist..
    res
}

fn parse_bug_tiles(lines: Vec<String>) -> HashSet<Tile> {
    lines.into_iter()
        .enumerate()
        .flat_map(|(line_id, l)| {
            let tmp : Vec<Tile> = l.chars()
                .enumerate()
                .filter_map(|(col_id, c)| if c == '#' { Some(Tile(line_id.clone(),col_id.clone(),0)) } else { None })
                .collect();
            tmp.into_iter()
        })
        .collect::<HashSet<Tile>>()
}

#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Tile(usize,usize, isize);
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
struct Pos(usize, usize);

#[cfg(test)]
mod test {
    use std::io;
    use std::io::Write;

    use super::*;
    use std::iter::FromIterator;

    #[test]
    fn regression() {
        let bugs = parse_bugs(read_lines(24));
        assert_eq!(pt1(&bugs), 24662545);
        //assert_eq!(pt2(&mem), 12326);
    }

    #[test]
    fn evolve_test() {
        let input = parse_bugs(
            "....#
#..#.
#..##
..#..
#....".split("\n").map(|s|String::from(s)).collect());
        let output_1 = parse_bugs(
            "#..#.
####.
###.#
##.##
.##..".split("\n").map(|s|String::from(s)).collect());
        let output_2 = parse_bugs(
            "#####
....#
....#
...#.
#.###".split("\n").map(|s|String::from(s)).collect());
        let output_3 = parse_bugs(
            "#....
####.
...##
#.##.
.##.#".split("\n").map(|s|String::from(s)).collect());
        let output_4 = parse_bugs(
            "####.
....#
##..#
.....
##...".split("\n").map(|s|String::from(s)).collect());
        assert_eq!(evolve(&input), output_1);
        assert_eq!(evolve(&evolve(&input)), output_2);
        assert_eq!(evolve(&evolve(&evolve(&input))), output_3);
        assert_eq!(evolve(&evolve(&evolve(&evolve(&input)))), output_4);
    }

    #[test]
    fn rate_test() {
        let mut input = parse_bugs(
            ".....
.....
.....
#....
.#...".split("\n").map(|s|String::from(s)).collect());
        assert_eq!(rate(&input), 2129920);
    }

    #[test]
    fn bugs_from_string() {
        let strings: Vec<String> = read_lines(24);
        assert_eq!(parse_bugs(strings), vec![
            vec![true,false,false,true,false],
            vec![false,false,true,false,false],
            vec![false,false,false,true,true],
            vec![false,false,false,true,false],
            vec![true,false,true,true,true],
        ]);
    }
    #[test]
    fn cnt_adjacent_test() {
        let bugs: Vec<Vec<bool>> = parse_bugs(read_lines(24));
        assert_eq!(cnt_adjacent(0,0, &bugs), 0);
        assert_eq!(cnt_adjacent(0,3, &bugs), 0);
        assert_eq!(cnt_adjacent(1,2, &bugs), 0);
        assert_eq!(cnt_adjacent(2,3, &bugs), 2);
    }
    #[test]
    fn is_bug_test() {
        let bugs: Vec<Vec<bool>> = parse_bugs(read_lines(24));
        assert_eq!(is_bug(0,0, &bugs), true);
        assert_eq!(is_bug(0,3, &bugs), true);
        assert_eq!(is_bug(1,2, &bugs), true);
        assert_eq!(is_bug(2,3, &bugs), true);
        assert_eq!(is_bug(0,1, &bugs), false);
    }

    #[test]
    fn around_test() {
        // on same lvl
        assert_eq!(around(&Tile(1,1,0)),vec![Tile(2,1,0),Tile(0,1,0),Tile(1,0,0),Tile(1,2,0)].into_iter().collect());
        // Tile 19 has four adjacent tiles: 14, 18, 20, and 24.
        assert_eq!(around(&Tile(3,3,1)),vec![Tile(2,3,1),Tile(3,2,1),Tile(3,4,1),Tile(4,3,1)].into_iter().collect());
        // Tile D has four adjacent tiles: 8, C, E, and I.
        assert_eq!(around(&Tile(0,3,0)),vec![Tile(1,2,1),Tile(0,2,0),Tile(0,4,0),Tile(1,3,0)].into_iter().collect());
        // Tile E has four adjacent tiles: 8, D, 14, and J.
        assert_eq!(around(&Tile(0,4,0)), vec![Tile(1,2,1),Tile(2,3,1),Tile(0,3,0),Tile(1,4,0)].into_iter().collect());
        // Tile 14 has eight adjacent tiles: 9, E, J, O, T, Y, 15, and 19.
        assert_eq!(around(&Tile(2,3,1)), vec![Tile(0,4,0),Tile(1,4,0),Tile(2,4,0),Tile(3,4,0),Tile(4,4,0),Tile(3,3,1),Tile(1,3,1),Tile(2,4,1)].into_iter().collect());
        //Tile N has eight adjacent tiles: I, O, S, and five tiles within the sub-grid marked ?.
        assert_eq!(around(&Tile(2,3,0)), vec![Tile(0,4,-1),Tile(1,4,-1),Tile(2,4,-1),Tile(3,4,-1),Tile(4,4,-1),Tile(3,3,0),Tile(1,3,0),Tile(2,4,0)].into_iter().collect());
    }
    #[test]
    fn evolve2_test() {
        let mut input : HashSet<Tile>  = HashSet::from_iter(vec![
            Tile(1,1,0),
            Tile(1,3,0)
        ].into_iter());
        // easy, stays in same dimension. pre-existing bugs die, all tiles around get infested
        assert_eq!(evolve2(input.clone()),vec![Tile(1, 0, 0), Tile(0, 3, 0), Tile(0, 1, 0), Tile(1, 2, 0), Tile(2, 3, 0), Tile(1, 4, 0), Tile(2, 1, 0)].into_iter().collect());
        input = HashSet::from_iter(vec![
            Tile(0,0,0)
        ].into_iter());
        // only one edge is infested, will propagate to the outer layer
        assert_eq!(evolve2(input.clone()),vec![Tile(2, 1, 1), Tile(1, 0, 0), Tile(0, 1, 0), Tile(1, 2, 1)].into_iter().collect());
        input = HashSet::from_iter(vec![
            Tile(1,2,0)
        ].into_iter());
        // one inner edge is infested, will propagate to inner layer
        assert_eq!(evolve2(input.clone()),vec![Tile(0, 1, -1), Tile(0, 0, -1), Tile(0, 2, -1), Tile(1, 1, 0), Tile(0, 2, 0), Tile(0, 4, -1), Tile(1, 3, 0), Tile(0, 3, -1)].into_iter().collect());
    }
    #[test]
    fn parse_tile_test() {
        let input =
            "....#
#..#.
#..##
..#..
#....".split("\n").map(|s|String::from(s)).collect();
        // parse the thing properly
        assert_eq!(parse_bug_tiles(input), HashSet::from_iter(vec![Tile(0, 4, 0), Tile(1, 0, 0), Tile(1, 3, 0), Tile(2, 0, 0), Tile(2, 3, 0), Tile(2, 4, 0), Tile(3, 2, 0), Tile(4, 0, 0)]))
    }
    #[test]
    fn evolve_tile_example_test() {
        let input = parse_bug_tiles(
            "....#
#..#.
#..##
..#..
#....".split("\n").map(|s|String::from(s)).collect::<Vec<String>>());
        // parse the thing properly
        assert_eq!(pt2(&input,10), 99_usize)
    }

}
