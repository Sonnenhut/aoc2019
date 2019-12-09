use std::collections::VecDeque;
use std::iter::successors;
use std::ops::RangeInclusive;

use aoc2019::intcode::IntCode;
use aoc2019::read_lines;

fn main() {
    let nums: Vec<i64> = read_lines(7)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", max_thruster_out(&nums)); // 262086
    println!("pt2: {}", max_feedback_loop(&nums)); // 5371621
}

fn max_feedback_loop(pgm: &Vec<i64>) -> i64 {
    sequences(5..=9).iter()
        .map(|seq| feedback_loop(&seq, &pgm))
        .max().unwrap()
}

fn feedback_loop(seq: &Vec<u64>, pgm: &Vec<i64>) -> i64 {
    let mut programs : Vec<IntCode> = seq.iter().map(|phase_instr| IntCode::create(&vec![*phase_instr as i64], &pgm)).collect();
    let mut last_out = Some(0);
    loop {
        for pgm in programs.iter_mut() {
            pgm.push_input(last_out.unwrap() as i64);
            let out = pgm.next();
            if out.is_none() {
                return last_out.unwrap();
            } else {
                last_out = out;
            }
        }
    }
}

fn max_thruster_out(pgm: &Vec<i64>) -> i64 {
    sequences(0..=4).iter()
        .map(|seq| thruster_out(&seq, &pgm))
        .max().unwrap()
}

fn thruster_out(seq: &Vec<u64>, pgm: &Vec<i64>) -> i64 {
    let mut remaining_seq = seq.clone();
    successors(Some(0), |last| {
        if remaining_seq.is_empty() {
            None
        } else {
            let out = IntCode::resolve(&vec![remaining_seq.remove(0) as i64, *last as i64], &pgm);
            Some(out)
        }
    }).max().unwrap()
}

fn sequences(range: RangeInclusive<u64>) -> Vec<Vec<u64>> {
    let mut queue = range.collect::<VecDeque<_>>();
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
    #[test]
    fn test_feedback_loop() {
        assert_eq!(feedback_loop(&vec![9,8,7,6,5], &vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                                                         27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]),139629729);
        assert_eq!(feedback_loop(&vec![9,7,8,5,6], &vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                                                         -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                                                         53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10]),18216);
    }
    #[test]
    fn test_max_feedback_loop() {
        assert_eq!(max_feedback_loop(&vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,
                                                         27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5]),139629729);
        assert_eq!(max_feedback_loop(&vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                                                         -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                                                         53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10]),18216);
    }
}