use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, LinkedList};
use std::convert::TryInto;
use std::fmt::DebugTuple;

use std::iter;
use std::ops::{RemAssign, Sub};
use std::path::Iter;
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use std::time::Duration;

use aoc2019::intcode::IntCode;
use aoc2019::read_lines;
use std::thread::Thread;


fn main() {
    let mem: Vec<i64> = read_lines(23)[0].split(',').map(|s| s.parse().unwrap()).collect();
    //println!("pt1: {}", pt1(&mem)); // 17949
    println!("pt2: {}", pt2(&mem)); // 12410 too high
}

fn pt1(mem: &Vec<i64>) -> i64 {
    let computers : Vec<Computer>= (0..50)
        .map(|network_id| IntCode::run_sync(mem))
        .map(|(snd, rcv)| Computer{snd, rcv})
        .collect();

    computers.iter().enumerate().for_each(|(i,c)| c.snd.send(i as i64).unwrap());

    let mut broadcast_y = -1;
    let mut loop_cnt = 0;
    while broadcast_y == -1 {
        println!("loop # {:?}", loop_cnt);
        let mut accessed_addr = vec![];
        computers.iter().enumerate().for_each(|(i, c)| {
            match c.rcv.try_recv().ok() {
                Some(dest_addr) => {
                    println!("#{:?} sending to {:?}", i, dest_addr);
                    let x = c.rcv.recv_timeout(Duration::from_millis(100)).ok().unwrap();
                    let y = c.rcv.recv_timeout(Duration::from_millis(100)).ok().unwrap();
                    if dest_addr == 255 {
                        broadcast_y = y;
                    } else {
                        println!("X={:?}", x);
                        println!("Y={:?}", y);
                        computers[dest_addr as usize].snd.send(x);
                        computers[dest_addr as usize].snd.send(y);
                        accessed_addr.push(dest_addr);
                    }
                }
                _ => {}
            }
        });
        // snd -1
        computers.iter()
            .enumerate()
            //.filter(|(i,c)| !accessed_addr.contains(&(*i as i64) ))
            .for_each(|(i, c)| {
                c.snd.send(-1).unwrap();
            });
        loop_cnt += 1;
    }

    broadcast_y
}

fn pt2(mem: &Vec<i64>) -> i64 {
    let computers : Vec<Computer>= (0..50)
        .map(|network_id| IntCode::run_sync(mem))
        .map(|(snd, rcv)| Computer{snd, rcv})
        .collect();

    computers.iter().enumerate().for_each(|(i,c)| c.snd.send(i as i64).unwrap());

    let mut nat_y_values = vec![];
    let mut nat_payload : Option<(i64,i64)> = None;
    let mut nat_looped = false;
    let mut consecutive_idle = 0;
    let mut loop_cnt = 0;
    while !nat_looped {
        println!("loop {:?}", loop_cnt);
        let mut accessed_addr = vec![];
        computers.iter().enumerate().for_each(|(i, c)| {
            match c.rcv.try_recv().ok() {
                Some(dest_addr) => {
                    println!("#{:?} sending to {:?}", i, dest_addr);
                    let x = c.rcv.recv_timeout(Duration::from_millis(100)).ok().unwrap();
                    let y = c.rcv.recv_timeout(Duration::from_millis(100)).ok().unwrap();
                    println!("X={:?}", x);
                    println!("Y={:?}", y);
                    if dest_addr == 255 {
                        if nat_y_values.contains(&y) {
                            println!("NAT looped with value: {:?}", y);
                            nat_looped = true;
                        }
                        nat_y_values.push(y);
                        nat_payload = Some((x,y));
                    } else {
                        computers[dest_addr as usize].snd.send(x);
                        computers[dest_addr as usize].snd.send(y);
                        accessed_addr.push(dest_addr);
                    }
                }
                _ => {}
            }
        });
        let broadcast_msg = if accessed_addr.is_empty() && consecutive_idle >= 2 { nat_payload } else { None };
        if let Some((x,y)) = broadcast_msg {
            println!("idle... nat is jumping in with {:?}",(x,y));
            computers[0].snd.send(x);
            computers[0].snd.send(y);
            consecutive_idle = 0;
        } else {
            consecutive_idle = if accessed_addr.is_empty() { consecutive_idle + 1} else { 0 };
            // snd -1 to everyone that did not receive anything
            computers.iter().enumerate()
                .filter(|(i, c)| !accessed_addr.contains(&(*i as i64)))
                .for_each(|(i, c)| {
                    c.snd.send(-1).unwrap();
                });
        };

        loop_cnt += 1;
        //12384 // nosleep; idle = 2000
        //std::thread::sleep(Duration::from_millis(500));
    }

    *nat_y_values.last().unwrap()
}

struct Computer {
    snd : SyncSender<i64>,
    rcv : Receiver<i64>
}

#[cfg(test)]
mod test {
    use std::io;
    use std::io::Write;

    use super::*;

    #[test]
    fn regression() {
    }

    #[test]
    fn test_modulo() {
        assert_eq!(1, 1);
    }
}
