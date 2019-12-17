use aoc2019::read_lines;
use aoc2019::intcode::IntCode;

fn main() {
    let mem: Vec<i64> = read_lines(17)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", pt1(&mem)); // 6680
    println!("pt2: {}", pt2(&mem)); // 1103905
}

// solved by printing it out and solving by hand (as many others did...)
fn pt2(intcode_mem: &Vec<i64>) -> usize {
    let mut modified_mem = intcode_mem.to_vec();
    modified_mem[0] = 2;
    let (i,o) = IntCode::run_async(&modified_mem);

    let movement = "A,B,A,C,C,A,B,C,B,B";
    let a = "L,8,R,10,L,8,R,8";
    let b = "L,12,R,8,R,8";
    let c = "L,8,R,6,R,6,R,10,L,8";
    let use_vid = "n";
    let instr = format!("{}\n{}\n{}\n{}\n{}\n", movement,a,b,c,use_vid);
    for input in instr.chars() {
        let _ = i.send(input as i64);
    }

    let output : Vec<i64> = o.iter().collect();
    *output.last().unwrap() as usize
}

fn pt1(intcode_mem: &Vec<i64>) -> usize {
    let (_,o) = IntCode::run_async(&intcode_mem);
    let map = o.iter().collect();
    draw(&map);
    alignment_sum(&map)
}

fn draw(nums : &Vec<i64>) {
    for n in nums.iter() {
        let u = n.wrapping_abs() as u8;
        print!("{}", std::str::from_utf8(&[u]).unwrap())
    }
    println!();
}

fn alignment_sum(nums: &Vec<i64>) -> usize {
    let line_len = nums.iter().take_while(|n| **n != 10).count(); // until a newline
    let lines : Vec<Vec<i64>> = nums.iter()
        .filter(|n| **n != 10)
        .cloned()
        .collect::<Vec<i64>>()
        .chunks(line_len)
        .map(|chunk| chunk.to_vec())
        .collect();

    let mut sum = 0;
    for (y,line) in lines.iter().enumerate() {
        for (x, n) in line.iter().enumerate() {
            if *n == 35 && around(x,y,&lines).iter().all(|other_n| *other_n == 35) {
                sum += x * y;
            }
        }
    }
    sum
}

fn around(x: usize, y: usize, lines : &Vec<Vec<i64>>) -> Vec<i64>{
    let mut res : Vec<i64> = vec![];
    if y != 0 && y < lines.len() -1 {
        res.push(lines[y-1][x]);
        res.push(lines[y+1][x]);
    } else {
        res.push(0); // on the edge, is not a valid thing
        res.push(0);
    }
    if x != 0 && x < lines[0].len() -1 {
        res.push(lines[y][x+1]);
        res.push(lines[y][x-1]);
    } else {
        res.push(0); // on the edge, is not a valid thing
        res.push(0);
    }
    res
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_regression() {
        let mem: Vec<i64> = read_lines(17)[0].split(',').map(|s| s.parse().unwrap()).collect();
        assert_eq!(pt1(&mem), 6680);
        assert_eq!(pt2(&mem),1103905);
    }
    #[test]
    fn test() {
        let mem: Vec<i64> = read_lines(17)[0].split(',').map(|s| s.parse().unwrap()).collect();
        pt2(&mem);
    }
    #[test]
    fn test_ex() {
        let ex : Vec<i64> =
"..#..........
..#..........
#######...###
#.#...#...#.#
#############
..#...#...#..
..#####...^..".chars().map(|c| c as i64).collect();

        //draw(&ex);
        assert_eq!(alignment_sum(&ex), 76);
    }

}