use std::collections::HashMap;


use aoc2019::read_lines;
use std::hash::Hash;
use std::iter::successors;
use std::cmp::Ordering;
use std::ops::RangeInclusive;

fn main() {
    let reactions = parse(read_lines(14));
    println!("pt1: {}", pt1(&reactions)); // 337075
    println!("pt2: {}", pt2(&reactions)); // 5194174
}

fn pt1(reactions: &HashMap<Chem, Vec<Chem>>) -> i64 {
    explode(Chem::new(1,&"FUEL".to_string()), &reactions)
}

fn pt2(reactions: &HashMap<Chem, Vec<Chem>>) -> i64 {
    let ore_for_one_fuel = pt1(&reactions);
    let trillion = 1000000000000;
    let possible_fuels: Vec<i64> = (trillion/ore_for_one_fuel..=trillion/ore_for_one_fuel*2).collect();

    let res = possible_fuels.binary_search_by(|fuel| {
        let ore_needed = explode(Chem::new(*fuel,&"FUEL".to_string()), &reactions);
        let target_range = (trillion-ore_for_one_fuel..=trillion);
        if target_range.contains(&ore_needed) {
            Ordering::Equal
        } else if ore_needed < *target_range.start() {
            Ordering::Less
        } else if *target_range.end() < ore_needed {
            Ordering::Greater
        } else {panic!("unexpected idx not covered")}
    });
    possible_fuels[res.ok().unwrap_or(0)] as i64
}


fn explode(c: Chem, reactions: &HashMap<Chem, Vec<Chem>>) -> i64 {
    let mut rest : HashMap<String, i64> = HashMap::new();

    let mut res : Vec<Chem> = vec![c];
    let mut i = 0;
    while !(res[0].name == String::from("ORE") && res.len() == 1) {
        res = res.iter_mut()
            .flat_map(|chem| {
                let existing_rest = *rest.get(&chem.name).unwrap_or(&0);
                if existing_rest > 0 {
                    let sub = existing_rest.min(chem.amount);
                    chem.amount -= sub;
                    rest.insert(chem.name.clone(), existing_rest - sub);
                }
                if chem.amount == 0 {
                    return vec![]
                }
                return if let Some((chem_yield, needed)) = find_reaction(&chem.name, &reactions) {
                    let amount_to_create = next_highest_multiple(chem.amount, chem_yield.amount);
                    let modulo = amount_to_create - chem.amount;
                    let multiply = (amount_to_create) / chem_yield.amount;
                    let xploded : Vec<Chem> = needed.clone().iter_mut().map(|needed_chem| Chem::new(needed_chem.amount * multiply, needed_chem.name.as_ref())).collect();
                    if modulo > 0 {
                        rest.insert(chem.name.clone(), rest.get(&chem.name).unwrap_or(&0) + modulo);
                    }
                    xploded
                } else {
                    vec![chem.clone()]
                }
            }).collect();
        res = flatten(&res);
    }
    res[0].amount
}

fn flatten(v: &Vec<Chem>) -> Vec<Chem>{
    v.iter()
        .fold(vec![],|mut acc,next| {
            if let Some(existing) = acc.iter_mut().filter(|other| other.name == next.name).next() {
                existing.amount += next.amount;
            } else {
                acc.push(next.clone());
            }
            acc
        })
}
fn find_reaction(chem_name: &String, reactions: &HashMap<Chem, Vec<Chem>>) -> Option<(Chem, Vec<Chem>)> {
    reactions.iter().filter(|(other, _)| other.name == *chem_name)
        .map(|other| (other.0.clone(), other.1.clone()))
        .next()
}

fn parse(input: Vec<String>) -> HashMap<Chem, Vec<Chem>> {
    input.iter().map(|line| {
        let mut left_right : Vec<&str> = line.split(" => ").collect();
        let needed_chem = parse_chem(&left_right[0].into());
        let res_chem = parse_chem(&left_right[1].into()).remove(0);
        (res_chem,needed_chem)
    }).collect()
}

fn parse_chem(halve_line: &String) -> Vec<Chem> {
    halve_line.split(", ")
        .map(|part| {
            let amount_split : Vec<&str> = part.split(' ').collect();
            Chem { amount: amount_split[0].parse().unwrap(), name:  amount_split[1].to_string() }
        }).collect()
}

fn next_highest_multiple(n: i64, multiple: i64) -> i64 {
    if multiple == 0 {
        return n;
    }

    let remainder = n % multiple;
    if remainder == 0 {
        return n;
    }

    return n + multiple - remainder;
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
struct Chem {amount: i64,name:  String}

impl Chem {
    fn new(amount: i64, name: &str) -> Chem{
        Chem {amount, name: name.to_string()}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let mut ex =
            "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
        let mut expected = HashMap::new();
        expected.insert(Chem:: new(10, "A"), vec![Chem:: new(10, "ORE")]);
        expected.insert(Chem:: new(1, "B"), vec![Chem:: new(1, "ORE")]);
        expected.insert(Chem:: new(1, "C"), vec![Chem:: new(7, "A"), Chem:: new(1, "B")]);
        expected.insert(Chem:: new(1, "D"), vec![Chem:: new(7, "A"), Chem:: new(1, "C")]);
        expected.insert(Chem:: new(1, "E"), vec![Chem:: new(7, "A"), Chem:: new(1, "D")]);
        expected.insert(Chem:: new(1, "FUEL"), vec![Chem:: new(7, "A"), Chem:: new(1, "E")]);

        assert_eq!(parse(ex.lines().map(String::from).collect()), expected);
    }

    #[test]
    fn test_flatten() {
        let unflattened  = vec![Chem:: new(1, "A"), Chem:: new(2, "A")];
        let expected  = vec![Chem:: new(3, "A")];
        assert_eq!(flatten(&unflattened), expected);
    }
    #[test]
    fn test_next_highest_multiple() {
        assert_eq!(next_highest_multiple(2,4), 4);
        assert_eq!(next_highest_multiple(23,3), 24);
        assert_eq!(next_highest_multiple(15,3), 15);
    }
    #[test]
    fn test_parts() {
        let mut ex =
            "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        let mut reactions = parse(ex.lines().map(String::from).collect());
        assert_eq!(pt1(&reactions), 13312);
        assert_eq!(pt2(&reactions), 82892753);

        ex =
            "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
        reactions = parse(ex.lines().map(String::from).collect());
        assert_eq!(pt1(&reactions), 180697);
        assert_eq!(pt2(&reactions), 5586022 );

        ex =
            "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        reactions = parse(ex.lines().map(String::from).collect());
        assert_eq!(pt1(&reactions), 2210736);
        assert_eq!(pt2(&reactions), 460664  );
    }
}