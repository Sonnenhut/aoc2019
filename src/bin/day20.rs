use aoc2019::read_lines;
use aoc2019::intcode::IntCode;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;
use std::iter::{Map, repeat};
use std::path::Iter;
use std::convert::TryInto;

use std::iter;

const START_NAME: &str = "AA";
const GOAL_NAME: &str = "ZZ";

fn main() {
    let mem: Vec<Vec<char>> = read_lines(20).iter().map(|s| s.chars().collect::<Vec<char>>()).collect();
    println!("pt1: {}", pt1(&mem)); // 498
    println!("pt2: {}", pt2(&mem)); // 5564
}

fn pt1(maze: &Vec<Vec<char>>) -> usize {
    let (start, goal) = start_and_goal(maze);
    shortest_path(&maze, start, goal, false).unwrap()
}
fn pt2(maze: &Vec<Vec<char>>) -> usize {
    let (start, goal) = start_and_goal(maze);
    shortest_path(&maze, start, goal, true).unwrap()
}


#[derive(Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    edge: Edge
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.cost.cmp(&self.cost)
        .then_with(|| other.edge.level.cmp(&self.edge.level))
        .then_with(|| self.edge.position.cmp(&other.edge.position))
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Edge {
    position: Coord,
    level: usize
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn shortest_path(maze: &Vec<Vec<char>>, start: Coord, goal: Coord, with_levels: bool) -> Option<usize> {
    let shortcuts = connected_portals(maze);
    let mut dist : HashMap<Edge, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    dist.insert(Edge{position: start.clone(), level: 0},0);
    heap.push(State { cost: 0, edge: Edge{position: start, level: 0} });

    while let Some(State { cost, edge}) = heap.pop() {
        let Edge {position, level} = edge.clone();
        if position == goal && level == 0 { return Some(cost); }

        if cost > dist[&edge] { continue; }

        let mut possible_neighbors: Vec<(Coord, usize)> = around_walkable(&position, maze)
            .into_iter()
            .zip(repeat(level))
            .collect(); // neighbors on same level
        if let Some(portal_buddy) = shortcuts.get(&position) {
            let mut new_lvl = if is_at_outer_edge(&position,maze) {
                if level == 0 { None/* cannot use outer portal on lvl 0 */ } else { level.checked_sub(1) }
            } else {
                level.checked_add(1)
            };
            new_lvl = if with_levels { new_lvl } else { Some(level) };
            if let Some(lvl) = new_lvl {
                possible_neighbors.push((portal_buddy.clone(), lvl));
            }
        }
        for (neighbor, new_lvl) in possible_neighbors {
            let next = State { cost: cost + 1, edge: Edge { position: neighbor.clone(), level: new_lvl } };
            if next.cost < *dist.get(&next.edge).unwrap_or(&usize::max_value()) {
                heap.push(next.clone());
                dist.insert(next.edge.clone(), next.cost);
            }
        }
    }
    None
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
struct Coord {
    x: usize,
    y: usize
}

fn at_coord(coord: &Coord, maze: &Vec<Vec<char>>) -> Option<char> {
    maze.get(coord.y)
        .and_then(|row| row.get(coord.x))
        .map(|c| *c)
}

fn portals(maze: &Vec<Vec<char>>) -> Vec<(String, Coord)> {
    portal_parts(maze).iter()
        .filter_map(|c|  {
            let surroundings = around(c,maze);
            let portal = surroundings.iter()
                .filter(|(portalCoord,charr)| *charr == '.')
                .nth(0);
            let one_portal_name = at_coord(c, maze).unwrap();
            let other_portal_name = surroundings.iter()
                .filter(|(portalCoord,charr)| charr.is_alphabetic())
                .map(|(portalCoord,charr)| *charr)
                .nth(0);
            let portal_with_loc: Option<(String, Coord)> = portal.map(|(portalCoord, charr)|{
                let full_portal_name_v = &mut vec![other_portal_name.unwrap(), one_portal_name];
                full_portal_name_v.sort();
                (full_portal_name_v.iter().cloned().collect(), portalCoord.clone())
            });
            portal_with_loc
        }
        ).collect()
}

fn portal_parts(maze: &Vec<Vec<char>>) -> Vec<Coord> {
    maze.iter()
        .enumerate()
        .flat_map(|(y,row)| {
            row.iter().enumerate()
                .filter(|(x,c)| c.is_alphabetic())
                .map(|(x,_)| Coord{x,y})
                .collect::<Vec<Coord>>()
        })
        .collect()
}

fn start_and_goal(maze: &Vec<Vec<char>>) -> (Coord, Coord) {
    let portals: Vec<(String, Coord)> = portals(maze);
    let start = portals.iter().filter(|t| &t.0 == START_NAME).nth(0).unwrap().1.clone();
    let goal = portals.iter().filter(|t| &t.0 == GOAL_NAME).nth(0).unwrap().1.clone();
    (start,goal)
}

fn connected_portals(maze: &Vec<Vec<char>>) -> HashMap<Coord, Coord> {
    let portals: Vec<(String, Coord)> = portals(maze);

    let connected_portals: HashMap<Coord, Coord> = portals.clone().into_iter()
        .filter_map(|(name, one_coord)| {
            portals.clone().iter()
                .filter(|(not_start_or_goal,_)| not_start_or_goal != &START_NAME && not_start_or_goal != &GOAL_NAME)
                .filter(|(same_name, other_coord)| &name == same_name && &one_coord != other_coord) // don't find the same coord again ..
                .nth(0)
                .map(|t|(t.1.clone(), one_coord.clone()))
        })
        .flat_map(|t|vec![t.clone(),(t.1,t.0)].into_iter()) // we can also go back through the portal..
        .collect();

    connected_portals
}

fn around(c: &Coord, maze: &Vec<Vec<char>>) -> Vec<(Coord, char)> {
    vec![c.x.checked_sub(1).map(|x|Coord{x, y:c.y}), // left
         c.x.checked_add(1).map(|x|Coord{x, y:c.y}), // right
         c.y.checked_sub(1).map(|y|Coord{x:c.x, y}), // above
         c.y.checked_add(1).map(|y|Coord{x:c.x, y}), // below
    ].into_iter()
        .filter_map(|opt| opt)
        .filter_map(|new_c| at_coord(&new_c, maze).map(|charr|(new_c, charr)))
        .collect()
}
fn around_walkable(c: &Coord, maze: &Vec<Vec<char>>) -> Vec<Coord> {
    around(c,maze).into_iter()
        .filter(|(portalCoord,charr)| *charr == '.')
        .map(|t|t.0)
        .collect()
}

fn is_at_outer_edge(c: &Coord, maze: &Vec<Vec<char>>) -> bool {
    let max_y = maze.len() - 1;
    let max_x = maze.iter().map(|s|s.len()).max().unwrap() - 1;
    (0_usize..=2_usize).contains(&c.x) // left edge
    || (max_x-3..=max_x).contains(&c.x) // right edge
    || (0_usize..=2_usize).contains(&c.y) // upper edge
    || (max_y-3..=max_y).contains(&c.y) // lower edge
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn regression() {
        let mem: Vec<Vec<char>> = read_lines(20).iter().map(|s| s.chars().collect::<Vec<char>>()).collect();
        assert_eq!(pt1(&mem), 498);
        assert_eq!(pt2(&mem), 5564);
    }

    #[test]
    fn ex2() {
        let mut ex =
"             Z L X W       C
             Z P Q B       K
  ###########.#.#.#.#######.###############
  #...#.......#.#.......#.#.......#.#.#...#
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###
  #.#...#.#.#...#.#.#...#...#...#.#.......#
  #.###.#######.###.###.#.###.###.#.#######
  #...#.......#.#...#...#.............#...#
  #.#########.#######.#.#######.#######.###
  #...#.#    F       R I       Z    #.#.#.#
  #.###.#    D       E C       H    #.#.#.#
  #.#...#                           #...#.#
  #.###.#                           #.###.#
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#
CJ......#                           #.....#
  #######                           #######
  #.#....CK                         #......IC
  #.###.#                           #.###.#
  #.....#                           #...#.#
  ###.###                           #.#.#.#
XF....#.#                         RF..#.#.#
  #####.#                           #######
  #......CJ                       NM..#...#
  ###.#.#                           #.###.#
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#
  #.....#        F   Q       P      #.#.#.#
  ###.###########.###.#######.#########.###
  #.....#...#.....#.......#...#.....#.#...#
  #####.#.###.#######.#######.###.###.#.#.#
  #.......#.......#.#.#.#.#...#...#...#.#.#
  #####.###.#####.#.#.#.#.###.###.#.###.###
  #.......#.....#.#...#...............#...#
  #############.#.#.###.###################
               A O F   N
               A A D   M                     ";
        let mut maze = ex.split('\n').map(|s| s.chars().collect::<Vec<char>>()).collect();
        assert_eq!(connected_portals(&maze).contains_key(&Coord{x:13,y:3}), false); // ZZ not connected

        assert_eq!(pt2(&maze), 396);
    }

    #[test]
    fn test_connected_portals() {
        let maze: Vec<Vec<char>> = read_lines(20).iter().map(|s| s.chars().collect::<Vec<char>>()).collect();
        assert_eq!(connected_portals(&maze)[&Coord{x:39,y:2}], Coord{x:26,y:61}); // SS is found
        assert_eq!(connected_portals(&maze)[&Coord{x:26,y:61}], Coord{x:39,y:2});

        let expected_portal_len = 29+27-2;
        assert_eq!(connected_portals(&maze).keys().len(), expected_portal_len);
        let mut unique_portals : HashSet<Coord> = connected_portals(&maze).keys().cloned().collect();
        assert_eq!(unique_portals.len(), expected_portal_len);
    }

    #[test]
    fn utility_fns() {
        let mut ex =
            "         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#AF
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z       ";
        let mut maze = ex.split('\n').map(|s| s.chars().collect::<Vec<char>>()).collect();
        assert_eq!(is_at_outer_edge(&Coord{x:0,y:6},&maze), true); // left
        assert_eq!(is_at_outer_edge(&Coord{x:1,y:6},&maze), true); // left
        assert_eq!(is_at_outer_edge(&Coord{x:2,y:6},&maze), true); // left
        assert_eq!(is_at_outer_edge(&Coord{x:3,y:6},&maze), false); // not left


        assert_eq!(is_at_outer_edge(&Coord{x:20,y:6},&maze), true); // right
        assert_eq!(is_at_outer_edge(&Coord{x:18,y:6},&maze), true); // right
        assert_eq!(is_at_outer_edge(&Coord{x:17,y:6},&maze), true); // right
        assert_eq!(is_at_outer_edge(&Coord{x:16,y:6},&maze), false); // not right

        assert_eq!(is_at_outer_edge(&Coord{x:5,y:18},&maze), true); // bottom
        assert_eq!(is_at_outer_edge(&Coord{x:5,y:17},&maze), true); // bottom
        assert_eq!(is_at_outer_edge(&Coord{x:5,y:16},&maze), true); // bottom
        assert_eq!(is_at_outer_edge(&Coord{x:5,y:15},&maze), true); // not bottom


        assert_eq!(is_at_outer_edge(&Coord{x:0,y:6},&maze), true); // top
        assert_eq!(is_at_outer_edge(&Coord{x:1,y:6},&maze), true); // top
        assert_eq!(is_at_outer_edge(&Coord{x:2,y:6},&maze), true); // top
        assert_eq!(is_at_outer_edge(&Coord{x:3,y:6},&maze), false); // not top


        assert_eq!(is_at_outer_edge(&Coord{x:9,y:6},&maze), false); // somewhere at center
    }

    #[test]
    fn ex() {
        let mut ex =
"         A
         A
  #######.#########
  #######.........#
  #######.#######.#
  #######.#######.#
  #######.#######.#
  #####  B    ###.#
BC...##  C    ###.#
  ##.##       ###.#
  ##...DE  F  ###.#
  #####    G  ###.#
  #########.#####.#
DE..#######...###.#
  #.#########.###.#
FG..#########.....#
  ###########.#####
             Z
             Z       ";
        let mut maze = ex.split('\n').map(|s| s.chars().collect::<Vec<char>>()).collect();
        assert_eq!(at_coord(&Coord {x:9,y:0},&maze),Some('A'));
        assert_eq!(at_coord(&Coord {x:9,y:1},&maze),Some('A'));
        assert_eq!(at_coord(&Coord {x:9,y:2},&maze),Some('.'));

        assert_eq!(pt1(&maze), 23);

        ex =
"                   A
                   A
  #################.#############
  #.#...#...................#.#.#
  #.#.#.###.###.###.#########.#.#
  #.#.#.......#...#.....#.#.#...#
  #.#########.###.#####.#.#.###.#
  #.............#.#.....#.......#
  ###.###########.###.#####.#.#.#
  #.....#        A   C    #.#.#.#
  #######        S   P    #####.#
  #.#...#                 #......VT
  #.#.#.#                 #.#####
  #...#.#               YN....#.#
  #.###.#                 #####.#
DI....#.#                 #.....#
  #####.#                 #.###.#
ZZ......#               QG....#..AS
  ###.###                 #######
JO..#.#.#                 #.....#
  #.#.#.#                 ###.#.#
  #...#..DI             BU....#..LF
  #####.#                 #.#####
YN......#               VT..#....QG
  #.###.#                 #.###.#
  #.#...#                 #.....#
  ###.###    J L     J    #.#.###
  #.....#    O F     P    #.#...#
  #.###.#####.#.#####.#####.###.#
  #...#.#.#...#.....#.....#.#...#
  #.#####.###.###.#.#.#########.#
  #...#.#.....#...#.#.#.#.....#.#
  #.###.#####.###.###.#.#.#######
  #.#.........#...#.............#
  #########.###.###.#############
           B   J   C
           U   P   P               ";
        maze = ex.split('\n').map(|s| s.chars().collect::<Vec<char>>()).collect();
        assert_eq!(pt1(&maze), 58);
    }
}
