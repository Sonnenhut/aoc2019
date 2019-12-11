
fn main() {
    println!("pt1: {}", pt1(168630,718098)); // 1686
    println!("pt2: {}", pt2(168630, 718098)); // 1145
}
fn pt1(from: u32, to: u32) -> u32 {
    (from..=to).filter(|n| valid_pw_1(*n)).collect::<Vec<u32>>().len() as u32
}
fn pt2(from: u32, to: u32) -> u32 {
    (from..=to).filter(|n| valid_pw_2(*n)).collect::<Vec<u32>>().len() as u32
}
fn valid_pw_1(p: u32) -> bool {
    let p_str : String = p.to_string();
    let nums : Vec<u32> = p_str.chars().filter_map(|c| c.to_digit(10)).collect();
    p <= 999_999
        && nums.windows(2).any(|win| win[0] == win[1]) // any two have to be same
        && nums.windows(2).all(|win| win[0] <= win[1])
}
fn valid_pw_2(p: u32) -> bool {
    let p_str : String = p.to_string();
    let nums : Vec<u32> = p_str.chars().filter_map(|c| c.to_digit(10)).collect();
    p <= 999_999
        && contains_two_adjacent(&nums) // at least one pair of two adjacent value have to be present
        && nums.windows(2).all(|win| win[0] <= win[1])
}

fn contains_two_adjacent(nums: &Vec<u32>) -> bool { // contains exactly two adjacent (at least one time)
    let adjacents : Vec<Vec<u32>> = nums.windows(2).filter(|win| win[0] == win[1])
                                        .map(|win| win.to_vec())
                                        .collect();
    adjacents.iter().any(|win| {
        adjacents.iter().filter(|other| other[0] == win[0]).count() == 1
    }) // at least one tuple has to be present
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_pw_1() {
        assert_eq!(valid_pw_1(111111), true);
        assert_eq!(valid_pw_1(112345), true);
        assert_eq!(valid_pw_1(112389), true);
        assert_eq!(valid_pw_1(223450), false);
        assert_eq!(valid_pw_1(123789), false);
    }
    #[test]
    fn test_valid_pw_2() {
        assert_eq!(valid_pw_2(112233), true);
        assert_eq!(valid_pw_2(123444), false);
        assert_eq!(valid_pw_2(111122), true);
        assert_eq!(valid_pw_2(112244), true);
        assert_eq!(valid_pw_2(114444), true);
        assert_eq!(valid_pw_2(111111), false);
        assert_eq!(valid_pw_2(111189), false);
    }
}