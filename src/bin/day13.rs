use aoc2019::read_lines;
use aoc2019::intcode::IntCode;
use std::io;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use aoc2019::intcode2::IntCode;
use std::sync::mpsc::RecvTimeoutError::Disconnected;
use std::sync::mpsc::{RecvTimeoutError, RecvError};

fn main() {
    let mem: Vec<i64> = read_lines(13)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", pt1(&mem)); // 452
    println!("pt2: {}", pt2(&mem, false)); // 21415
}

fn pt2(mem: &Vec<i64>, print: bool) -> i64 {
    let mut free_play = mem.to_vec();
    free_play[0] = 2;
    let (input, output) = IntCode::run_async(&free_play);
    let mut map: HashMap<(i64,i64),i64> = HashMap::new();

    loop {
        let mut x_opt = output.recv();
        match x_opt {
            Err(_) => {break;},
            Ok(None) => {input.send(calc_next_input(&map));},
            Ok(Some(x)) => {
                let (y_opt, val_opt) = (output.recv().unwrap(), output.recv().unwrap());
                if let (Some(y), Some(val)) = (y_opt.clone(), val_opt.clone()) {
                    map.insert((x,y), val);
                    if print { draw(&map); }
                } else { panic!("Got an inconsistent amount of outputs!");}
            }
        }
    }
    score(&map)
}

fn draw(map: &HashMap<(i64,i64), i64>) {
    print!("\x1B[2J"); // clear
    io::stdout().flush();
    let max_x = *map.keys().map(|(x,_)|x).max().unwrap_or(&0);
    let max_y = *map.keys().map(|(_,y)|y).max().unwrap_or(&0);
    for y in 0..=max_y {
        for x in 0..=max_x {
            let v = match map.get(&(x as i64,y as i64)) {
                Some(0) => " ", // no
                Some(1) => "█", // wall
                Some(2) => "X", // block
                Some(3) => "_", // paddle
                Some(4) => "O", //ball
                _ => " "
            };
            print!("{}",v)
        }
        print!("\n")
    }
    println!("\nscore: {}", score(map));
    io::stdout().flush();
    thread::sleep(Duration::from_millis(100));
}

fn score(map: &HashMap<(i64,i64), i64>) -> i64{
    *map.get(&(-1_i64, 0_i64)).unwrap_or(&0)
}

fn calc_next_input(map: &HashMap<(i64,i64),i64>) -> i64 {
    let ball_x = map.iter()
        .filter(|((x,y),v)| **v == 4)
        .map(|((x,_),_)|x)
        .next().unwrap();
    let paddle_x = map.iter()
        .filter(|((x,y),v)| **v == 3)
        .map(|((x,_),_)|x)
        .next().unwrap();
    ball_x.cmp(paddle_x) as i64
}

fn pt1(mem: &Vec<i64>) -> usize  {
    IntCode::resolve(&vec![],&mem)
        .chunks(3)
        .filter(|c| c[2] == 2)
        .count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn regression() {
        let mem: Vec<i64> = read_lines(13)[0].split(',').map(|s| s.parse().unwrap()).collect();
        assert_eq!(pt1(&mem), 452);
        assert_eq!(pt2(&mem, false), 21415);
    }
}