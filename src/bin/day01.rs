use aoc2019::read_lines;

fn main() {
    let masses: Vec<usize> = read_lines(1).iter().map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", pt1(&masses)); // 3395944
    println!("pt2: {}", pt2(&masses)); // 5091036
}

fn pt1(masses: &Vec<usize>) -> usize {
    masses.iter()
        .map(|mass| fuel(*mass))
        .sum()
}

fn pt2(masses: &Vec<usize>) -> usize {
    masses.iter()
        .map(|mass| total_fuel(*mass))
        .sum()
}

fn fuel(mass: usize) -> usize {
    ((mass / 3) as isize - 2).max(0) as usize
}

// calc the fuel plus the fuel for the fuel plus the fuel for the fuel plus the fuel for ...
fn total_fuel(mass: usize) -> usize {
    let res = fuel(mass);
    match res {
        r if r > 0 =>  total_fuel(r) + r,
        _ => res
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_total_fuel() {
        assert_eq!(total_fuel(14), 2);
        //assert_eq!(total_fuel(1969), 966);
        //assert_eq!(total_fuel(100756), 50346);
    }
    #[test]
    fn test_fuel() {
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(100756), 33583);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(654), 216);
        assert_eq!(fuel(216), 70);
        assert_eq!(fuel(70), 21);
        assert_eq!(fuel(21), 5);
        assert_eq!(fuel(5), 0);
    }
}