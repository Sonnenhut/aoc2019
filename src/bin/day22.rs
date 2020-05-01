use aoc2019::read_lines;
use aoc2019::intcode::IntCode;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;
use std::iter::{Map, repeat};
use std::path::Iter;
use std::convert::TryInto;

use std::iter;
use std::ops::Sub;

fn main() {
    let instr = read_lines(22);
    println!("pt1: {}", pt1(&instr)); // 6526
    //println!("pt2: {}", pt2(&mem)); // 1141066762
}

fn pt1(instr: &Vec<String>) -> usize {
    apply_all(&instr, 10007).iter().position(|v| *v == 2019usize).unwrap()
}
/*
fn pt2(instr: &Vec<String>) -> usize {
    let deck_size: usize = 101741582076661;
    let iterations: usize = 101741582076661;
    let known_combos = HashSet::new();

    for i in 101741582076661 {

    }

    apply_all(&instr, 10007).iter().position(|v| *v == 2019usize).unwrap()
}
*/

fn apply_all(instr: &Vec<String>, deck_size: usize) -> Vec<usize>{
    let mut deck : Vec<usize> = (0_usize..deck_size).collect();
    for single in instr {
        deck = apply_single(single, &deck);
        //println!("{:?}:\n{:?}", single, deck)
    }
    //println!("---");
    deck
}

fn apply_single(instr: &String, deck: &Vec<usize>) -> Vec<usize> {
    if instr.starts_with("deal into new stack") {
        deck.clone().into_iter().rev().collect()
    } else if instr.starts_with("deal with increment ") {
        let inc : usize = instr.split(" ").last().unwrap().parse().unwrap();
        let mut res : Vec<(usize,usize)> = deck.clone()
            .iter()
            .enumerate()
            .map(|(idx, v)| ((idx * inc) % deck.len(), v.clone()))
            .collect::<Vec<(usize,usize)>>();

        res.sort_by_key(|(i,v)| *i);
        res.into_iter().map(|(_,v)|v).collect()
    } else if instr.starts_with("cut") {
        let cut_isize: isize = instr.split(" ").last().unwrap().parse().unwrap();
        let cut: usize = if cut_isize.is_negative() {
            deck.len().sub(cut_isize.abs() as usize)
        } else { cut_isize.abs() as usize };

        let top = &deck[0..cut];
        let bottom = &deck[cut..];

        bottom.iter().chain(top.iter()).cloned().collect()
    } else { panic!("unknown instruction...") }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io;
    use std::io::Write;

    #[test]
    fn regression() {
        let instr = read_lines(22);
        println!("pt1: {}", pt1(&instr)); // 6526
        //println!("pt2: {}", pt2(&instr)); // 6526
    }
    #[test]
    fn test_single_instructions() {
        assert_eq!(apply_all(&vec![String::from("cut 3")], 10), vec![3,4,5,6,7,8,9,0,1,2]);
        assert_eq!(apply_all(&vec![String::from("deal into new stack")], 10), vec![9,8,7,6,5,4,3,2,1,0]);
        assert_eq!(apply_all(&vec![String::from("deal with increment 3")], 10), vec![0,7,4,1,8,5,2,9,6,3]);

        assert_eq!(apply_all(&vec![String::from("cut -4")], 10), vec![6,7,8,9,0,1,2,3,4,5]);
    }
    #[test]
    fn test_instructions() {
        let mut instr_vec = r#"deal with increment 7
deal into new stack
deal into new stack"#.split("\n").map(|s|String::from(s)).collect();
        assert_eq!(apply_all(&instr_vec, 10), vec![0,3,6,9,2,5,8,1,4,7]);

        instr_vec = r#"cut 6
deal with increment 7
deal into new stack"#.split("\n").map(|s|String::from(s)).collect();
        assert_eq!(apply_all(&instr_vec, 10), vec![3,0,7,4,1,8,5,2,9,6]);

        instr_vec = r#"deal with increment 7
deal with increment 9
cut -2"#.split("\n").map(|s|String::from(s)).collect();
        assert_eq!(apply_all(&instr_vec, 10), vec![6,3,0,7,4,1,8,5,2,9]);

        instr_vec = r#"deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1"#.split("\n").map(|s|String::from(s)).collect();
        assert_eq!(apply_all(&instr_vec, 10), vec![9,2,5,8,1,4,7,0,3,6]);
    }
}