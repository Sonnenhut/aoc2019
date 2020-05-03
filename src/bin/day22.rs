use aoc2019::read_lines;
use aoc2019::intcode::IntCode;
use std::collections::{HashMap, HashSet, BinaryHeap, LinkedList};
use std::cmp::Ordering;
use std::iter::{Map, repeat};
use std::path::Iter;
use std::convert::TryInto;

use std::iter;
use std::ops::Sub;
use std::fmt::DebugTuple;

fn main() {
    let instr = read_lines(22);
    println!("pt1: {}", pt1(&instr)); // 6526
    println!("pt2: {}", pt2(&instr)); // 1141066762
}

fn pt1(instr: &Vec<String>) -> usize {
    apply_all(&instr, &deck(10007)).iter().position(|v| *v == 2019usize).unwrap()
}

fn pt2(instr: &Vec<String>) -> usize {
    let deck_size: usize = 119315717514047;
    let iterations: usize = 101741582076661;
    println!("hashset");
    let mut known_combos = HashSet::new();

    let mut ldeck: LinkedList<usize> = LinkedList::new();
    for c in 0..deck_size {
        ldeck.push_back(c);
    }

    println!("test end");
    let mut changed = deck(deck_size);
    println!("initial deck");
    known_combos.insert(changed.clone());
    for i in 0..iterations {
        changed = apply_all(instr, &changed);
        //println!("loop {}",i);
        if !known_combos.insert(changed.clone()) {
            //panic!("found repeat at loop {}", i)
        }
    }

    apply_all(&instr, &deck(10007)).iter().position(|v| *v == 2019usize).unwrap()
}

fn apply_all(instr: &Vec<String>, initial_deck: &Vec<usize>) -> Vec<usize>{
    let mut deck : Vec<usize> = initial_deck.clone();
    for single in instr {
        deck = apply_single(single, &deck);
        //println!("{:?}:\n{:?}", single, deck)
    }
    //println!("---");
    deck
}

fn deck(deck_size: usize) -> Vec<usize> {
    (0_usize..deck_size).collect()
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

fn combine_lcf(first: (isize, isize), second: (isize,isize)) -> (isize,isize) {
    //https://www.reddit.com/r/adventofcode/comments/eh1d6p/2019_day_22_part_2_tutorial/
    //https://codeforces.com/blog/entry/72593
    let (a,b) = first;
    let (c,d) = second;

}



/*
// --- pt2 reverse the instructions because we know what card no to look at in the end
fn idx_before_instr(deck_size: usize, instr: &String, idx: usize) -> usize {
    if instr.starts_with("deal into new stack") {
        deck_size - idx // will be at the other side dude
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
}*/

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
        assert_eq!(apply_all(&vec![String::from("cut 3")], &deck(10)), vec![3,4,5,6,7,8,9,0,1,2]);
        assert_eq!(apply_all(&vec![String::from("deal into new stack")], &deck(10)), vec![9,8,7,6,5,4,3,2,1,0]);
        assert_eq!(apply_all(&vec![String::from("deal with increment 3")], &deck(10)), vec![0,7,4,1,8,5,2,9,6,3]);

        assert_eq!(apply_all(&vec![String::from("cut -4")], &deck(10)), vec![6,7,8,9,0,1,2,3,4,5]);
    }
    #[test]
    fn test_instructions() {
        let mut instr_vec = r#"deal with increment 7
deal into new stack
deal into new stack"#.split("\n").map(|s|String::from(s)).collect();
        assert_eq!(apply_all(&instr_vec, &deck(10)), vec![0,3,6,9,2,5,8,1,4,7]);

        instr_vec = r#"cut 6
deal with increment 7
deal into new stack"#.split("\n").map(|s|String::from(s)).collect();
        assert_eq!(apply_all(&instr_vec, &deck(10)), vec![3,0,7,4,1,8,5,2,9,6]);

        instr_vec = r#"deal with increment 7
deal with increment 9
cut -2"#.split("\n").map(|s|String::from(s)).collect();
        assert_eq!(apply_all(&instr_vec, &deck(10)), vec![6,3,0,7,4,1,8,5,2,9]);

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
        assert_eq!(apply_all(&instr_vec, &deck(10)), vec![9,2,5,8,1,4,7,0,3,6]);
    }
}