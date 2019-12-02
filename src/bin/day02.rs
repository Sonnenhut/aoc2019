use aoc2019::read_lines;

fn main() {
    let masses: Vec<usize> = read_lines(2).iter().map(|s| s.parse().unwrap()).collect();
    //println!("pt1: {}", pt1(&masses)); // 3395944
    //println!("pt2: {}", pt2(&masses)); // 5091036
}

fn pt1(nums: &mut Vec<usize>) -> usize {
    let mut op : usize = 0;
    let mut taken = vec![];
    for (i, ref_idx) in nums.iter_mut().enumerate() {
        println!("{}", num);
        if *ref_idx == 99 {
            println!("break");
            break;
        } else if i % 4 == 0 {
            op = ref_idx.to_owned()
        } else if i % 3 == 0 { // output
            println!("{:?} {}", taken, ref_idx)
        } else if op == 1 || op == 2{
            taken.push(ref_idx);
        }
    }
    1
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex() {
        assert_eq!(pt1(&mut vec![1,9,10,3,99]), vec![1,9,10,3]);
        //assert_eq!(total_fuel(1969), 966);
        //assert_eq!(total_fuel(100756), 50346);
    }
}