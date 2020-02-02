use aoc2019::read_lines;
use aoc2019::intcode::IntCode;
use std::collections::{HashMap, HashSet};

fn main() {
    let mem: Vec<i64> = read_lines(19)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", pt1(&mem)); // 234
    println!("pt2: {}", pt2(&mem)); // 9290812
}

fn pt1(mem: &Vec<i64>) -> usize {
    let mut cnt = 0;
    for x in 0..50 {
        for y in 0..50 {
            cnt += IntCode::resolve_single(&vec![x, y], &mem);
        }
    }
    cnt as usize
}

fn pt2(mem: &Vec<i64>) -> usize {
    let mut lazor: HashSet<(usize, usize)> = HashSet::new();
    let mut last_lazer_y_start = 0;

    for x in 0.. {
        if x < 99 { continue; }
        for y in last_lazer_y_start.. {
            let out = IntCode::resolve_single(&vec![x, y], &mem);

            if out == 1 { // current pos is at the lower left edge of the square
                last_lazer_y_start = y;

                let left_up_edge = &vec![x - 99, y];
                let right_up_edge = &vec![x - 99, y + 99];
                let right_down_edge = &vec![x, y + 99];
                if IntCode::resolve_single(&left_up_edge, &mem) == 1
                    && IntCode::resolve_single(&right_up_edge, &mem) == 1
                    && IntCode::resolve_single(&right_down_edge, &mem) == 1 {
                    return (left_up_edge[0] * 10000 + left_up_edge[1]) as usize
                }
                break; // move to next row...
            }
        }
    }

    panic!("nothing found")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn regression() {
        let mem: Vec<i64> = read_lines(19)[0].split(',').map(|s| s.parse().unwrap()).collect();
        assert_eq!(gauge_beam(&mem), 234);

    }
}