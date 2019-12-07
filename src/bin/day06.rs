use std::collections::HashMap;
use std::iter::successors;

use aoc2019::read_lines;

fn main() {
    let lines: Vec<String> = read_lines(6);
    let orbits = orbitalMap(&lines);
    println!("pt1: {}", checksum(&orbits)); // 224901
    println!("pt2: {}", path_between(String::from("YOU"),String::from("SAN"),&orbits)); // 334
}

fn path_to_COM(origin: String, orbits: &HashMap<String, String>) -> Vec<String> {
    successors(orbits.get(&origin), |name| orbits.get(*name)).cloned().collect()
}

fn path_between(origin: String, target: String, orbits: &HashMap<String, String>) -> usize {
    let origin_path = path_to_COM(origin, orbits);
    let target_path = path_to_COM(target, orbits);

    let intersection : &String = origin_path.iter()
        .filter(|p| target_path.contains(p))
        .next().unwrap();
    let origin_distance = origin_path.iter().position(|other| other == intersection);
    let target_distance = target_path.iter().position(|other| other == intersection);
    [origin_distance, target_distance].into_iter().flatten().sum()
}

fn checksum(orbits: &HashMap<String, String>) -> u32 {
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
    #[test]
    fn test_ex_pt2() {
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
            "K)L",
            "K)YOU",
            "I)SAN"].into_iter().map(String::from).collect();
        let orbits = orbitalMap(&example);
        assert_eq!(orbits["B"], "COM");
        assert_eq!(orbits["C"], "B");
        assert_eq!(orbits["D"], "C");

        assert_eq!(path_to_COM(String::from("YOU"), &orbits), vec!["K","J","E","D","C","B","COM"]);
        assert_eq!(path_to_COM(String::from("SAN"), &orbits), vec!["I","D","C","B","COM"]);

        assert_eq!(path_between(String::from("YOU"),String::from("SAN"), &orbits), 4)
    }
}