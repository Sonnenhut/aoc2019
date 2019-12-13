use aoc2019::read_lines;
use aoc2019::intcode::IntCode;
use std::io;
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use aoc2019::intcode2::IntCode2;
use std::sync::mpsc::RecvTimeoutError::Disconnected;

fn main() {
    let mem: Vec<i64> = read_lines(13)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", pt1(&mem)); // 452
    println!("pt2: {}", pt2(&mem)); // 21415
}

fn pt2(mem: &Vec<i64>) -> i64 {
    let mut free_play = mem.to_vec();
    free_play[0] = 2;
    let (input, output) = IntCode2::run_async(&free_play);
    let mut map: HashMap<(i64,i64),i64> = HashMap::new();

    loop {
        let x_opt = output.recv_timeout(Duration::from_millis(1));
        if x_opt.is_err() { // no output data, get the next input for the intCode
            //input.send(read_input());
            if x_opt.err().unwrap() == Disconnected {
                println!("IntCode disconnected.");
                break;
            }
            let next = calc_next_input(&map);
            //println!("sending {}", next);
            input.send(next);
        } else {
            let (x,y,val) = (x_opt.unwrap(),output.recv().unwrap(),output.recv().unwrap());
            map.insert((x,y), val);
            //draw(&map);
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
    thread::sleep(Duration::from_millis(1000));
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

fn read_input() -> i64 {
    print!("Input next movement [l]eft/[r]ight/[s]tay (or a/s/d): ");
    io::stdout().flush();
    let mut wanted = HashMap::new();
    wanted.insert('a',-1);
    wanted.insert('s',0);
    wanted.insert('d',1);
    wanted.insert('l',-1);
    wanted.insert('s',0);
    wanted.insert('r',1);
    let mut character = [0];
    loop {
        io::stdin().read(&mut character).unwrap();
        let c = character[0] as char;
        if wanted.get(&c).is_some() {
            return wanted[&c]
        }
    }
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
    }
}