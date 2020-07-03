use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet, LinkedList};
use std::convert::TryInto;
use std::fmt::DebugTuple;

use std::iter;
use std::ops::{RemAssign, Sub};
use std::path::Iter;
use std::sync::mpsc::{Receiver, Sender, SyncSender};
use std::time::Duration;

use aoc2019::intcode::{IntCode, IntCodeClient};
use aoc2019::read_lines;
use std::thread::Thread;


fn main() {
    let mem: Vec<i64> = read_lines(23)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", pt1(&mem)); // 17949
    println!("pt2: {}", pt2(&mem)); // 12326
}

fn pt1(mem: &Vec<i64>) -> i64 {
    let computers : Vec<IntCodeClient>= (0..50)
        .map(|network_id| IntCode::run_async(mem))
        .collect();

    computers.iter().enumerate().for_each(|(i,c)| c.snd.send(i as i64).unwrap());

    let mut broadcast_y = -1;
    while broadcast_y == -1 {
        let mut accessed_addr = vec![];
        computers.iter().enumerate().for_each(|(i, c)| {
            match c.rcv.try_recv().ok() {
                Some(dest_addr) => {
                    let x = c.rcv.recv_timeout(Duration::from_millis(100)).ok().unwrap();
                    let y = c.rcv.recv_timeout(Duration::from_millis(100)).ok().unwrap();
                    if dest_addr == 255 {
                        broadcast_y = y;
                    } else {
                        computers[dest_addr as usize].snd.send(x);
                        computers[dest_addr as usize].snd.send(y);
                        accessed_addr.push(dest_addr);
                    }
                }
                _ => {}
            }
        });

        // snd -1 to everyone that is idling
        computers.iter()
            .filter(|c| *c.idle.lock().unwrap())
            .for_each(|c|  c.snd.send(-1).unwrap());
    }

    broadcast_y
}

fn pt2(mem: &Vec<i64>) -> i64 {
    let computers : Vec<IntCodeClient>= (0..50)
        .map(|network_id| IntCode::run_async(mem))
        .collect();

    computers.iter().enumerate().for_each(|(i,c)| c.snd.send(i as i64).unwrap());

    let mut last_nat_delivery = None;
    let mut nat_payload : Option<(i64,i64)> = None;
    let mut nat_looped = false;
    let mut consecutive_idle = 0;
    while !nat_looped {
        computers.iter().enumerate().for_each(|(i, c)| {
            match c.rcv.try_recv().ok() {
                Some(dest_addr) => {
                    let x = c.rcv.recv_timeout(Duration::from_millis(100)).ok().unwrap();
                    let y = c.rcv.recv_timeout(Duration::from_millis(100)).ok().unwrap();
                    if dest_addr == 255 {
                        nat_payload = Some((x,y));
                    } else {
                        computers[dest_addr as usize].snd.send(x);
                        computers[dest_addr as usize].snd.send(y);
                    }
                }
                _ => {}
            }
        });
        let all_idle = computers.iter().all(|c| *c.idle.lock().unwrap());
        let broadcast_msg = if consecutive_idle >= 2 { nat_payload } else { None };
        if let Some((x,y)) = broadcast_msg {
            computers[0].snd.send(x);
            computers[0].snd.send(y);
            consecutive_idle = 0;
            if last_nat_delivery.is_some() && last_nat_delivery.unwrap() == y {
                nat_looped = true;
            } else {
                last_nat_delivery = Some(y) // sent to #0
            }
        } else {
            consecutive_idle = if all_idle { consecutive_idle + 1} else { 0 };

            // snd -1 to everyone that is idling
            computers.iter()
                .filter(|c| *c.idle.lock().unwrap())
                .for_each(|c|  c.snd.send(-1).unwrap());
        };
    }

    last_nat_delivery.unwrap()
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
        let mem: Vec<i64> = read_lines(23)[0].split(',').map(|s| s.parse().unwrap()).collect();
        assert_eq!(pt1(&mem), 17949);
        assert_eq!(pt2(&mem), 12326);
    }

    #[test]
    fn test_modulo() {
        assert_eq!(1, 1);
    }
}
