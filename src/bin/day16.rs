use std::collections::HashMap;
use aoc2019::read_lines;
use aoc2019::intcode::IntCode;
use std::sync::mpsc::{Sender, Receiver};
use std::iter::{repeat, Skip, FlatMap};
use std::time::Instant;

fn main() {
    let input = &read_lines(16)[0];
    println!("pt1: {}", pt1(&input)); // 22122816
    //println!("pt2: {}", pt2(&mem)); //
    //fft_10000(&input, 100);
}

fn pt1(n: &String) -> String {
    fft(n, 100)[..8].to_string()
}

/*
fn fft_10000(n: &String, phase_cnt: usize) -> String{
    let base_pattern = [0,1,0,-1].to_vec();
    let mut seq : Vec<i32> = n.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    for _ in 0..phase_cnt {
        let mut out_list: Vec<i32> = vec![];
        for i in 0..seq.len() {
            let pattern = base_pattern.iter()
            .flat_map(|x| {
                repeat(*x).take(i + 1).collect::<Vec<i32>>()
            })
            .cycle()
            .skip(1);
            let out_raw : i32 = seq.iter()
                .zip(pattern)
                .map(|(lhs,rhs)| lhs * rhs)
                .sum();
            let out = (out_raw % 10).abs();
            out_list.push(out);
        }

        //print!("{:?}", out_list[0]);
        seq = out_list
    }
    let str_res : Vec<String> = seq.iter().map(|x| x.to_string()).collect();
    str_res.concat()
}
*/
fn fft(n: &String, phase_cnt: usize) -> String{
    let base_pattern = [0,1,0,-1].to_vec();
    let mut seq : Vec<i32> = n.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();

    let patterns : Vec<Vec<i32>> = (0..seq.len())
        .map(|i| {
            let pattern : Vec<i32> = base_pattern.iter()
                .flat_map(|x| {
                    repeat(*x).take(i + 1).collect::<Vec<i32>>()
                })
                .cycle().take(seq.len()+1)
                .collect();
            pattern[1..].to_vec()
        })
        .collect();

    let mut out_list: Vec<i32> = seq.clone();
    for _ in 0..phase_cnt {
        for i in 0..out_list.len() {
            let out_raw : i32 = out_list.iter().zip(patterns[i].iter())
                .filter(|(lhs,rhs)| **lhs != 0 && **rhs != 0)
                .map(|(lhs,rhs)| lhs * rhs)
                .sum();
            let out = (out_raw % 10).abs();
            out_list[i] = out;
        }
    }
    let str_res : Vec<String> = out_list.iter().map(|x| x.to_string()).collect();
    str_res.concat()
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_regression() {
        let input = &read_lines(16)[0];
        assert_eq!(pt1(&input), "22122816");
    }
    #[test]
    fn test_ex() {
        fft(&"12345678".to_string(),3);
        assert_eq!(fft(&"12345678".to_string(),1), "48226158");
        assert_eq!(fft(&"12345678".to_string(),2), "34040438");
        assert_eq!(fft(&"12345678".to_string(),3), "03415518");
        assert_eq!(fft(&"12345678".to_string(),4), "01029498");
    }

    #[test]
    fn test_ex_100() {
        assert_eq!(pt1(&"80871224585914546619083218645595".to_string()), "24176176");
        assert_eq!(pt1(&"19617804207202209144916044189917".to_string()), "73745418");
        assert_eq!(pt1(&"69317163492948606335995924319873".to_string()), "52432133");
    }
}