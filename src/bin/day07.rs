
use std::iter::successors;

use aoc2019::read_lines;
use aoc2019::intcode::IntCode;
use std::collections::VecDeque;

fn main() {
    let nums: Vec<i32> = read_lines(7)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", max_thruster_out(&nums)); //
    //println!("pt2: {}", max_thruster_out(&nums)); //
}

fn run_amp(inputs: Vec<i32>, pgm: Vec<i32>) -> i32 {
    IntCode::resolve(&inputs, &pgm)
}

fn max_thruster_out(pgm: &Vec<i32>) -> i32 {
    sequences().iter()
        .map(|seq| thruster_out(&seq, &pgm))
        .max().unwrap()
}

fn thruster_out(seq: &Vec<u32>, pgm: &Vec<i32>) -> i32 {
    let mut remaining_seq = seq.clone();
    successors(Some(0), |last| {
        if(remaining_seq.is_empty()) {
            None
        } else {
            let out = IntCode::resolve(&vec![remaining_seq.remove(0) as i32, *last as i32], &pgm);
            Some(out)
        }
    }).max().unwrap()
}

fn sequences() -> Vec<Vec<u32>> {
    let mut queue = (0..5).collect::<VecDeque<_>>();
    permute(&mut Vec::new(), &mut queue)
}

fn permute<T: Clone>(used: &mut Vec<T>, unused: &mut VecDeque<T>) -> Vec<Vec<T>>{
    if unused.is_empty() {
        vec![used.to_vec()]
    } else {
        let mut res = vec![];
        for _ in 0..unused.len() {
            used.push(unused.pop_front().unwrap());
            res = [res, permute(used, unused)].concat();
            unused.push_back(used.pop().unwrap());
        }
        res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex() {
        assert_eq!(thruster_out(&vec![4,3,2,1,0], &vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]),43210);
        assert_eq!(thruster_out(&vec![0,1,2,3,4], &vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
                                                        101,5,23,23,1,24,23,23,4,23,99,0,0]),54321);
        assert_eq!(thruster_out(&vec![1,0,4,3,2], &vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                                                        1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]),65210);
    }

    #[test]
    fn test_max_thruster_out() {
        assert_eq!(max_thruster_out(&vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]),43210);
        assert_eq!(max_thruster_out( &vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,
                                                        101,5,23,23,1,24,23,23,4,23,99,0,0]),54321);
        assert_eq!(max_thruster_out( &vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,
                                                        1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]),65210);
    }
}