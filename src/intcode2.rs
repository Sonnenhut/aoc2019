use std::sync::mpsc;
use std::sync::mpsc::{Receiver, channel, Sender};
use std::thread;
use crate::intcode::IntCode;

impl IntCode2 {
    pub fn run_async(mem: &Vec<i64>) -> (Sender<i64>, Receiver<Option<i64>>) {
        let mem_copy = mem.to_vec();
        let (send_in, recv_in) = channel();
        let (send_out, recv_out) = channel();
        thread::spawn(move || {
            let mut int_code = IntCode2 {inputs: recv_in, csr: Some(0), mem: mem_copy, output: send_out, base: 0};
            int_code.run()
        });
        (send_in, recv_out)
    }
    pub fn run(&mut self) {
        while self.csr.is_some() {
            self.csr = self.run_single();
        }
    }
    fn run_single(&mut self) -> Option<usize> {
        let csr = self.csr.unwrap();
        let (op, param_modes) = parse_op(self.mem[csr] as u32);

        let(p1,p2) = self.params(&param_modes);
        match op {
            1 => self.write_at(3, &param_modes, p1? + p2?), // add
            2 => self.write_at(3, &param_modes, p1? * p2?), // mul
            3 => { // read_in
                self.output.send(None); // Give me more input!
                let input = self.inputs.recv().unwrap();
                self.write_at(1, &param_modes, input)
            },
            4 => { //write_out
                self.output.send(Some(p1?));
                Some(csr + 2)
            },
            5 => if p1? != 0 { Some(p2? as usize) } else { Some(csr + 3) }, // jump-if-true
            6 => if p1? == 0 { Some(p2? as usize) } else { Some(csr + 3) },// jump-if-false
            7 => self.write_at(3, &param_modes, (p1? < p2?) as i64), // less-than
            8 => self.write_at(3, &param_modes, (p1? == p2?) as i64), // eq
            9 => { // adjust-relative-base
                self.base += p1?;
                Some(csr + 2)
            },
            99 => None,
            _ => panic!("Unknown opcode")
        }
    }

    fn write_at(&mut self, offset: usize, param_modes: &Vec<u32>, val: i64) -> Option<usize> {
        let out_csr = self.resolve_param_csr(offset, &param_modes).unwrap() as usize;
        if self.mem.len() <= out_csr {
            self.mem.resize(out_csr +1, 0)
        }
        self.mem[out_csr] = val;
        Some(self.csr.unwrap() + offset + 1) // new cursor after writing
    }

    fn params(&self,param_modes: &Vec<u32>) -> (Option<i64>, Option<i64>){
        let csr = self.csr.unwrap();
        let p1 : Option<i64> = self.resolve_param_csr(1,&param_modes)
            .map(|c| self.get_at(c as usize)) // at csr
            .or_else(|| Some(self.get_at(csr+1))); // immediate
        let p2 :Option<i64> = self.resolve_param_csr(2,&param_modes)
            .map(|c| self.get_at(c as usize))
            .or_else(|| Some(self.get_at(csr+2)));

        (p1,p2)
    }
    fn get_at(&self, csr: usize) -> i64 {
        self.mem.get(csr).cloned().unwrap_or(0)
    }

    fn resolve_param_csr(&self, offset: usize, param_modes: &Vec<u32>) -> Option<i64> {
        let csr = self.csr.unwrap();
        let mode = param_modes[offset -1];
        let val_at_csr = Some(self.get_at((csr + offset) as usize));
        if mode == 0 {
            // get at csr
            val_at_csr
        } else if mode == 1 {
            None // no csr, is immediate (also: outputs should never go to immediate)
        } else if mode == 2 {
            // get at base + csr
            val_at_csr.map(|c| (self.base + c))
        } else {
            panic!("ParamMode not defined!")
        }
    }
}

pub struct IntCode2 {
    inputs: Receiver<i64>,
    output: Sender<Option<i64>>,
    csr: Option<usize>,
    mem: Vec<i64>,
    base: i64,
}

fn parse_op(input: u32) -> (u32, Vec<u32>) {
    let s : String = format!("{:05}", input);
    let op : u32 = s[3..=4].parse().unwrap();
    let param_modes = s[0..=2].chars().rev().map(|x| x.to_digit(10).unwrap()).collect();
    (op, param_modes)
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex() {
        assert_eq!(parse_op(1002), (2,vec![0,1,0]));
        assert_eq!(parse_op(1102), (2, vec![1,1,0]));
        assert_eq!(parse_op(10002), (2, vec![0,0,1]));
    }/*
    #[test]
    fn test_single_instr() {
        let mut to_test = IntCode::create(&vec![0], &vec![1, 0, 0, 0, 99]);
        to_test.csr = to_test.run_single();
        assert_eq!(to_test.csr, Some(4));
        assert_eq!(to_test.mem, vec![2, 0, 0, 0, 99]);
        to_test.csr = Some(0);
        to_test.csr = to_test.run_single();
        assert_eq!(to_test.csr, Some(4));
        assert_eq!(to_test.mem, vec![4, 0, 0, 0, 99]);

        to_test = IntCode::create(&vec![0], &vec![1002, 4, 3, 4, 33]);
        to_test.csr = to_test.run_single();
        assert_eq!(to_test.csr, Some(4));
        assert_eq!(to_test.mem, vec![1002, 4, 3, 4, 99]);

        to_test = IntCode::create(&vec![0], &vec![1101, 100, -1, 4, 0]);
        to_test.csr = to_test.run_single();
        assert_eq!(to_test.csr, Some(4));
        assert_eq!(to_test.mem, vec![1101, 100, -1, 4, 99]);
    }

    #[test]
    fn test_int_code() {
        let mut to_test = IntCode::create(&vec![0], &vec![1101, 100, -1, 4, 0]);
        to_test.run();
        assert_eq!(to_test.mem, vec![1101, 100, -1, 4, 99]);

        to_test = IntCode::create(&vec![0], &vec![1, 1, 1, 4, 99, 5, 6, 0, 99]);
        to_test.run();
        assert_eq!(to_test.mem, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);


        to_test = IntCode::create(&vec![0], &vec![2, 4, 4, 5, 99, 0]);
        to_test.run();
        assert_eq!(to_test.mem, vec![2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn fix_nonzero_code() {
        let mut to_test = IntCode::create(&vec![1], &vec![3, 15, 1, 15, 6, 6, 1100, 1, 238, 15, 104, 0, 1101, 40, 0, /*15*/0]);
        to_test.csr = to_test.run_single();
        assert_eq!(to_test.mem, vec![3, 15, 1, 15, 6, 6, 1100, 1, 238, 15, 104, 0, 1101, 40, 0, /*15*/1]);
        to_test.csr = to_test.run_single();
        assert_eq!(to_test.mem, vec![3, 15, 1, 15, 6, 6, 1101, 1, 238, 15, 104, 0, 1101, 40, 0, /*15*/1]);
        to_test.csr = to_test.run_single();
        assert_eq!(to_test.mem, vec![3, 15, 1, 15, 6, 6, 1101, 1, 238, 15, 104, 0, 1101, 40, 0, /*15*/239]);
        to_test.csr = to_test.run_single();
        assert_eq!(to_test.mem, vec![3, 15, 1, 15, 6, 6, 1101, 1, 238, 15, 104, 0, 1101, 40, 0, /*15*/239]);
    }

    #[test]
    fn test_mem_pt2_day5() {
        assert_eq!(IntCode::resolve_single(&vec![8], &vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]), 1); // is eq 8
        // lt 8
        let lt8_position_mode = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(IntCode::resolve_single(&vec![1], &lt8_position_mode), 1);
        assert_eq!(IntCode::resolve_single(&vec![2], &lt8_position_mode), 1);
        assert_eq!(IntCode::resolve_single(&vec![3], &lt8_position_mode), 1);
        assert_eq!(IntCode::resolve_single(&vec![4], &lt8_position_mode), 1);
        assert_eq!(IntCode::resolve_single(&vec![5], &lt8_position_mode), 1);
        assert_eq!(IntCode::resolve_single(&vec![6], &lt8_position_mode), 1);
        assert_eq!(IntCode::resolve_single(&vec![7], &lt8_position_mode), 1);

        // eq 8
        let eq8_position_mode = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(IntCode::resolve_single(&vec![1], &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve_single(&vec![2], &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve_single(&vec![3], &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve_single(&vec![4], &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve_single(&vec![5], &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve_single(&vec![6], &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve_single(&vec![7], &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve_single(&vec![8], &eq8_position_mode), 1);

        // lt 8
        let lt8_immediate_mode = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(IntCode::resolve_single(&vec![1], &lt8_immediate_mode), 1);
        assert_eq!(IntCode::resolve_single(&vec![2], &lt8_immediate_mode), 1);
        assert_eq!(IntCode::resolve_single(&vec![3], &lt8_immediate_mode), 1);
        assert_eq!(IntCode::resolve_single(&vec![4], &lt8_immediate_mode), 1);
        assert_eq!(IntCode::resolve_single(&vec![5], &lt8_immediate_mode), 1);
        assert_eq!(IntCode::resolve_single(&vec![6], &lt8_immediate_mode), 1);
        assert_eq!(IntCode::resolve_single(&vec![7], &lt8_immediate_mode), 1);


        // non_zero
        let non_zero_position_mode = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        assert_eq!(IntCode::resolve_single(&vec![1], &non_zero_position_mode), 1);
        assert_eq!(IntCode::resolve_single(&vec![-1], &non_zero_position_mode), 1);
        assert_eq!(IntCode::resolve_single(&vec![0], &non_zero_position_mode), 0);

        // non_zero
        let non_zero_immediate_mode = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        assert_eq!(IntCode::resolve_single(&vec![1], &non_zero_immediate_mode), 1);
        assert_eq!(IntCode::resolve_single(&vec![-1], &non_zero_immediate_mode), 1);
        assert_eq!(IntCode::resolve_single(&vec![0], &non_zero_immediate_mode), 0);

        //larger mem
        let large_mem = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                             1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                             999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        assert_eq!(IntCode::resolve_single(&vec![6], &large_mem), 999); // below 8
        assert_eq!(IntCode::resolve_single(&vec![6], &large_mem), 999);
        assert_eq!(IntCode::resolve_single(&vec![7], &large_mem), 999);
        assert_eq!(IntCode::resolve_single(&vec![8], &large_mem), 1000); // is 8
        assert_eq!(IntCode::resolve_single(&vec![9], &large_mem), 1001);
        assert_eq!(IntCode::resolve_single(&vec![10], &large_mem), 1001);
        assert_eq!(IntCode::resolve_single(&vec![11], &large_mem), 1001);
    }

    #[test]
    fn test_access_outside_intial_memory_day9() {
        let mut to_test = IntCode::create(&vec![1], &vec![1, 5, 6, 0, 99]);
        to_test.csr = to_test.run_single();
        assert_eq!(to_test.mem, vec![0, 5, 6, 0, 99]);

        to_test = IntCode::create(&vec![1], &vec![1, 6, 7, 0, 99]);
        to_test.csr = to_test.run_single();
        assert_eq!(to_test.mem, vec![0, 6, 7, 0, 99]);

        to_test = IntCode::create(&vec![1], &vec![1, 7, 8, 0, 99]);
        to_test.csr = to_test.run_single();
        assert_eq!(to_test.mem, vec![0, 7, 8, 0, 99]);
    }

    #[test]
    fn test_read_write_outside_initial_memory() {
        let mut to_test = IntCode::create(&vec![], &vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]);
        to_test.run();
        assert_eq!(to_test.output, vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]);

        to_test = IntCode::create(&vec![], &vec![1102, 34915192, 34915192, 7, 4, 7, 99, 0]);
        to_test.run();
        assert_eq!(to_test.output, vec![1219070632396864]);

        to_test = IntCode::create(&vec![], &vec![104, 1125899906842624, 99]);
        to_test.run();
        assert_eq!(to_test.output, vec![1125899906842624]);
    }
*/
}