use aoc2019::read_lines;
use std::cmp::{max, min};

fn main() {
    let nums: Vec<i32> = read_lines(5)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: see below", ); // 16434972
    run_intcode(1,nums);
    //println!("pt2: {}", pt2(&nums)); // 2552
}

fn run_intcode(input: u32, series: Vec<i32>) -> Vec<i32>{
    let mut scope = series.clone();
    let mut csr = Some(0);
    while csr.is_some() {
        csr = exec_instr(input, csr.unwrap(), &mut scope);
    }
    scope.clone()
}

fn exec_instr(input: u32, csr: usize, scope: &mut Vec<i32>) -> Option<usize> {
    let (op, param_modes) = parse_op(scope[csr] as u32);
    let curr_view:Vec<i32> = scope.clone().into_iter().skip(csr).take(min(4, scope.len())).collect();
    println!("{:?}", curr_view);
    if op == 1 {
        let p1 = resolve_param(param_modes[0], scope[csr + 1], &scope);
        let p2 = resolve_param(param_modes[1], scope[csr + 2], &scope);
        let out_csr = scope[csr + 3] as usize;
        scope[out_csr] = p1 + p2;
        Some(csr + 4)
    } else if op == 2 {
        let p1 = resolve_param(param_modes[0], scope[csr + 1], &scope);
        let p2 = resolve_param(param_modes[1], scope[csr + 2], &scope);
        let out_csr = scope[csr + 3] as usize;
        scope[out_csr] = p1 * p2;
        Some(csr + 4)
    } else if op == 3 {
        let out_csr = scope[csr + 1] as usize;
        scope[out_csr] = input as i32;
        Some(csr + 2)
    } else if op == 4 {
        println!("{}",resolve_param(param_modes[0], scope[csr + 1], &scope));
        Some(csr + 2)
    } else {
        println!("exiting at csr {}", csr);
        None
    }
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
        assert_eq!(exec_instr(0,0,&mut scope), Some(3));
        assert_eq!(scope, vec![2,0,0,0,99]);
        assert_eq!(exec_instr(0,0,&mut scope), Some(3));
        assert_eq!(scope, vec![4,0,0,0,99]);
        scope = vec![1002,4,3,4,33];
        assert_eq!(exec_instr(0,0,&mut scope), Some(3));
        assert_eq!(scope, vec![1002,4,3,4,99]);
        scope = vec![1101,100,-1,4,0];
        assert_eq!(exec_instr(0,0,&mut scope), Some(3));
        assert_eq!(scope, vec![1101,100,-1,4,99]);
    }
    #[test]
    fn test_int_code() {
        assert_eq!(run_intcode(0,vec![1101,100,-1,4,0]), vec![1101,100,-1,4,99]);
        assert_eq!(run_intcode(0,vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
        assert_eq!(run_intcode(0,vec![2,4,4,5,99,0]), vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn fix_nonzero_code() {
        let mut scope = vec![3,15,1,15,6,6,1100,1,238,15,104,0,1101,40,0,/*15*/0];
        assert_eq!(exec_instr(1,0,&mut scope), Some(2));
        assert_eq!(scope, vec![3,15,1,15,6,6,1100,1,238,15,104,0,1101,40,0,/*15*/1]);
        assert_eq!(exec_instr(1,2,&mut scope), Some(6));
        assert_eq!(scope, vec![3,15,1,15,6,6,1101,1,238,15,104,0,1101,40,0,/*15*/1]);
        assert_eq!(exec_instr(1,6,&mut scope), Some(10));
        assert_eq!(scope, vec![3,15,1,15,6,6,1101,1,238,15,104,0,1101,40,0,/*15*/239]);
        assert_eq!(exec_instr(1,6,&mut scope), Some(10));
        assert_eq!(scope, vec![3,15,1,15,6,6,1101,1,238,15,104,0,1101,40,0,/*15*/239]);


    }

}