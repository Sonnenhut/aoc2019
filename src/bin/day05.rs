use aoc2019::read_lines;
use aoc2019::intcode::IntCode;

fn main() {
    let nums: Vec<i64> = read_lines(5)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", IntCode::resolve(&vec![1], &nums)); // 16434972
    println!("pt2: {}", IntCode::resolve(&vec![5], &nums)); // 16694270
}