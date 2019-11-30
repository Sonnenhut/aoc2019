use std::fs::read_to_string;
// aoc 2017 day 1
fn main() {
    let input = read_to_string("inputs/day00.txt").unwrap();
    println!("pt1 1253: {}", pt1(input.as_ref()));
    println!("pt2 1278: {}", pt2(input.as_ref()));
}

fn pt1(st: &str) -> u32 {
    let seq = format!("{}{}", st, st.chars().next().unwrap());
    let mut res = 0;
    let mut last = 999;
    for c in seq.chars() {
        let curr = c.to_digit(10).unwrap();
        if last == curr {
            res += curr;
        }
        last = curr;
    }
    res
}

fn pt2(st: &str) -> u32 {
    let view : Vec<u32> = format!("{}{}", st, st).chars()
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    let mut res = 0;
    for i in 0..st.len() {
        let curr = view[i];
        let halfway = view[i + (st.len() / 2)];
        if halfway == curr {
            res += curr;
        }
    }
    res
}

#[cfg(test)]
mod test_pt2 {
    use super::*;
    #[test]
    fn ex1() {
        assert_eq!(pt2("1212"), 6)
    }
    #[test]
    fn ex2() {
        assert_eq!(pt2("1221"), 0)
    }
    #[test]
    fn ex3() {
        assert_eq!(pt2("123425"),4 )
    }
    #[test]
    fn ex4() {
        assert_eq!(pt2("123123"),12 )
    }
    #[test]
    fn ex5() {
        assert_eq!(pt2("12131415"),4 )
    }
}

#[cfg(test)]
mod test_pt1 {
    use super::*;

    #[test]
    fn ex1() {
        assert_eq!(pt1("1122"), 3)
    }
    #[test]
    fn ex2() {
        assert_eq!(pt1("1111"),4)
    }
    #[test]
    fn ex3() {
        assert_eq!(pt1("1234"),0)
    }
    #[test]
    fn ex4() {
        assert_eq!(pt1("91212129"),9)
    }
}