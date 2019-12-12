use std::ops::RangeInclusive;
use std::thread;
use std::thread::JoinHandle;

fn main() {
    println!("pt1: {}", pt1(168630..=718098)); // 1686
    println!("pt2: {}", pt2(168630..=718098)); // 1145
}
fn pt1(range: RangeInclusive<usize>) -> usize {
    chunked_in_threads(range, |rng|
        rng.iter().filter(|n| valid_pw_1(**n)).count() as usize)
}
fn pt2(range: RangeInclusive<usize>) -> usize {
    chunked_in_threads(range, |rng|
        rng.iter().filter(|n| valid_pw_2(**n)).count() as usize)
}
// chunks given range and processes each chunk in a separate thread
// this way the runtime goes from ~4s to ~500ms
// there may be smarter approaches (like skipping number ranges), but in this case: more threads suffice
fn chunked_in_threads(range: RangeInclusive<usize>, func: fn(Vec<usize>) -> usize) -> usize {
    let all : Vec<usize> = range.collect();
    let handles: Vec<JoinHandle<usize>> = all.chunks(10_000)
        .into_iter()
        .map(|part_range|  {
            let rng = part_range.to_vec();
            thread::spawn( move || {
                func(rng)
            })
        })
        .collect();
    handles.into_iter().map(|h| h.join().unwrap().clone()).sum()
}
fn valid_pw_1(p: usize) -> bool {
    let p_str : String = p.to_string();
    let nums : Vec<usize> = p_str.chars().filter_map(|c| c.to_digit(10)).map(|n| n as usize).collect();
    p <= 999_999
        && nums.windows(2).any(|win| win[0] == win[1]) // any two have to be same
        && nums.windows(2).all(|win| win[0] <= win[1])
}
fn valid_pw_2(p: usize) -> bool {
    let p_str : String = p.to_string();
    let nums : Vec<usize> = p_str.chars().filter_map(|c| c.to_digit(10)).map(|n| n as usize).collect();
    p <= 999_999
        && nums.windows(2).all(|win| win[0] <= win[1])
        && contains_two_adjacent(&nums) // at least one pair of two adjacent value have to be present
}

fn contains_two_adjacent(nums: &Vec<usize>) -> bool { // contains exactly two adjacent (at least one time)
    let adjacents : Vec<Vec<usize>> = nums.windows(2).filter(|win| win[0] == win[1])
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
    fn regression() {
        assert_eq!(pt1(168630..=718098), 1686);
        assert_eq!(pt2(168630..=718098), 1145);
    }
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