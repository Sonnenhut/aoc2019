use std::fs::File;
use std::io::{BufRead, BufReader};

pub mod intcode;

pub fn read_lines(day_no: u32) -> Vec<String>{
    let file_name = format!("inputs/day{:0>2}.txt", day_no);
    let reader = BufReader::new(File::open(file_name).unwrap());
    let lines : Vec<String> = reader.lines().collect::<Result<_, _>>().unwrap();
    lines
}