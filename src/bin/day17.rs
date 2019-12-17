use aoc2019::read_lines;
use std::iter::repeat;
use std::ops::Sub;
use aoc2019::intcode::IntCode;
use std::time::Duration;

fn main() {
    let mem: Vec<i64> = read_lines(17)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", pt1(&mem)); // 6680
    //println!("pt2: {}", pt2(&input)); //

}

fn pt1(intcode_mem: &Vec<i64>) -> usize {
    let (i,o) = IntCode::run_async(&intcode_mem);
    let mut map = o.iter().collect();
    //draw(&map);
    alignment_sum(&map)
}

fn draw(nums : &Vec<i64>) {
    for n in nums.iter() {
        let u = n.wrapping_abs() as u8;
        print!("{}", std::str::from_utf8(&[u]).unwrap())
    }
    println!();
}

fn alignment_sum(nums: &Vec<i64>) -> usize {
    let line_len = nums.iter().take_while(|n| **n != 10).count(); // until a newline
    let lines : Vec<Vec<i64>> = nums.iter()
        .filter(|n| **n != 10)
        .cloned()
        .collect::<Vec<i64>>()
        .chunks(line_len)
        .map(|chunk| chunk.to_vec())
        .collect();

    let mut sum = 0;
    for (y,line) in lines.iter().enumerate() {
        for (x, n) in line.iter().enumerate() {
            if *n == 35 && around(x,y,&lines).iter().all(|other_n| *other_n == 35) {
                //println!("{} {}", x,y);
                sum += x * y;
            }
        }
    }
    sum
}

fn around(x: usize, y: usize, lines : &Vec<Vec<i64>>) -> Vec<i64>{
    let mut res : Vec<i64> = vec![];
    if y != 0 && y < lines.len() -1 {
        res.push(lines[y-1][x]);
        res.push(lines[y+1][x]);
    } else {
        res.push(0); // on the edge, is not a valid thing
        res.push(0);
    }
    if x != 0 && x < lines[0].len() -1 {
        res.push(lines[y][x+1]);
        res.push(lines[y][x-1]);
    } else {
        res.push(0); // on the edge, is not a valid thing
        res.push(0);
    }
    res
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_regression() {
        //let input = &read_lines(16)[0];
        //assert_eq!(pt1(&input), "22122816");
    }
    #[test]
    fn test_ex() {
        let ex : Vec<i64> =
"..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..".chars().map(|c| c as i64).collect();

        //draw(&ex);
        assert_eq!(alignment_sum(&ex), 76);
    }

}