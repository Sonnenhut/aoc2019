use std::collections::HashMap;
use std::io;
use std::io::Write;
use std::thread;

use aoc2019::intcode::{IntCode, IntCodeClient};
use aoc2019::read_lines;
use std::time::Duration;

fn main() {
    let mem: Vec<i64> = read_lines(13)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", pt1(&mem)); // 452
    println!("pt2: {}", pt2(&mem, true)); // 21415
}

fn pt2(mem: &Vec<i64>, print: bool) -> i64 {
    let mut free_play = mem.to_vec();
    free_play[0] = 2;
    let IntCodeClient {snd, rcv, idle: _ } = IntCode::run_async(&free_play);
    let mut map: HashMap<(i64,i64),i64> = HashMap::new();

    let mut ball_updated = false;
    let mut paddle_updated = false;
    loop {
        let x_opt = rcv.recv();
        if x_opt.is_err() {
            println!("IntCode disconnected.");
            break;
        } else {
            let (x, y, val) = (x_opt.unwrap(), rcv.recv().unwrap(), rcv.recv().unwrap());
            map.insert((x, y), val);

            ball_updated |= val == 4;
            paddle_updated |= val == 3;
            if ball_updated && paddle_updated {
                let res  = pos_x(4,&map).cmp(&pos_x(3,&map)) as i64;
                let _ = snd.send(res);
                paddle_updated = res == 0; // paddle not changed, thus don't expect an update
                ball_updated = false;

                if print {draw(&map)};
            }
        }
    }
    score(&map)
}

fn draw(map: &HashMap<(i64,i64), i64>) {
    let max_x = *map.keys().map(|(x,_)|x).max().unwrap_or(&0);
    let max_y = *map.keys().map(|(_,y)|y).max().unwrap_or(&0);
    for y in 0..=max_y {
        for x in 0..=max_x {
            let v = match map.get(&(x as i64,y as i64)) {
                Some(0) => " ", // no
                Some(1) => "█", // wall
                Some(2) => "X", // block
                Some(3) => "=", // paddle
                Some(4) => "*", //ball
                _ => " "
            };
            print!("{}",v)
        }
        print!("\n")
    }
    println!("\nscore: {}", score(map));
    let _ = io::stdout().flush();
    thread::sleep(Duration::from_millis(50));
}

fn score(map: &HashMap<(i64,i64), i64>) -> i64{
    *map.get(&(-1_i64, 0_i64)).unwrap_or(&0)
}

fn pos_x(val: i64, map: &HashMap<(i64,i64),i64>) -> i64 {
    map.iter()
        .filter(|((_,_),v)| **v == val)
        .map(|((x,_),_)|*x)
        .next().unwrap()
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
