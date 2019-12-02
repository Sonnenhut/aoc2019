use aoc2019::read_lines;

fn main() {
    let nums: Vec<usize> = read_lines(2)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", pt1(&nums)); // 9706670
    println!("pt2: {}", pt2(&nums)); // 2552
}

fn pt1(nums: &Vec<usize>) -> usize {
    let input = &mut nums.clone();
    input[1] = 12;
    input[2] = 2;
    return compute_pt1(input)[0];
}

fn pt2(nums: &Vec<usize>) -> usize {
    for noun in 0..100 {
        for verb in 0..100 {
            let input = &mut nums.clone();
            input[1] = noun;
            input[2] = verb;
            let res = compute_pt1(input)[0];
            if res == 19690720 {
                return 100 * noun + verb;
            }
        }
    }
    0
}

fn compute_pt1(nums: &Vec<usize>) -> Vec<usize> {
    let res  = &mut nums.clone();

    for instr_idx in (0..res.len()).step_by(4) {
        let op = res[instr_idx];
        if op == 99 {break;}

        let csr_1 = res[instr_idx+1];
        let csr_2 = res[instr_idx+2];
        let csr_target = res[instr_idx+3];
        if op == 1 {
            res[csr_target] = res[csr_1] + res[csr_2];
        } else if op == 2 {
            res[csr_target] = res[csr_1] * res[csr_2];
        }
    }
    res.to_vec()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex() {
        assert_eq!(compute_pt1(&mut vec![1,1,1,0,99]), vec![2,1,1,0,99]);
        assert_eq!(compute_pt1(&mut vec![1,9,10,3,2,3,11,0,99,30,40,50]), vec![3500,9,10,70,2,3,11,0, 99,30,40,50]);
        assert_eq!(compute_pt1(&mut vec![1,0,0,0,99]), vec![2,0,0,0,99]);
        assert_eq!(compute_pt1(&mut vec![2,4,4,5,99,0]), vec![2,4,4,5,99,9801]);
        assert_eq!(compute_pt1(&mut vec![1,1,1,4,99,5,6,0,99]), vec![30,1,1,4,2,5,6,0,99]);
    }
}