use aoc2019::read_lines;
use aoc2019::intcode::{IntCode, IntCodeClient};
use std::io::{self, BufRead, Write};
use std::{thread, time};
use std::sync::mpsc::Receiver;

fn main() {
    let mem: Vec<i64> = read_lines(25)[0].split(',').map(|s| s.parse().unwrap()).collect();
    run(vec!["east\n", "take mug\n", "north\n", "take monolith\n", "south\n", "south\n", "west\n", "north\n", "south\n", "east\n", "north\n", "west\n", "east\n", "south\n", "west\n", "north\n", "west\n", "take bowl of rice\n", "east\n", "south\n", "east\n", "north\n", "west\n", "west\n", "take ornament\n", "east\n", "west\n", "west\n", "take astrolabe\n", "south\n", "take hologram\n", "west\n", "east\n", "north\n", "north\n", "take fuel cell\n", "south\n", "east\n", "south\n", "west\n", "east\n", "east\n", "take weather machine\n", "south\n", "north\n", "west\n", "north\n", "east\n", "east\n", "south\n", "west\n", "north\n", "west\n", "north\n", "west\n", "north\n", "north\n"],
        &mem,
        true);
    //result 1073874948
}


fn run(input: Vec<&str>,mem: &Vec<i64>, with_brute_force_security: bool) {
    let client = IntCode::run_async(&mem);
    // replay
    let mut history: Vec<String> = input.clone().iter().map(|s|String::from(*s)).collect();
    println!("Replaying commands: {:?}",history);
    input.iter().for_each(|line| line.chars().for_each(|c| client.snd.send(c as i64).unwrap()));
    thread::sleep(time::Duration::from_secs(1));
    printChannel(&client.rcv);
    thread::sleep(time::Duration::from_millis(100));

    if with_brute_force_security {
        // find the way to the security checkpoint
        let available_items = vec!["bowl of rice", "monolith", "mug", "weather machine", "fuel cell", "astrolabe", "ornament", "hologram"].iter().map(|s|String::from(*s)).collect();
        brute_force_security(&client, &available_items);
    } else {
        loop {
            client.rcv.try_iter()
                .map(|code| code as u8 as char)
                .for_each(|x| print!("{}", x as u8 as char));
            io::stdout().flush().unwrap();

            if input.is_empty() {
                // manually find the items and the way to the security checkpoint (let humans have fun too!!)
                let stdin = io::stdin();
                let line = stdin.lock().lines().next().unwrap().unwrap();
                if "exit" == &*line {
                    break;
                }
                history.push(line + "\n");
                history.last().unwrap().chars()
                    .for_each(|c| client.snd.send(c as i64).unwrap())
            }
        }
    }

    println!("{:?}", history);
}

fn brute_force_security(client: &IntCodeClient, all_items: &Vec<String>) {
    for n in 0..255 {
        let items : Vec<String> = (0..all_items.len())
            .filter_map(|idx| if get_bit_at(n, idx as u8) { all_items.get(idx) } else { None })
            .map(|s| String::from(s))
            .collect();
        let instructions = to_instructions(&all_items, &items, String::from("north\n"));

        println!("Testing {:?}", instructions);
        instructions.join("").chars().for_each(|c| client.snd.send(c as i64).unwrap());

        thread::sleep(time::Duration::from_millis(100));
        let output :String = client.rcv.try_iter()
            .map(|code| code as u8 as char).collect();
        println!("out from test: {}", output);
        if !output.contains("Security Check") {
            println!("DING DING DING, correct combination is:");
            println!("{:?}",instructions);
            break;
        }
    }
}

fn get_bit_at(input: u32, n: u8) -> bool {
    if n < 32 {
        input & (1 << n) as u32 != 0
    } else {
        false
    }
}
fn to_instructions(all_items: &Vec<String>, wanted_items: &Vec<String>, postfix: String) -> Vec<String> {
    // drop all
    let drop_all :Vec<String>= all_items.iter().map(|d| format!("drop {}\n",d)).collect();
    // take_wanted
    let take_wanted :Vec<String>= wanted_items.iter().map(|d| format!("take {}\n",d)).collect();
    [drop_all, take_wanted, vec![postfix]].concat()
}

fn printChannel(rcv: &Receiver<i64>){
    rcv.try_iter()
        .map(|code| code as u8 as char)
        .for_each(|x| print!("{}", x as u8 as char));
}

