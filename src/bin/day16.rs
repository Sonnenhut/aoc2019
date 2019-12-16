use aoc2019::read_lines;
use std::iter::repeat;

fn main() {
    let input = &read_lines(16)[0];
    println!("pt1: {}", pt1(&input)); // 22122816
    println!("pt2: {}", pt2(&input)); // 41402171
}

fn pt1(n: &String) -> String {
    let seq : Vec<i32> = n.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    fft(seq, 100)
}
fn pt2(n: &String) -> String {
    let seq : Vec<i32> = n.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    fft_2(&seq, 100)
}

fn fft_2(initial_seq: &Vec<i32>, phase_cnt: usize) -> String {
    let off : usize = initial_seq[..7].iter().map(|x|x.to_string()).collect::<Vec<String>>().concat().parse().unwrap();

    let mut out_list: Vec<i32> = initial_seq.iter()
        .cycle()
        .skip(off)
        .take(10_000 * initial_seq.len() - off)
        .cloned().collect();
    for _ in 0..phase_cnt {
        // the trick is that the pattern does not really matter, the offset is so big taht all remaining numbers have 1
        let before_phase: Vec<i32> = out_list.iter().cloned().collect();
        for i in (0..out_list.len()).rev() {
            let last_calc = out_list.get(i+1).unwrap_or(&0);
            let out = ((last_calc + before_phase[i]) % 10).abs();
            out_list[i] = out;
        }
    }
    let str_res : Vec<String> = out_list.iter().map(|x| x.to_string()).take(8).collect();
    str_res.concat()
}

fn fft(seq: Vec<i32>, phase_cnt: usize) -> String {
    let base_pattern = [0,1,0,-1].to_vec();
    let off = 0;

    let patterns : Vec<Vec<i32>> = (0..seq.len())
        .skip(off)
        .map(|i| {
            base_pattern.iter()
                .flat_map(|x| {
                    repeat(*x).take(i + 1).collect::<Vec<i32>>()
                })
                .cycle()
                .skip(1)
                .take(seq.len())
                .collect::<Vec<i32>>()
        })
        .collect();

    let mut out_list: Vec<i32> = seq.clone();
    for _ in 0..phase_cnt {
        let before_phase: Vec<i32> = out_list.iter().cloned().collect();
        for i in 0..out_list.len() {
            let out_raw : i32 = before_phase.iter().zip(patterns[i].iter())
                .filter(|(lhs,rhs)| **lhs != 0 && **rhs != 0)
                .map(|(lhs,rhs)| lhs * rhs)
                .sum();
            let out = (out_raw % 10).abs();
            out_list[i] = out;
        }
    }
    let str_res : Vec<String> = out_list.iter().map(|x| x.to_string()).take(8).collect();
    str_res.concat()
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_regression() {
        let input = &read_lines(16)[0];
        assert_eq!(pt1(&input), "22122816");
    }
    #[test]
    fn test_ex() {
        assert_eq!(fft([1,2,3,4,5,6,7,8].to_vec(),1), "48226158");
        assert_eq!(fft([1,2,3,4,5,6,7,8].to_vec(),2), "34040438");
        assert_eq!(fft([1,2,3,4,5,6,7,8].to_vec(),3), "03415518");
        assert_eq!(fft([1,2,3,4,5,6,7,8].to_vec(),4), "01029498");
    }

    #[test]
    fn test_ex_pt1() {
        assert_eq!(pt1(&"80871224585914546619083218645595".to_string()), "24176176");
        assert_eq!(pt1(&"19617804207202209144916044189917".to_string()), "73745418");
        assert_eq!(pt1(&"69317163492948606335995924319873".to_string()), "52432133");
    }

    #[test]
    fn test_ex_pt2() {
        assert_eq!(pt2(&"03036732577212944063491565474664".to_string()), "84462026");
        assert_eq!(pt2(&"02935109699940807407585447034323".to_string()), "78725270");
        assert_eq!(pt2(&"03081770884921959731165446850517".to_string()), "53553731");
    }
}