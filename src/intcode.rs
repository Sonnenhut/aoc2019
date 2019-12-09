impl IntCode {
    pub fn create(inputs: &Vec<i64>, pgm: &Vec<i64>) -> IntCode{
        IntCode {inputs: inputs.clone(), csr: Some(0), pgm: pgm.clone(), output:vec![], base: 0, initial_memory: pgm.len()}
    }
    pub fn resolve(inputs: &Vec<i64>, pgm: &Vec<i64>) -> i64 {
        IntCode::create(inputs, pgm).run()
    }
    fn csr_at(&mut self, new_csr: usize) {
        self.csr = Some(new_csr);
    }
    fn run(&mut self) -> i64 {
        while self.csr.is_some() {
            self.csr = self.run_single();
        }
        *self.output.last().unwrap_or(&0)
    }
    pub fn push_input(&mut self, input : i64) {
        self.inputs.push(input);
    }
    pub fn next(&mut self) -> Option<i64> { // until next output
        let last_out = self.output.clone();
        while self.csr.is_some() {
            self.csr = self.run_single();
            if last_out != self.output {
                // some new output, send it back
                return self.output.last().cloned();
            }
        }
        return None
    }
    fn run_single(&mut self) -> Option<usize> {
        let csr = self.csr.unwrap();
        let (op, param_modes) = parse_op(self.pgm[csr] as u32);

        let(p1,p2) = self.params(&param_modes);
        if op == 1 { // add
            self.write_at(3, &param_modes, p1? + p2?);
            Some(csr + 4)
        } else if op == 2 { // mul
            self.write_at(3, &param_modes, p1? * p2?);
            Some(csr + 4)
        } else if op == 3 { // read_in
            let input = self.next_input();
            self.write_at(1, &param_modes, input);
            Some(csr + 2)
        } else if op == 4 { //write_out
            self.output.push(p1?);
            Some(csr + 2)
        } else if op == 5 { // jump-if-true
            if p1? != 0 { Some(p2? as usize) } else { Some(csr + 3) }
        } else if op == 6 { // jump-if-false
            if p1? == 0 { Some(p2? as usize) } else { Some(csr + 3) }
        } else if op == 7 { // less-than
            self.write_at(3, &param_modes, (p1? < p2?) as i64);
            Some(csr + 4)
        } else if op == 8 { // less-than
            self.write_at(3, &param_modes, (p1? == p2?) as i64);
            Some(csr + 4)
        } else if op == 9 { // adjust-relative-base
            self.base += p1?;
            Some(csr + 2)
        } else if op == 99 {
            None
        } else {
            panic!("Unknown opcode");
        }
    }
    fn next_input(&mut self) -> i64{
        self.inputs.remove(0)
    }

    fn write_at(&mut self, offset: usize, param_modes: &Vec<u32>, val: i64) {
        let out_csr = self.resolve_param_csr(offset, &param_modes).unwrap() as usize;
        if self.pgm.len() <= out_csr {
            self.pgm.resize(out_csr +1, 0)
        }
        self.pgm[out_csr] = val;
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
        self.pgm.get(csr).cloned().unwrap_or(0)
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

fn resolve_param(mode : u32, param: i64, scope: &Vec<i64>) -> Option<i64> {
    if mode == 0 {
        scope.get(param as usize).cloned()
    } else if mode == 1 {
        Some(param)
    } else {
        panic!("ParamMode not defined!")
    }
}

pub struct IntCode {
    inputs: Vec<i64>,
    output: Vec<i64>,
    csr: Option<usize>,
    pgm: Vec<i64>,
    base: i64,
    initial_memory: usize
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
        assert_eq!(resolve_param(0,0, &vec![99]), Some(99));
        assert_eq!(resolve_param(1,0, &vec![99]), Some(0));
    }
    #[test]
    fn test_single_instr() {
        let mut scope = vec![1,0,0,0,99];
        let mut toTest = IntCode::create(&vec![0],&vec![1,0,0,0,99]);
        toTest.csr = toTest.run_single();
        assert_eq!(toTest.csr, Some(4));
        assert_eq!(toTest.pgm, vec![2,0,0,0,99]);
        toTest.csr_at(0);
        toTest.csr = toTest.run_single();
        assert_eq!(toTest.csr, Some(4));
        assert_eq!(toTest.pgm, vec![4,0,0,0,99]);

        toTest = IntCode::create(&vec![0],&vec![1002,4,3,4,33]);
        toTest.csr = toTest.run_single();
        assert_eq!(toTest.csr, Some(4));
        assert_eq!(toTest.pgm, vec![1002,4,3,4,99]);

        toTest = IntCode::create(&vec![0],&vec![1101,100,-1,4,0]);
        toTest.csr = toTest.run_single();
        assert_eq!(toTest.csr, Some(4));
        assert_eq!(toTest.pgm, vec![1101,100,-1,4,99]);
    }

    #[test]
    fn test_int_code() {
        let mut toTest = IntCode::create(&vec![0],&vec![1101,100,-1,4,0]);
        toTest.run();
        assert_eq!(toTest.pgm, vec![1101,100,-1,4,99]);

        toTest = IntCode::create(&vec![0],&vec![1,1,1,4,99,5,6,0,99]);
        toTest.run();
        assert_eq!(toTest.pgm, vec![30,1,1,4,2,5,6,0,99]);


        toTest = IntCode::create(&vec![0],&vec![2,4,4,5,99,0]);
        toTest.run();
        assert_eq!(toTest.pgm, vec![2,4,4,5,99,9801]);
    }

    #[test]
    fn fix_nonzero_code() {
        let mut toTest = IntCode::create(&vec![1], &vec![3, 15, 1, 15, 6, 6, 1100, 1, 238, 15, 104, 0, 1101, 40, 0, /*15*/0]);
        toTest.csr = toTest.run_single();
        assert_eq!(toTest.pgm, vec![3,15,1,15,6,6,1100,1,238,15,104,0,1101,40,0,/*15*/1]);
        toTest.csr = toTest.run_single();
        assert_eq!(toTest.pgm, vec![3,15,1,15,6,6,1101,1,238,15,104,0,1101,40,0,/*15*/1]);
        toTest.csr = toTest.run_single();
        assert_eq!(toTest.pgm, vec![3,15,1,15,6,6,1101,1,238,15,104,0,1101,40,0,/*15*/239]);
        toTest.csr = toTest.run_single();
        assert_eq!(toTest.pgm, vec![3,15,1,15,6,6,1101,1,238,15,104,0,1101,40,0,/*15*/239]);
    }

    #[test]
    fn test_pgm_pt2_day5() {
        assert_eq!(IntCode::resolve(&vec![8], &vec![3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]), 1); // is eq 8
        // lt 8
        let lt8_position_mode = vec![3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8];
        assert_eq!(IntCode::resolve(&vec![1], &lt8_position_mode), 1);
        assert_eq!(IntCode::resolve(&vec![2], &lt8_position_mode), 1);
        assert_eq!(IntCode::resolve(&vec![3], &lt8_position_mode), 1);
        assert_eq!(IntCode::resolve(&vec![4], &lt8_position_mode), 1);
        assert_eq!(IntCode::resolve(&vec![5], &lt8_position_mode), 1);
        assert_eq!(IntCode::resolve(&vec![6], &lt8_position_mode), 1);
        assert_eq!(IntCode::resolve(&vec![7], &lt8_position_mode), 1);

        // eq 8
        let eq8_position_mode = vec![3, 3, 1108, -1, 8, 3, 4, 3, 99];
        assert_eq!(IntCode::resolve(&vec![1], &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve(&vec![2], &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve(&vec![3], &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve(&vec![4], &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve(&vec![5], &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve(&vec![6], &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve(&vec![7], &eq8_position_mode), 0);
        assert_eq!(IntCode::resolve(&vec![8], &eq8_position_mode), 1);

        // lt 8
        let lt8_immediate_mode = vec![3, 3, 1107, -1, 8, 3, 4, 3, 99];
        assert_eq!(IntCode::resolve(&vec![1], &lt8_immediate_mode), 1);
        assert_eq!(IntCode::resolve(&vec![2], &lt8_immediate_mode), 1);
        assert_eq!(IntCode::resolve(&vec![3], &lt8_immediate_mode), 1);
        assert_eq!(IntCode::resolve(&vec![4], &lt8_immediate_mode), 1);
        assert_eq!(IntCode::resolve(&vec![5], &lt8_immediate_mode), 1);
        assert_eq!(IntCode::resolve(&vec![6], &lt8_immediate_mode), 1);
        assert_eq!(IntCode::resolve(&vec![7], &lt8_immediate_mode), 1);


        // non_zero
        let non_zero_position_mode = vec![3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9];
        assert_eq!(IntCode::resolve(&vec![1], &non_zero_position_mode), 1);
        assert_eq!(IntCode::resolve(&vec![-1], &non_zero_position_mode), 1);
        assert_eq!(IntCode::resolve(&vec![0], &non_zero_position_mode), 0);

        // non_zero
        let non_zero_immediate_mode = vec![3,3,1105,-1,9,1101,0,0,12,4,12,99,1];
        assert_eq!(IntCode::resolve(&vec![1], &non_zero_immediate_mode), 1);
        assert_eq!(IntCode::resolve(&vec![-1], &non_zero_immediate_mode), 1);
        assert_eq!(IntCode::resolve(&vec![0], &non_zero_immediate_mode), 0);

        //larger pgm
        let large_pgm = vec![3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,
                             1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,
                             999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99];
        assert_eq!(IntCode::resolve(&vec![6], &large_pgm), 999); // below 8
        assert_eq!(IntCode::resolve(&vec![6], &large_pgm), 999);
        assert_eq!(IntCode::resolve(&vec![7], &large_pgm), 999);
        assert_eq!(IntCode::resolve(&vec![8], &large_pgm), 1000); // is 8
        assert_eq!(IntCode::resolve(&vec![9], &large_pgm), 1001);
        assert_eq!(IntCode::resolve(&vec![10], &large_pgm), 1001);
        assert_eq!(IntCode::resolve(&vec![11], &large_pgm), 1001);
    }

    #[test]
    fn test_access_outside_intial_memory_day9() {
        let mut toTest = IntCode::create(&vec![1], &vec![1,5,6,0,99]);
        toTest.csr = toTest.run_single();
        assert_eq!(toTest.pgm, vec![1,5,6,0,99]);

        toTest = IntCode::create(&vec![1], &vec![1,6,7,0,99]);
        toTest.csr = toTest.run_single();
        assert_eq!(toTest.pgm, vec![3,6,7,0,99]);

        toTest = IntCode::create(&vec![1], &vec![1,7,8,0,99]);
        toTest.csr = toTest.run_single();
        assert_eq!(toTest.pgm, vec![5,7,8,0,99]);
    }

    #[test]
    fn test_read_write_outside_initial_memory() {
        let mut toTest = IntCode::create(&vec![], &vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);
        toTest.run();
        assert_eq!(toTest.output, vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]);

        toTest = IntCode::create(&vec![], &vec![1102,34915192,34915192,7,4,7,99,0]);
        toTest.run();
        assert_eq!(toTest.output, vec![1219070632396864]);

        toTest = IntCode::create(&vec![], &vec![104,1125899906842624,99]);
        toTest.run();
        assert_eq!(toTest.output, vec![1125899906842624]);
    }

}