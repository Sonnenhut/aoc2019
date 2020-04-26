use aoc2019::read_lines;
use aoc2019::intcode::IntCode;
use std::collections::{HashMap, HashSet, BinaryHeap};
use std::cmp::Ordering;
use std::iter::{Map, repeat};
use std::path::Iter;
use std::convert::TryInto;

use std::iter;

fn main() {
    let mem = read_lines(21)[0].split(',').map(|s| s.parse().unwrap()).collect();
    //println!("pt1: {}", pt1(&mem)); // 19362259
    println!("pt2: {}", pt2(&mem)); //
}

fn pt1(mem: &Vec<i64>) -> usize {
    let input : Vec<i64> =
        r#"NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
WALK
"#.chars().map(|c| c as i64).collect();

    let out = IntCode::resolve(&input,mem);
    print_dump(&out);
    *out.last().unwrap() as usize
}

fn pt2(mem: &Vec<i64>) -> usize {
    let mut input_str =
r#"NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
RUN
"#;
    let input : Vec<i64> =input_str.
        chars().map(|c| c as i64).collect();

    let out = IntCode::resolve(&input,mem);
    print_dump(&out);
    *out.last().unwrap() as usize
}

/*
if in 4 tiles is ground
 and in 8 tiles is ground
 => JMP
// is there a hole in the next 3 tiles?
NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J

// and can will I be able to jump another time?!
AND H J




RUN
--- xxx ---

// if there is a hole I have to jump over and I land on ground? -> J = true
NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J

// 8 is also ground
NOT J T <- init false
NOT J T <- init true (if above is true)
AND H T

// 5 && 9 are ground
AND E J
AND I J

// combine above with OR
OR T J





*/


fn print_dump(out: &Vec<i64>) {
    out.split(|s| *s == 10_i64)
        .map(|i_line| i_line.into_iter().map(|i| *i as u8 as char).collect())
        .for_each(|line: String| println!("{:?}",line));
}

fn rust_code_pt2(sight: Vec<bool>) -> bool {
    let a = sight[0];
    let b = sight[1];
    let c = sight[2];
    let d = sight[3];
    let e = sight[4];
    let f = sight[5];
    let g = sight[6];
    let h = sight[7];
    let i = sight[8];
    println!("{:?}", sight);
    let mut j = false;
    let mut t = false;
    //println!("a hole ? {:?}", j);
    // = !
    // &=
    // |=


    j = !a; // is a hole?
    t = !b;
    j |= t; // a or b a hole?
    t = !c;
    j |= t; // a or b or c a hole?
    j &= d; // d is ground also?

    j
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io;
    use std::io::Write;

    #[test]
    fn regression() {
        //let mem: Vec<String> = read_lines(20);
        //assert_eq!(pt1(&mem), 498);
        //assert_eq!(pt2(&mem), 5564);
    }
    #[test]
    fn by_hand() {
        let input = "###..#.##";
        assert_eq!(rust_code_pt2(conv("...######")), true);
        assert_eq!(rust_code_pt2(conv("##.######")), true);
        assert_eq!(rust_code_pt2(conv("###.#####")), false);
        assert_eq!(rust_code_pt2(conv("###..#.##")), false); // dont jump yet
        assert_eq!(rust_code_pt2(conv("##.#.#.##")), false); // we could, but would not be able to continue...
        //assert_eq!(rust_code_pt2(conv("#...#...#")), false);
        //assert_eq!(rust_code_pt2(conv("###..#.##")), false)
    }

    fn conv(in_str : &str) -> Vec<bool>{
        return in_str.chars().map(|c| c == '#').collect()
    }

    #[test] // convert (rust) code to jumpcode
    fn run_as_jumpcode() {
        let mut rust_code: String =
r#"j = !a; // is a hole?
    t = !b;
    j |= t; // a or b a hole?
    t = !c;
    j |= t; // a or b or c a hole?
    j &= d; // d is ground also?
"#.parse().unwrap();

        let r_lines : String = rust_code.split("\n")
            .map(|line| line.trim())
            .filter(|line| line.len() > 0)
            .map(|line| line.split(";").next().unwrap())
            .map(|line: &str| {
                let op = if line.contains("= !") {
                    "NOT".to_string()
                } else if line.contains(" |= ") {
                    "OR".to_string()
                } else if line.contains(" &= ") {
                    "AND".to_string()
                } else { "".to_string() };
                let mut_var = line.chars().next().unwrap().to_ascii_uppercase().to_string();
                let read_var = line.chars().last().unwrap().to_ascii_uppercase().to_string();
                //res.push_str(&mut_var);
                format!("{} {} {}", op, read_var, mut_var)
            }).collect::<Vec<String>>().join("\n");
        print!("{}\nRUN\n", r_lines);
        io::stdout().flush();


        let input : Vec<i64> =r_lines.
            chars().map(|c| c as i64).collect();

        let mem : Vec<i64>= read_lines(21)[0].split(',').map(|s| s.parse().unwrap()).collect();
        /*let out = IntCode::resolve(&input,&mem);
        print_dump(&out);
        println!("last out: {}",*out.last().unwrap())
        */
    }


}