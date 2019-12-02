use aoc2019::read_lines;

fn main() {
    let nums: Vec<usize> = read_lines(2)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", pt1(&nums)); // 9706670
    //println!("pt2: {}", pt2(&masses)); //
}

fn pt1(nums: &Vec<usize>) -> usize {
    let input = &mut nums.clone();
    input[1] = 12;
    input[2] = 2;
    return compute_pt1(input)[0];
}

fn compute_pt1(nums: &Vec<usize>) -> Vec<usize> {
    let res  = &mut nums.clone();

    let all_indexes : Vec<usize>= (0..res.len()).collect();
    for indexes in all_indexes.chunks(4) {
        let op = res[indexes[0]];
        if op == 99 {break;}

        let csr_1 = res[indexes[1]];
        let csr_2 = res[indexes[2]];
        let csr_target = res[indexes[3]];
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