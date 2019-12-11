use aoc2019::read_lines;
use aoc2019::intcode::IntCode;

fn main() {
    let mem: Vec<i64> = read_lines(2)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", pt1(&mem)); // 9706670
    println!("pt2: {}", pt2(&mem)); // 2552
}

fn pt1(mem: &Vec<i64>) -> i64 {
    let mut modified_mem = mem.to_vec();
    modified_mem[1] = 12;
    modified_mem[2] = 2;
    let mut int_code = IntCode::create(&vec![], &modified_mem);
    int_code.run();

    return int_code.memory()[0]
}

fn pt2(mem: &Vec<i64>) -> i64 {
    for noun in 0..100 {
        for verb in 0..100 {
            let mut modified_mem = mem.to_vec();
            modified_mem[1] = noun;
            modified_mem[2] = verb;
            let mut int_code = IntCode::create(&vec![], &modified_mem);
            int_code.run();
            let res = int_code.memory()[0];
            if res == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}
