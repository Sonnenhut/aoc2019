use aoc2019::read_lines;
use std::cmp::{max, min};

fn main() {
    let nums: Vec<i32> = read_lines(5)[0].split(',').map(|s| s.parse().unwrap()).collect();
    let mut intcode = IntCode::create(1,nums);
    intcode.run();
    println!("pt1: {}", intcode.output.last().unwrap()); // 16434972
    //println!("pt2: {}", pt2(&nums)); // 2552
}

impl IntCode {
    fn create(input: i32, pgm: Vec<i32>) -> IntCode{
        IntCode {input, csr: Some(0),pgm, output:vec![]}
    }
    fn csr_at(&mut self, new_csr: usize) {
        self.csr = Some(new_csr);
    }
    fn run(&mut self) {
        let scope = &mut self.pgm;
        while self.csr.is_some() {
            self.run_single();
        }
    }
    fn run_single(&mut self) {
        let csr = self.csr.unwrap();
        let scope = &mut self.pgm;
        let (op, param_modes) = parse_op(scope[csr] as u32);
        let curr_view:Vec<i32> = scope.clone().into_iter().skip(csr).take(min(4, scope.len())).collect();
        println!("{:?}", curr_view);
        if op == 1 { // add
            let p1 = resolve_param(param_modes[0], scope[csr + 1], &scope);
            let p2 = resolve_param(param_modes[1], scope[csr + 2], &scope);
            let out_csr = scope[csr + 3] as usize;
            scope[out_csr] = p1 + p2;
            self.csr = Some(csr + 4);
        } else if op == 2 { // mul
            let p1 = resolve_param(param_modes[0], scope[csr + 1], &scope);
            let p2 = resolve_param(param_modes[1], scope[csr + 2], &scope);
            let out_csr = scope[csr + 3] as usize;
            scope[out_csr] = p1 * p2;
            self.csr = Some(csr + 4)
        } else if op == 3 { // read_in
            let out_csr = scope[csr + 1] as usize;
            scope[out_csr] = self.input as i32;
            self.csr = Some(csr + 2)
        } else if op == 4 { //write_out
            let out = resolve_param(param_modes[0], scope[csr + 1], &scope);
            self.output.push(out);
            println!("-> {}",out);
            self.csr = Some(csr + 2)
        } else if op == 99{
            println!("hat at csr: {}", csr);
            self.csr = None;
        } else {
            panic!("problem!"); // or is it?! exiting after an output is ok (via instructions)
        }
    }
}

struct IntCode {
    input: i32,
    output: Vec<i32>,
    csr: Option<usize>,
    pgm: Vec<i32>,
}

fn parse_op(input: u32) -> (u32, Vec<u32>) {
    let s : String = format!("{:05}", input);
    let op : u32 = s[3..=4].parse().unwrap();
    let param_modes = s[0..=2].chars().rev().map(|x| x.to_digit(10).unwrap()).collect();
    (op, param_modes)
}

fn resolve_param(mode : u32, param: i32, scope: &Vec<i32>) -> i32 {
    if mode == 0 {
        scope[param as usize]
    } else if mode == 1 {
        param
    } else {
        panic!("ParamMode not defined!")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex() {
        assert_eq!(parse_op(1002), (2,vec![0,1,0]));
        assert_eq!(parse_op(1102), (2, vec![1,1,0]));
        assert_eq!(parse_op(10002), (2, vec![0,0,1]));
        assert_eq!(resolve_param(0,0, &vec![99]), 99);
        assert_eq!(resolve_param(1,0, &vec![99]), 0);
    }
    #[test]
    fn test_single_instr() {
        let mut scope = vec![1,0,0,0,99];
        let mut toTest = IntCode::create(0,vec![1,0,0,0,99]);
        toTest.run_single();
        assert_eq!(toTest.csr, Some(4));
        assert_eq!(toTest.pgm, vec![2,0,0,0,99]);
        toTest.csr_at(0);
        toTest.run_single();
        assert_eq!(toTest.csr, Some(4));
        assert_eq!(toTest.pgm, vec![4,0,0,0,99]);

        toTest = IntCode::create(0,vec![1002,4,3,4,33]);
        toTest.run_single();
        assert_eq!(toTest.csr, Some(4));
        assert_eq!(toTest.pgm, vec![1002,4,3,4,99]);

        toTest = IntCode::create(0,vec![1101,100,-1,4,0]);
        toTest.run_single();
        assert_eq!(toTest.csr, Some(4));
        assert_eq!(toTest.pgm, vec![1101,100,-1,4,99]);
    }

    #[test]
    fn test_int_code() {
        let mut toTest = IntCode::create(0,vec![1101,100,-1,4,0]);
        toTest.run();
        assert_eq!(toTest.pgm, vec![1101,100,-1,4,99]);

        toTest = IntCode::create(0,vec![1,1,1,4,99,5,6,0,99]);
        toTest.run();
        assert_eq!(toTest.pgm, vec![30,1,1,4,2,5,6,0,99]);


        toTest = IntCode::create(0,vec![2,4,4,5,99,0]);
        toTest.run();
        assert_eq!(toTest.pgm, vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn fix_nonzero_code() {
        let mut toTest = IntCode::create(1, vec![3,15,1,15,6,6,1100,1,238,15,104,0,1101,40,0,/*15*/0]);
        toTest.run_single();
        assert_eq!(toTest.pgm, vec![3,15,1,15,6,6,1100,1,238,15,104,0,1101,40,0,/*15*/1]);
        toTest.run_single();
        assert_eq!(toTest.pgm, vec![3,15,1,15,6,6,1101,1,238,15,104,0,1101,40,0,/*15*/1]);
        toTest.run_single();
        assert_eq!(toTest.pgm, vec![3,15,1,15,6,6,1101,1,238,15,104,0,1101,40,0,/*15*/239]);
        toTest.run_single();
        assert_eq!(toTest.pgm, vec![3,15,1,15,6,6,1101,1,238,15,104,0,1101,40,0,/*15*/239]);
    }
}