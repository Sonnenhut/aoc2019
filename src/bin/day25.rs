use aoc2019::read_lines;
use aoc2019::intcode::{IntCode, IntCodeClient};
use std::io::{self, BufRead, Write};
use std::{thread, time};
use std::sync::mpsc::Receiver;



fn main() {
    let mem: Vec<i64> = read_lines(25)[0].split(',').map(|s| s.parse().unwrap()).collect();
    run(vec!["east\n", "take mug\n", "north\n", "take monolith\n", "south\n", "south\n", "west\n", "north\n", "south\n", "east\n", "north\n", "west\n", "east\n", "south\n", "west\n", "north\n", "west\n", "take bowl of rice\n", "east\n", "south\n", "east\n", "north\n", "west\n", "west\n", "take ornament\n", "east\n", "west\n", "west\n", "take astrolabe\n", "south\n", "take hologram\n", "west\n", "east\n", "north\n", "north\n", "take fuel cell\n", "south\n", "east\n", "south\n", "west\n", "east\n", "east\n", "take weather machine\n", "south\n", "north\n", "west\n", "north\n", "east\n", "east\n", "south\n", "west\n", "north\n", "west\n", "north\n", "west\n", "north\n", "north\n"],&mem);
}


fn run(input: Vec<&str>,mem: &Vec<i64>) {
    let client = IntCode::run_async(&mem);
    let mut history: Vec<String> = input.clone().iter().map(|s|String::from(*s)).collect();
    input.iter().for_each(|line| line.chars().for_each(|c| client.snd.send(c as i64).unwrap()));
    thread::sleep(time::Duration::from_secs(1));
    printChannel(&client.rcv);
    loop {
        println!("{:?}",history);
        thread::sleep(time::Duration::from_millis(100));
        client.rcv.try_iter()
            .map(|code| code as u8 as char)
            .for_each(|x| print!("{}", x as u8 as char));
        io::stdout().flush().unwrap();

        let stdin = io::stdin();
        let line = stdin.lock().lines().next().unwrap().unwrap();
        if "exit" == &*line {
            break;
        }
        history.push(line + "\n");
        history.last().unwrap().chars()
            .for_each(|c| client.snd.send(c as i64).unwrap())
    }

    println!("{:?}", history);
}

fn test_security(client: &IntCodeClient) {
    let all_items = vec!["bowl of rice", "monolith", "mug", "weather machine", "fuel cell", "astrolabe", "ornament", "hologram"];
    for n in 0..255 {
        //for
    }
}

fn get_bit_at(input: u32, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) as u32 != 0
    } else {
        false
    }
}
fn to_instructions(all_items: Vec<&str>, wanted_items: Vec<&str>) -> Vec<&str> {
    // drop all
    let drop_all = all_items.iter().map(|d| format!("drop {}\n")).collect();
    // take_wanted
    let take_wanted = wanted_items.iter().map(|d| format!("take {}\n")).collect();
    [drop_all, take_wanted].concat()
}

fn printChannel(rcv: &Receiver<i64>){
    rcv.try_iter()
        .map(|code| code as u8 as char)
        .for_each(|x| print!("{}", x as u8 as char));
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn regression() {
        let nums: Vec<Vec<String>> = read_lines(3).into_iter().map(|l| parse_wire(&l)).collect();
        //assert_eq!(pt1(&nums), 375);
        //assert_eq!(pt2(&nums), 14746);
    }
}