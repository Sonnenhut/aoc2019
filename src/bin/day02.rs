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
    (0..100).flat_map(|n| (0..100).map(move |v| (n,v)))
        .map(|(noun, verb)| {
            let mut modified_mem = mem.to_vec();
            modified_mem[1] = noun;
            modified_mem[2] = verb;
            let mut int_code = IntCode::create(&vec![], &modified_mem);
            int_code.run();
            ( int_code.memory()[0], 100 * noun + verb )
        })
        .filter(|(out, _)| *out == 19690720)
        .map(|(_, res)| res)
        .next().unwrap()
}



#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn regression() {
        let mem: Vec<i64> = read_lines(2)[0].split(',').map(|s| s.parse().unwrap()).collect();
        assert_eq!(pt1(&mem), 9706670);
        assert_eq!(pt2(&mem), 2552);
    }
}