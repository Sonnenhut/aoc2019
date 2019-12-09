use aoc2019::read_lines;
use aoc2019::intcode::IntCode;

fn main() {
    let nums: Vec<i64> = read_lines(9)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", IntCode::resolve(&vec![1], &nums)); // 4261108180
    println!("pt2: {}", IntCode::resolve(&vec![2], &nums)); // 77944
}