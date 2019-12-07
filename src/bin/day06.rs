use std::collections::HashMap;
use std::iter::successors;

use aoc2019::read_lines;

fn main() {
    let lines: Vec<String> = read_lines(6);
    println!("pt1: {}", checksum(&orbitalMap(&lines))); // 224901
}


fn checksum(orbits: &HashMap<String, String>) -> u32{
    orbits.keys()
        .into_iter()
        .map(|elem|  successors(orbits.get(elem), |name| orbits.get(*name)).count() as u32)
        .sum()
}

fn orbitalMap(input: &Vec<String>) -> HashMap<String, String> {
    input.iter()
        .map(|s| s.split(")"))
        .map(|mut kv| (kv.next().unwrap().into(), kv.next().unwrap().into()))
        .map(|tuple| (tuple.1, tuple.0))
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex() {
        let example: Vec<String> = vec![
            "COM)B",
            "B)C",
            "C)D",
            "D)E",
            "E)F",
            "B)G",
            "G)H",
            "D)I",
            "E)J",
            "J)K",
            "K)L"].into_iter().map(String::from).collect();
        let orbits = orbitalMap(&example);
        assert_eq!(orbits["B"], "COM");
        assert_eq!(orbits["C"], "B");
        assert_eq!(orbits["D"], "C");

        assert_eq!(checksum(&orbits), 42);
    }
}