use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, LinkedList};
use std::convert::TryInto;
use std::fmt::DebugTuple;

use std::iter;
use std::ops::{RemAssign, Sub};
use std::path::Iter;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;

use aoc2019::intcode::IntCode;
use aoc2019::read_lines;


fn main() {
    let mem: Vec<i64> = read_lines(23)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", pt1(&mem)); // 17949
    //println!("pt2: {}", pt2(&instr)); //
}

fn pt1(mem: &Vec<i64>) -> i64 {
    let computers : Vec<Computer>= (0..50)
        .map(|network_id| IntCode::run_async(mem))
        .map(|(snd, rcv)| Computer{snd, rcv})
        .collect();

    computers.iter().enumerate().for_each(|(i,c)| c.snd.send(i as i64).unwrap());

    let mut broadcast_y = -1;
    while broadcast_y == -1 {
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
        computers.iter().enumerate().filter(|(i,c)| !accessed_addr.contains(&(*i as i64) ))
            .for_each(|(i, c)| {
                c.snd.send(-1).unwrap();
            });
    }

    broadcast_y
}

fn pt2(mem: &Vec<i64>) -> i64 {
    let computers : Vec<Computer>= (0..50)
        .map(|network_id| IntCode::run_async(mem))
        .map(|(snd, rcv)| Computer{snd, rcv})
        .collect();

    computers.iter().enumerate().for_each(|(i,c)| c.snd.send(i as i64).unwrap());

    let mut broadcast_y = -1;
    while broadcast_y == -1 {
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
        computers.iter().enumerate().filter(|(i,c)| !accessed_addr.contains(&(*i as i64) ))
            .for_each(|(i, c)| {
                c.snd.send(-1).unwrap();
            });
    }

    broadcast_y
}

struct Computer {
    snd : Sender<i64>,
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
