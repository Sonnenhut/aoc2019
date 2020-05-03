use aoc2019::read_lines;
use aoc2019::intcode::IntCode;
use std::collections::{HashMap, HashSet, BinaryHeap, LinkedList};
use std::cmp::Ordering;
use std::iter::{Map, repeat};
use std::path::Iter;
use std::convert::TryInto;

use std::iter;
use std::ops::{Sub, RemAssign};
use std::fmt::DebugTuple;

fn main() {
    let instr = read_lines(22);
    println!("pt1: {}", pt1(&instr)); // 6526
    println!("pt2: {}", pt2(&instr)); //
}

fn pt1(instr: &Vec<String>) -> usize {
    apply_all(&instr, &deck(10007)).iter().position(|v| *v == 2019usize).unwrap();
    resulting_idx_of_card(instr, 10007, 2019)
}

fn pt2(instr: &Vec<String>) -> usize {
    let deck_size: usize = 119315717514047;
    let iterations: usize = 101741582076661;
    card_at_idx(instr,iterations, deck_size, 2020)
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

fn card_at_idx(instr: &Vec<String>, loops: usize, deck_size: usize, idx: usize) -> usize {
    // calculates what card is at the specified index after shuffling
    let mut lcf = composite_lcf(instr, deck_size);
    lcf = pow_composite(lcf, loops, deck_size);
    exec_lcf_inverted(idx, lcf,deck_size)
}

fn resulting_idx_of_card(instr: &Vec<String>, deck_size: usize, idx: usize) -> usize {
    // this calculates what position the card at idx will end up in
    let lcf = composite_lcf(instr, deck_size);
    println!("resulting lcf {:?}", lcf);
    println!("executed lcf {:?}", exec_lcf(idx, lcf, deck_size));
    exec_lcf(idx, lcf, deck_size)
}

// method 1
fn exec_lcf_k(k: u32, x: usize, ab: (isize, isize), modulo: usize) -> usize {
    // whereas k is the amount of loops to be executed
    let (a,b) = ab;
    let x_isize = x as isize;
    let modulo_isize = modulo as isize;
    // f(x) = ax+b mod m
    ((a.pow(k) * x_isize) + ((b * (1 - a.pow(k))) / (1-a))).checked_rem_euclid(modulo_isize).unwrap() as usize
}

// method 2
fn pow_composite(f_init: (isize, isize), k: usize, modulo: usize) -> (isize, isize) {
    let mut g = (1,0);
    let mut f = f_init;
    let mut curr_k = k;
    while curr_k > 0 {
        if curr_k % 2 == 1 { // is odd
            g = combine_lcf(modulo, g, f)
        }
        curr_k = curr_k / 2;
        f = combine_lcf(modulo, f, f)
    }
    g
}


fn exec_lcf_inverted(x: usize, ab: (isize, isize), modulo: usize) -> usize {
    let (a,b) = ab;
    let x_isize = x as isize;
    let modulo_isize = modulo as isize;
    // use modulus, not remainder (to work with negative values... see modulo vs remainder)
    // f(x) = ax+b mod m
    (((x_isize - b) / a).checked_rem_euclid(modulo_isize)).unwrap() as usize
}
fn exec_lcf(x: usize, ab: (isize, isize), modulo: usize) -> usize {
    let (a,b) = ab;
    let x_isize = x as isize;
    let modulo_isize = modulo as isize;
    // use modulus, not remainder (to work with negative values... see modulo vs remainder)
    // f(x) = ax+b mod m
    ((a * x_isize + b).checked_rem_euclid(modulo_isize)).unwrap() as usize
}

fn composite_lcf(instructions: &Vec<String>, modulo: usize) -> (isize, isize) {
    let lcf_instructions : Vec<(isize,isize)> = instructions.iter().map(|instr| {
        if instr.starts_with("deal into new stack") {
            lcf_deal()
        } else if instr.starts_with("deal with increment ") {
            let inc = instr.split(" ").last().unwrap().parse().unwrap();
            lcf_inc(inc)
        } else if instr.starts_with("cut") {
            let cut = instr.split(" ").last().unwrap().parse().unwrap();
            lcf_cut(cut)
        } else { panic!("unknown instruction...") }
    }).collect();
    println!("instructions {:?}", lcf_instructions);
    let first_instr = *lcf_instructions.first().unwrap();
    lcf_instructions.iter().skip(1).fold(first_instr, |f, s| combine_lcf(modulo, f, *s))
}


//https://codeforces.com/blog/entry/72593
fn lcf_deal() -> (isize, isize) { (-1, -1) }
fn lcf_cut(n: isize) -> (isize, isize) { (1, n * -1)}
fn lcf_inc(n: isize) -> (isize, isize) { (n, 0)}

fn combine_lcf(modulo: usize, first: (isize, isize), second: (isize,isize)) -> (isize, isize) {
    //https://www.reddit.com/r/adventofcode/comments/eh1d6p/2019_day_22_part_2_tutorial/
    //https://codeforces.com/blog/entry/72593
    let (a,b) = first;
    let (c,d) = second;
    let mod_isize = modulo as isize;
    println!("{} * {} mod {}, {} * {} + {} mod {}",a,c,modulo,b,c,d,modulo);
    ((a*c).checked_rem_euclid(mod_isize).unwrap(), ((b*c) + d).checked_rem_euclid(mod_isize).unwrap())
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
    fn test_lcf() {
        // vec![3,4,5,6,7,8,9,0,1,2]
        assert_eq!(resulting_idx_of_card(&vec![String::from("cut 3")], 10, 0), 7);
        assert_eq!(resulting_idx_of_card(&vec![String::from("cut 3")], 10, 1), 8);
        assert_eq!(resulting_idx_of_card(&vec![String::from("cut 3")], 10, 2), 9);
        assert_eq!(resulting_idx_of_card(&vec![String::from("cut 3")], 10, 3), 0);
        assert_eq!(resulting_idx_of_card(&vec![String::from("cut 3")], 10, 4), 1);
        assert_eq!(resulting_idx_of_card(&vec![String::from("cut 3")], 10, 5), 2);
        assert_eq!(resulting_idx_of_card(&vec![String::from("cut 3")], 10, 6), 3);
        assert_eq!(resulting_idx_of_card(&vec![String::from("cut 3")], 10, 7), 4);
        // .. and so on

        let mut instr_vec = r#"deal with increment 7
deal into new stack
deal into new stack"#.split("\n").map(|s|String::from(s)).collect();
        // vec![0,3,6,9,2,5,8,1,4,7]
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 0), 0);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 3), 1);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 6), 2);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 9), 3);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 2), 4);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 5), 5);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 8), 6);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 1), 7);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 4), 8);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 7), 9);
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