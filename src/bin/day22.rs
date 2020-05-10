
use aoc2019::read_lines;
use aoc2019::intcode::IntCode;
use std::collections::{HashMap, HashSet, BinaryHeap, LinkedList};
use std::cmp::Ordering;
use std::path::Iter;
use std::convert::TryInto;

use std::iter;
use std::ops::{Sub, RemAssign};
use std::fmt::DebugTuple;

fn main() {
    let instr = read_lines(22);
    println!("pt1: {}", pt1(&instr)); // 6526
    println!("pt2: {}", pt2(&instr)); // 1 (nope)
}

fn pt1(instr: &Vec<String>) -> usize {
    resulting_idx_of_card(instr, 10007, 2019)
}

// code from
// https://github.com/yzhong52/AdventOfCode/blob/master/src/y2019/day22.rs#L139-L180
// after many failed attempts I was not able to solve it...
fn pt2(instr: &Vec<String>) -> usize {
    let deck_size = 119315717514047;
    let parsed = parse(instr, deck_size);
    let result = shuffle_part2(
        parsed,
        2020,
        deck_size,
        101741582076661,
    );
    result as usize
}

fn resulting_idx_of_card(instr: &Vec<String>, deck_size: usize, card: usize) -> usize {
    // this calculates what position the card at idx will end up in
    let lcf = composite_lcf(instr, deck_size);
    exec_lcf(card, lcf, deck_size)
}

fn exec_lcf(x: usize, ab: (isize, isize), modulus: usize) -> usize {
    let (a,b) = ab;
    let x_i = x as isize;
    // use modulo, not remainder (to work with negative values... see modulo vs remainder)
    // f(x) = ax+b mod m
    modulo((a * x_i + b), modulus) as usize
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
    let first_instr = lcf_instructions.first().unwrap().clone();
    lcf_instructions.iter().skip(1).fold(first_instr, |f, s| combine_lcf(modulo, f, s.clone()))
}

//https://codeforces.com/blog/entry/72593
fn lcf_deal() -> (isize, isize) { (-1isize, -1isize) }
fn lcf_cut(n: isize) -> (isize, isize) { (1isize, n * -1isize)}
fn lcf_inc(n: isize) -> (isize, isize) { (n, 0isize)}

// modulo implmentation (not % remainder implementation)
fn modulo(x: isize, m: usize) -> isize {
    let m_i = m as isize;
    let res = x % m_i;
    if res < 0isize { res + m_i } else { res }
}

fn combine_lcf(modulus: usize, first: (isize, isize), second: (isize,isize)) -> (isize, isize) {
    //https://www.reddit.com/r/adventofcode/comments/eh1d6p/2019_day_22_part_2_tutorial/
    //https://codeforces.com/blog/entry/72593
    let (a,b) = first;
    let (c,d) = second;
    let newa = modulo(a*c, modulus);
    let newb = modulo(b*c + d, modulus);
    (newa, newb)
}

// -- copied part2 after MANY failed attempts...
// code from
// https://github.com/yzhong52/AdventOfCode/blob/master/src/y2019/day22.rs#L139-L180
// after many failed attempts I was not able to solve it...

// Inspired by:
// * https://www.reddit.com/r/adventofcode/comments/eh1d6p/2019_day_22_part_2_tutorial/
// * https://codeforces.com/blog/entry/72593
fn shuffle_part2(shuffles: Vec<Shuffle>, original_position: usize, deck_size: usize, times: usize) -> i128 {
    let deck_size = deck_size as i128;

    let mut reversed = shuffles.clone();
    reversed.reverse();

    // After shuffling, result = multiplier * result + constant
    // Refer to `shuffle_part1` also for the computation of `multiplier` and `constant`
    let mut multiplier: i128 = 1;
    let mut constant: i128 = 0;
    for row in shuffles {
        match row {
            Shuffle::DealWithIncrement(increment) => {
                multiplier *= increment as i128;
                constant *= increment as i128;
            }
            Shuffle::DealNewDeck => {
                multiplier = -multiplier;
                constant = -1 - constant;
            }
            Shuffle::Cut(count) => {
                constant = constant - count as i128;
            }
        }
        multiplier = multiplier % deck_size as i128;
        constant = constant % deck_size as i128;
    }

    // "When did you become and expert in modular arithmetic? "
    // https://www.reddit.com/r/adventofcode/comments/eeb40v/day_22_part_2/
    let inverse_multiplier = modinverse(multiplier, deck_size as i128).unwrap();
    let inverse_constant = (-inverse_multiplier * constant) % deck_size as i128;

    let (repeated_inverse_multiplier, repeated_inverse_constant) = repeat(
        inverse_multiplier,
        inverse_constant,
        times,
        deck_size as i128,
    );

    (original_position as i128 * repeated_inverse_multiplier + repeated_inverse_constant) % deck_size
}
fn repeat(multiplier: i128, constant: i128, count: usize, modulo: i128) -> (i128, i128) {
    if count == 1 {
        (multiplier % modulo, constant % modulo)
    } else {
        if count % 2 == 0 {
            let (m2, c2) = repeat(multiplier, constant, count / 2, modulo);
            let final_m = (m2 * m2) % modulo;
            let final_c = (m2 * c2 + c2) % modulo;
            (final_m, final_c)
        } else {
            let (m1, c1) = repeat(multiplier, constant, count - 1, modulo);
            let final_m = (multiplier * m1) % modulo;
            let final_c = (multiplier * c1 + constant) % modulo;
            (final_m, final_c)
        }
    }
}
fn parse(shuffles: &Vec<String>, deck_len: usize) -> Vec<Shuffle> {
    let mut result = vec![];
    for row in shuffles {
        if row.starts_with("deal with increment ") {
            let parts: Vec<&str> = row.split(" ").collect();
            let increment: usize = parts.last().unwrap().parse().unwrap();
            result.push(Shuffle::DealWithIncrement(increment));
        } else if row.starts_with("deal into new stack") {
            result.push(Shuffle::DealNewDeck);
        } else if row.starts_with("cut") {
            let parts: Vec<&str> = row.split(" ").collect();
            let number: i32 = parts.last().unwrap().parse().unwrap();
            if number >= 0 {
                result.push(Shuffle::Cut(number as usize % deck_len))
            } else {
                result.push(Shuffle::Cut(deck_len - (-number) as usize % deck_len))
            };
        }
    }
    result
}
#[derive(Copy, Clone)]
enum Shuffle {
    Cut(usize),
    DealWithIncrement(usize),
    DealNewDeck,
}
fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0i128 {
        (b, 0i128, 1i128)
    }
    else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}
fn modinverse(a: i128, m: i128) -> Option<i128> {
    let (g, x, _) = egcd(a, m as i128);
    if g != 1i128 {
        None
    }
    else {
        Some((x % m + m) % m)
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::io;
    use std::io::Write;

    #[test]
    fn regression() {
        let instr = read_lines(22);
        // (2183 * 2019 + 2129) mod 10007 === 6526
        // ?? ((6526 - 2129) / 2183)  mod 10007

        //95863891117386/9532079001633
        // ?? ((2020 - 9532079001633) / 95863891117386)  mod 119315717514047
        assert_eq!(pt1(&instr), 6526usize); // 6526
        // check if we can calculate the same in reverse...
        assert_eq!(pt2(&instr), 79855812422607);
    }

    #[test]
    fn test_modulo() {
        assert_eq!(modulo(-3usize,10usize), 7usize);
        assert_eq!(modulo(3usize,10usize), 3usize);
    }
    #[test]
    fn test_division_with_integers_in_rust() {
        assert_eq!(3usize / 2usize, 1usize); // floored
        assert_eq!(2usize / 2usize, 1usize);
    }

    #[test]
    fn test_lcf() {
        // vec![3,4,5,6,7,8,9,0,1,2]
        assert_eq!(resulting_idx_of_card(&vec![String::from("cut 3")], 10, 0), 7usize);
        assert_eq!(resulting_idx_of_card(&vec![String::from("cut 3")], 10, 1), 8usize);
        assert_eq!(resulting_idx_of_card(&vec![String::from("cut 3")], 10, 2), 9usize);
        assert_eq!(resulting_idx_of_card(&vec![String::from("cut 3")], 10, 3), 0usize);
        assert_eq!(resulting_idx_of_card(&vec![String::from("cut 3")], 10, 4), 1usize);
        assert_eq!(resulting_idx_of_card(&vec![String::from("cut 3")], 10, 5), 2usize);
        assert_eq!(resulting_idx_of_card(&vec![String::from("cut 3")], 10, 6), 3usize);
        assert_eq!(resulting_idx_of_card(&vec![String::from("cut 3")], 10, 7), 4usize);


        // .. and so on

        let mut instr_vec = r#"deal with increment 7
deal into new stack
deal into new stack"#.split("\n").map(|s|String::from(s)).collect();
        // vec![0,3,6,9,2,5,8,1,4,7]
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 0), 0usize);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 3), 1usize);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 6), 2usize);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 9), 3usize);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 2), 4usize);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 5), 5usize);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 8), 6usize);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 1), 7usize);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 4), 8usize);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 7), 9usize);
    }
    #[test]
    fn test_instructions() {
        let mut instr_vec = r#"deal with increment 7
deal into new stack
deal into new stack"#.split("\n").map(|s|String::from(s)).collect();
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 7), 9usize); // vec![0,3,6,9,2,5,8,1,4,7]);

        instr_vec = r#"cut 6
deal with increment 7
deal into new stack"#.split("\n").map(|s|String::from(s)).collect();
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 3), 0usize); // vec![3,0,7,4,1,8,5,2,9,6]);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 0), 1usize); // vec![3,0,7,4,1,8,5,2,9,6]);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 7), 2usize); // vec![3,0,7,4,1,8,5,2,9,6]);
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 4), 3usize); // vec![3,0,7,4,1,8,5,2,9,6]);

        instr_vec = r#"deal with increment 7
deal with increment 9
cut -2"#.split("\n").map(|s|String::from(s)).collect();
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 9), 9usize); // vec![6,3,0,7,4,1,8,5,2,9]);

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
        assert_eq!(resulting_idx_of_card(&instr_vec, 10, 6), 9usize); //vec![9,2,5,8,1,4,7,0,3,6]);
    }
}