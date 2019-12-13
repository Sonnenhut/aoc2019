use aoc2019::read_lines;
use aoc2019::intcode::IntCode;

fn main() {
    let mem: Vec<i64> = read_lines(9)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", IntCode::resolve_single(&vec![1], &mem)); // 4261108180
    println!("pt2: {}", IntCode::resolve_single(&vec![2], &mem)); // 77944
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn regression() {
        let mem: Vec<i64> = read_lines(9)[0].split(',').map(|s| s.parse().unwrap()).collect();
        assert_eq!(IntCode::resolve_single(&vec![1], &mem), 4261108180);
        assert_eq!(IntCode::resolve_single(&vec![2], &mem), 77944);
    }
}