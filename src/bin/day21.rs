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
    println!("pt1: {}", pt1(&mem)); // 19362259
    println!("pt2: {}", pt2(&mem)); // 1141066762
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
    //print_dump(&out);
    *out.last().unwrap() as usize
}

fn pt2(mem: &Vec<i64>) -> usize {
    let mut input_str = // "transpiled" from minimal rust code below (to debug/ use some known lang)
r#"NOT A J
NOT B T
OR T J
NOT C T
OR T J
AND D J
NOT J T
NOT T T
AND H T
AND E J
OR T J
RUN
"#;
    let input : Vec<i64> =input_str.
        chars().map(|c| c as i64).collect();

    let out = IntCode::resolve(&input,mem);
    //print_dump(&out);
    *out.last().unwrap() as usize
}


fn print_dump(out: &Vec<i64>) {
    out.split(|s| *s == 10_i64)
        .map(|i_line| i_line.into_iter().map(|i| *i as u8 as char).collect())
        .for_each(|line: String| println!("{:?}",line));
}

// rust (subset) -> jumpcode
fn transpile(rust_code: String) -> String {
    let mut jump_code: String = rust_code.split("\n")
        .map(|line| line.trim())
        .filter(|line| line.len() > 0)
        .map(|line| line.split(";").next().unwrap())
        .flat_map(|line: &str| {
            let op = if line.starts_with("//") {
                None
            } else if line.contains("= !") {
                Some("NOT".to_string())
            } else if line.contains(" |= ") {
                Some("OR".to_string())
            } else if line.contains(" &= ") {
                Some("AND".to_string())
            } else { None };
            if let Some(op_present) = op {
                let mut_var = line.chars().next().unwrap().to_ascii_uppercase().to_string();
                let read_var = line.chars().last().unwrap().to_ascii_uppercase().to_string();
                Some(format!("{} {} {}", op_present, read_var, mut_var))
            } else { None }
        }).collect::<Vec<String>>().join("\n");
    jump_code.push_str("\nRUN\n");
    jump_code
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
    let mut j = false;
    let mut t = false;

    // only allowed operators:
    // = !
    // &=
    // |=

    j = !a; // is a hole?
    t = !b;
    j |= t; // a or b a hole?
    t = !c;
    j |= t; // a or b or c a hole?
    j &= d; // d is ground also?

    // 8 is also ground
    t = !j; // init false
    t = !t; // init true
    t &= h;

    // 5 is also ground
    j &= e;

    // combine above with OR
    j |= t;

    j
}

#[cfg(test)]
mod test {
    use super::*;
    use std::io;
    use std::io::Write;

    #[test]
    fn regression() {
        let mem = read_lines(21)[0].split(',').map(|s| s.parse().unwrap()).collect();
        assert_eq!(pt1(&mem), 19362259);
        assert_eq!(pt2(&mem), 1141066762);
    }
    #[test]
    fn by_hand() {
        let input = "###..#.##";
        assert_eq!(rust_code_pt2(conv("...######")), true);
        assert_eq!(rust_code_pt2(conv("##.######")), true);
        assert_eq!(rust_code_pt2(conv("###.#####")), false);
        assert_eq!(rust_code_pt2(conv("###..#.##")), false); // dont jump yet
        assert_eq!(rust_code_pt2(conv("##.#.##.#")), false); // don't jump, we cannot move on then...
        assert_eq!(rust_code_pt2(conv("...###...")), true); // cannot see the horizon, jump anyways
        assert_eq!(rust_code_pt2(conv(".#.##.#.#")), true);  // can see, would work
        assert_eq!(rust_code_pt2(conv("...##...#")), true); // same as above
        assert_eq!(rust_code_pt2(conv("#...#...#")), false);
        assert_eq!(rust_code_pt2(conv("###..#.##")), false)
    }

    fn conv(in_str : &str) -> Vec<bool>{
        return in_str.chars().map(|c| c == '#').collect()
    }

    #[test] // convert (rust) code to jumpcode
    fn run_as_jumpcode() {
        let mut rust_code: String =
r#"
    j = !a; // is a hole?
    t = !b;
    j |= t; // a or b a hole?
    t = !c;
    j |= t; // a or b or c a hole?
    j &= d; // d is ground also?

    //...###...
    //.#.##.#..
    //...##...#

    // 8 is also ground
    t = !j; // init false
    t = !t; // init true
    t &= h;

    // 5 && 9 are ground
    j &= e;
    //j &= i;

    // combine above with OR
    j |= t;

"#.parse().unwrap();
        let jump_code = transpile(rust_code);
        print!("---\n{}\n---\n", jump_code);
        io::stdout().flush();

        // run the jumpcode
        let input : Vec<i64> = jump_code
            .chars().map(|c| c as i64).collect();

        let mem : Vec<i64>= read_lines(21)[0].split(',').map(|s| s.parse().unwrap()).collect();
        let out = IntCode::resolve(&input,&mem);
        print_dump(&out);
        println!("last output: {}",*out.last().unwrap())
    }
}