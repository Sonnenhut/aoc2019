use aoc2019::read_lines;
use aoc2019::intcode::IntCode;

fn main() {
    let mem: Vec<i64> = read_lines(5)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", IntCode::resolve_single(&vec![1], &mem)); // 16434972
    println!("pt2: {}", IntCode::resolve_single(&vec![5], &mem)); // 16694270
}