use std::collections::{HashMap, HashSet};

use aoc2019::read_lines;

fn main() {
    let bug_tiles = parse_bug_tiles(read_lines(24));
    println!("pt1: {}", pt1(&bug_tiles)); // 24662545
    println!("pt2: {}", pt2(&bug_tiles,200)); // 2063
}

fn pt1(bugs: &HashSet<Tile>) -> usize {
    let mut prev : HashSet<Vec<Tile>> = HashSet::new();
    let mut last_bug = bugs.clone();
    loop {
        prev.insert(sorted_tiles(&last_bug));
        last_bug = evolve_single_dim(last_bug);
        if prev.contains(&sorted_tiles(&last_bug)) { break;}
    }
    rate(last_bug)
}

fn sorted_tiles(tiles: &HashSet<Tile>) -> Vec<Tile> {
    let mut res : Vec<Tile> = tiles.iter().cloned().collect();
    res.sort_by_cached_key(|tile| format!("{:?}",tile));
    res
}

fn rate(bugs: HashSet<Tile>) -> usize {
    bugs.into_iter()
        .map(|Tile(row, col, _)| row * 5_usize + col)
        .map(|idx| 2_usize.pow(idx as u32))
        .sum()
}

fn pt2(bugs: &HashSet<Tile>, time: usize) -> usize {
    let mut res = bugs.clone();
    for _ in 0..time {
        res = evolve_multi_dim(res);
    }
    res.len()
}

fn evolve_single_dim(bugs: HashSet<Tile>) -> HashSet<Tile> {
    evolve(bugs, false)
}

fn evolve_multi_dim(bugs: HashSet<Tile>) -> HashSet<Tile> {
    evolve(bugs, true)
}

fn evolve(bugs: HashSet<Tile>, multiple_dimensions : bool) -> HashSet<Tile> {
    let mut bugs_around_map : HashMap<Tile,usize> = HashMap::new(); // tile -> bug_around_cnt

    bugs.iter().for_each(|bug| {
        // tile is around a bug
        around(bug, multiple_dimensions).into_iter().for_each(|empty| {
            let new_cnt = *bugs_around_map.get(&empty).unwrap_or(&0_usize) + 1_usize;
            bugs_around_map.insert(empty,new_cnt);
        })
    });

    // A bug dies (becoming an empty space) unless there is exactly one bug adjacent to it.
    // An empty space becomes infested with a bug if exactly one or two bugs are adjacent to it.
    bugs_around_map.into_iter()
        .filter(|(tile, cnt)| {
            let bug_stays_alive = *cnt == 1_usize && bugs.contains(tile);
            let empty_becomes_bug = (*cnt == 1_usize || *cnt == 2_usize) && !bugs.contains(tile);
            bug_stays_alive || empty_becomes_bug
        })
        .map(|(tile, _)|tile)
        .collect()
}

fn around(tile: &Tile, multiple_dimensions : bool) -> HashSet<Tile> {
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
    if row == 0 { res.insert(Tile(1,2,depth+1)); }
    // lower edge
    if row == 4 { res.insert(Tile(3,2,depth+1)); }
    // left edge
    if col == 0 { res.insert(Tile(2,1,depth+1)); }
    // right edge
    if col == 4 { res.insert(Tile(2,3,depth+1)); }

    if row != 0 { res.insert(Tile(row - 1,col,depth)); }
    if col != 0 { res.insert(Tile(row,col - 1,depth)); }
    if row != 4 { res.insert(Tile(row + 1,col,depth)); }
    if col != 4 { res.insert(Tile(row,col + 1,depth)); }

    if multiple_dimensions {
        res.remove(&Tile(2,2,depth)); // center element does not exist..
    } else {
        res = res.into_iter().filter(|Tile(_,_,depth)| *depth == 0_isize).collect();
    }
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

#[cfg(test)]
mod test {
    use std::io;
    use std::io::Write;
    use std::iter::FromIterator;

    use super::*;

    #[test]
    fn regression() {
        let bugs = parse_bug_tiles((read_lines(24)));
        assert_eq!(pt1(&bugs), 24662545);
        assert_eq!(pt2(&bugs,200), 2063);
    }

    #[test]
    fn evolve_test() {
        let input = parse_bug_tiles(
            "....#
#..#.
#..##
..#..
#....".split("\n").map(|s|String::from(s)).collect());
        let output_1 = parse_bug_tiles(
            "#..#.
####.
###.#
##.##
.##..".split("\n").map(|s|String::from(s)).collect());
        let output_2 = parse_bug_tiles(
            "#####
....#
....#
...#.
#.###".split("\n").map(|s|String::from(s)).collect());
        let output_3 = parse_bug_tiles(
            "#....
####.
...##
#.##.
.##.#".split("\n").map(|s|String::from(s)).collect());
        let output_4 = parse_bug_tiles(
            "####.
....#
##..#
.....
##...".split("\n").map(|s|String::from(s)).collect());
        assert_eq!(evolve_single_dim(input.clone()), output_1);
        assert_eq!(evolve_single_dim(evolve_single_dim(input.clone())), output_2);
        assert_eq!(evolve_single_dim(evolve_single_dim(evolve_single_dim(input.clone()))), output_3);
        assert_eq!(evolve_single_dim(evolve_single_dim(evolve_single_dim(evolve_single_dim(input.clone())))), output_4);
    }

    #[test]
    fn rate_test() {
        let mut input = parse_bug_tiles(
            ".....
.....
.....
#....
.#...".split("\n").map(|s|String::from(s)).collect());
        assert_eq!(rate(input), 2129920);

        input = parse_bug_tiles(
            ".....
.....
..#..
.....
.....".split("\n").map(|s|String::from(s)).collect());
        assert_eq!(rate(input), 4096);
    }

    #[test]
    fn around_test() {
        // on same lvl
        assert_eq!(around(&Tile(1,1,0), true),vec![Tile(2,1,0),Tile(0,1,0),Tile(1,0,0),Tile(1,2,0)].into_iter().collect());
        // Tile 19 has four adjacent tiles: 14, 18, 20, and 24.
        assert_eq!(around(&Tile(3,3,1), true),vec![Tile(2,3,1),Tile(3,2,1),Tile(3,4,1),Tile(4,3,1)].into_iter().collect());
        // Tile D has four adjacent tiles: 8, C, E, and I.
        assert_eq!(around(&Tile(0,3,0), true),vec![Tile(1,2,1),Tile(0,2,0),Tile(0,4,0),Tile(1,3,0)].into_iter().collect());
        // Tile E has four adjacent tiles: 8, D, 14, and J.
        assert_eq!(around(&Tile(0,4,0), true), vec![Tile(1,2,1),Tile(2,3,1),Tile(0,3,0),Tile(1,4,0)].into_iter().collect());
        // Tile 14 has eight adjacent tiles: 9, E, J, O, T, Y, 15, and 19.
        assert_eq!(around(&Tile(2,3,1), true), vec![Tile(0,4,0),Tile(1,4,0),Tile(2,4,0),Tile(3,4,0),Tile(4,4,0),Tile(3,3,1),Tile(1,3,1),Tile(2,4,1)].into_iter().collect());
        //Tile N has eight adjacent tiles: I, O, S, and five tiles within the sub-grid marked ?.
        assert_eq!(around(&Tile(2,3,0), true), vec![Tile(0,4,-1),Tile(1,4,-1),Tile(2,4,-1),Tile(3,4,-1),Tile(4,4,-1),Tile(3,3,0),Tile(1,3,0),Tile(2,4,0)].into_iter().collect());
    }
    #[test]
    fn evolve2_test() {
        let mut input : HashSet<Tile>  = HashSet::from_iter(vec![
            Tile(1,1,0),
            Tile(1,3,0)
        ].into_iter());
        // easy, stays in same dimension. pre-existing bugs die, all tiles around get infested
        assert_eq!(evolve_multi_dim(input.clone()), vec![Tile(1, 0, 0), Tile(0, 3, 0), Tile(0, 1, 0), Tile(1, 2, 0), Tile(2, 3, 0), Tile(1, 4, 0), Tile(2, 1, 0)].into_iter().collect());
        input = HashSet::from_iter(vec![
            Tile(0,0,0)
        ].into_iter());
        // only one edge is infested, will propagate to the outer layer
        assert_eq!(evolve_multi_dim(input.clone()), vec![Tile(2, 1, 1), Tile(1, 0, 0), Tile(0, 1, 0), Tile(1, 2, 1)].into_iter().collect());
        input = HashSet::from_iter(vec![
            Tile(1,2,0)
        ].into_iter());
        // one inner edge is infested, will propagate to inner layer
        assert_eq!(evolve_multi_dim(input.clone()), vec![Tile(0, 1, -1), Tile(0, 0, -1), Tile(0, 2, -1), Tile(1, 1, 0), Tile(0, 2, 0), Tile(0, 4, -1), Tile(1, 3, 0), Tile(0, 3, -1)].into_iter().collect());
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
