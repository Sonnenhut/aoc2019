use aoc2019::read_lines;
use std::cmp::{max, min};

fn main() {
    let nums: Vec<i32> = read_lines(5)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", IntCode::resolve(1, &nums)); // 16434972
    println!("pt2: {}", IntCode::resolve(5, &nums)); //
    //println!("pt2: {}", pt2(&nums)); // 2552
}

impl IntCode {
    fn create(input: i32, pgm: &Vec<i32>) -> IntCode{
        IntCode {input, csr: Some(0), pgm: pgm.clone(), output:vec![]}
    }
    fn resolve(input: i32, pgm: &Vec<i32>) -> i32 {
        IntCode::create(input, pgm).run()
    }
    fn csr_at(&mut self, new_csr: usize) {
        self.csr = Some(new_csr);
    }
    fn run(&mut self) -> i32{
        let scope = &mut self.pgm;
        while self.csr.is_some() {
            self.run_single();
        }
        *self.output.last().unwrap_or(&0)
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
        } else if op == 5 { // jump-if-true
            let p1 = resolve_param(param_modes[0], scope[csr + 1], &scope);
            let p2 = resolve_param(param_modes[1], scope[csr + 2], &scope);
            self.csr = if p1 != 0 { Some(p2 as usize) } else { Some(csr + 3) }
        } else if op == 6 { // jump-if-false
            let p1 = resolve_param(param_modes[0], scope[csr + 1], &scope);
            let p2 = resolve_param(param_modes[1], scope[csr + 2], &scope);
            self.csr = if p1 == 0 { Some(p2 as usize) } else { Some(csr + 3) }
        } else if op == 7 { // less-than
            let p1 = resolve_param(param_modes[0], scope[csr + 1], &scope);
            let p2 = resolve_param(param_modes[1], scope[csr + 2], &scope);
            let out_csr = scope[csr + 3] as usize;
            scope[out_csr] = (p1 < p2) as i32;
            self.csr = Some(csr + 4)
        } else if op == 8 { // less-than
            let p1 = resolve_param(param_modes[0], scope[csr + 1], &scope);
            let p2 = resolve_param(param_modes[1], scope[csr + 2], &scope);
            let out_csr = scope[csr + 3] as usize;
            scope[out_csr] = (p1 == p2) as i32;
            self.csr = Some(csr + 4)
        } else if op == 99{
            println!("halt at csr: {}", csr);
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
        let mut toTest = IntCode::create(0,&vec![1,0,0,0,99]);
        toTest.run_single();
        assert_eq!(toTest.csr, Some(4));
        assert_eq!(toTest.pgm, vec![2,0,0,0,99]);
        toTest.csr_at(0);
        toTest.run_single();
        assert_eq!(toTest.csr, Some(4));
        assert_eq!(toTest.pgm, vec![4,0,0,0,99]);

        toTest = IntCode::create(0,&vec![1002,4,3,4,33]);
        toTest.run_single();
        assert_eq!(toTest.csr, Some(4));
        assert_eq!(toTest.pgm, vec![1002,4,3,4,99]);

        toTest = IntCode::create(0,&vec![1101,100,-1,4,0]);
        toTest.run_single();
        assert_eq!(toTest.csr, Some(4));
        assert_eq!(toTest.pgm, vec![1101,100,-1,4,99]);
    }

    #[test]
    fn test_int_code() {
        let mut toTest = IntCode::create(0,&vec![1101,100,-1,4,0]);
        toTest.run();
        assert_eq!(toTest.pgm, vec![1101,100,-1,4,99]);

        toTest = IntCode::create(0,&vec![1,1,1,4,99,5,6,0,99]);
        toTest.run();
        assert_eq!(toTest.pgm, vec![30,1,1,4,2,5,6,0,99]);


        toTest = IntCode::create(0,&vec![2,4,4,5,99,0]);
        toTest.run();
        assert_eq!(toTest.pgm, vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn fix_nonzero_code() {
        let mut toTest = IntCode::create(1, &vec![3, 15, 1, 15, 6, 6, 1100, 1, 238, 15, 104, 0, 1101, 40, 0, /*15*/0]);
        toTest.run_single();
        assert_eq!(toTest.pgm, vec![3,15,1,15,6,6,1100,1,238,15,104,0,1101,40,0,/*15*/1]);
        toTest.run_single();
        assert_eq!(toTest.pgm, vec![3,15,1,15,6,6,1101,1,238,15,104,0,1101,40,0,/*15*/1]);
        toTest.run_single();
        assert_eq!(toTest.pgm, vec![3,15,1,15,6,6,1101,1,238,15,104,0,1101,40,0,/*15*/239]);
        toTest.run_single();
        assert_eq!(toTest.pgm, vec![3,15,1,15,6,6,1101,1,238,15,104,0,1101,40,0,/*15*/239]);
    }

    #[test]
    fn test_pgm_pt2() {
        assert_eq!(IntCode::resolve(8, &vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]), 1); // is eq 8
        // lt 8
        let lt8_position_mode = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(IntCode::resolve(1, &lt8_position_mode), 1);
        assert_eq!(IntCode::resolve(2, &lt8_position_mode), 1);
        assert_eq!(IntCode::resolve(3, &lt8_position_mode), 1);
        assert_eq!(IntCode::resolve(4, &lt8_position_mode), 1);
        assert_eq!(IntCode::resolve(5, &lt8_position_mode), 1);
        assert_eq!(IntCode::resolve(6, &lt8_position_mode), 1);
        assert_eq!(IntCode::resolve(7, &lt8_position_mode), 1);

        // eq 8
        let eq8_position_mode = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(IntCode::resolve(1, &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve(2, &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve(3, &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve(4, &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve(5, &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve(6, &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve(7, &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve(8, &eq8_position_mode), 1);

        // lt 8
        let lt8_immediate_mode = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(IntCode::resolve(1, &lt8_immediate_mode), 1);
        assert_eq!(IntCode::resolve(2, &lt8_immediate_mode), 1);
        assert_eq!(IntCode::resolve(3, &lt8_immediate_mode), 1);
        assert_eq!(IntCode::resolve(4, &lt8_immediate_mode), 1);
        assert_eq!(IntCode::resolve(5, &lt8_immediate_mode), 1);
        assert_eq!(IntCode::resolve(6, &lt8_immediate_mode), 1);
        assert_eq!(IntCode::resolve(7, &lt8_immediate_mode), 1);


        // non_zero
        let non_zero_position_mode = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        assert_eq!(IntCode::resolve(1, &non_zero_position_mode), 1);
        assert_eq!(IntCode::resolve(-1, &non_zero_position_mode), 1);
        assert_eq!(IntCode::resolve(0, &non_zero_position_mode), 0);

        // non_zero
        let non_zero_immediate_mode = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        assert_eq!(IntCode::resolve(1, &non_zero_immediate_mode), 1);
        assert_eq!(IntCode::resolve(-1, &non_zero_immediate_mode), 1);
        assert_eq!(IntCode::resolve(0, &non_zero_immediate_mode), 0);

        //larger pgm
        let large_pgm = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                             1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                             999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        assert_eq!(IntCode::resolve(6, &large_pgm), 999); // below 8
        assert_eq!(IntCode::resolve(6, &large_pgm), 999);
        assert_eq!(IntCode::resolve(7, &large_pgm), 999);
        assert_eq!(IntCode::resolve(8, &large_pgm), 1000); // is 8
        assert_eq!(IntCode::resolve(9, &large_pgm), 1001);
        assert_eq!(IntCode::resolve(10, &large_pgm), 1001);
        assert_eq!(IntCode::resolve(11, &large_pgm), 1001);
    }
}