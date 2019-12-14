use std::collections::HashMap;


use aoc2019::read_lines;
use std::hash::Hash;

fn main() {
    //let mem: Vec<i64> = read_lines(14)[0].split(',').map(|s| s.parse().unwrap()).collect();
    //println!("pt1: {}", pt1(&mem)); //
    //println!("pt2: {}", pt2(&mem, true)); //
}


fn pt1(reactions: HashMap<String, Vec<Amount>>) -> usize{
0
}

fn flatten_ore(reactions: &HashMap<Amount, Vec<Amount>>) -> usize {
    let (wanted, all_needed) = find_reaction(&"FUEL".to_string(), &reactions).unwrap();

    let mut i = 0;
    let mut needed = all_needed.clone();
    while needed.len() > 1 {
        let new = explode_once(&needed, true, reactions);
        println!("=> {:?}",  new);
        let unchanged = new.iter().all(|item| needed.contains(&item));
        if unchanged { // unchanged
            needed = explode_once(&new, false, reactions).clone();
            println!("=> {:?}",  needed);
        } else {
            needed = new;
        }
        //println!("{:?}", needed);
        if i == 100 {
            break
        }
        println!();
        i += 1;
    }

    needed[0].0 as usize
}

fn flatten(v: &Vec<Amount>) -> Vec<Amount>{
    v.iter()
        .fold(vec![],|mut acc,next| {
            if let Some(existing) = acc.iter_mut().filter(|other| other.1 == next.1).next() {
                existing.0 += next.0
            } else {
                acc.push(next.clone());
            }
            acc
        })
}

fn explode_once(v: &Vec<Amount>, exact: bool, reactions: &HashMap<Amount, Vec<Amount>>) -> Vec<Amount> {
    let with_dupes = v.iter()
        .flat_map( |need| {
            let mut res = vec![need.clone()];

            if let Some((Amount(need_react_amount,_),react)) = find_reaction(&need.1, &reactions) {
                println!("possible reaction {:?} => {:?}", need.1, react);
                // only resolve, when possible without loss
                if need.0 % need_react_amount == 0 { // exact (without loss)
                    let times = need.0 / need_react_amount;
                    res = react.into_iter().map(move |r| Amount(r.0 * times, r.1.to_string())).collect();
                    println!("used {:?} => {:?}", need, res);
                } else if !exact {
                    let times = (need.0 as f64 / need_react_amount as f64).ceil() as i64;
                    res = react.into_iter().map(move |r| Amount(r.0 * times, r.1.to_string())).collect();
                    println!("used {:?} => {:?}", need, res);
                }
            }
            res
        }).collect::<Vec<Amount>>();
    flatten(&with_dupes)
}

fn find_reaction(chem: &String, reactions: &HashMap<Amount, Vec<Amount>>) -> Option<(Amount, Vec<Amount>)> {
    reactions.iter().filter(|(created, _)| created.1 == *chem)
        .map(|t|(t.0.clone(), t.1.clone()))
        .next()
}

fn parse(input: Vec<String>) -> HashMap<Amount, Vec<Amount>> {
    input.iter().map(|line| {
        let mut left_right : Vec<&str> = line.split(" => ").collect();
        let needed_chem = parse_amount(&left_right[0].into());
        let res_chem = parse_amount(&left_right[1].into()).remove(0);
        (res_chem,needed_chem)
    }).collect()
}

fn parse_amount(halve_line: &String) -> Vec<Amount> {
    halve_line.split(", ")
        .map(|part| {
            let amount_split : Vec<&str> = part.split(' ').collect();
            Amount(amount_split[0].parse().unwrap(), amount_split[1].to_string())
        }).collect()

}

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
struct Amount(i64, String);


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
        expected.insert(Amount(10, "A".to_string()), vec![Amount(10,"ORE".to_string())]);
        expected.insert(Amount(1, "B".to_string()), vec![Amount(1,"ORE".to_string())]);
        expected.insert(Amount(1, "C".to_string()), vec![Amount(7,"A".to_string()),Amount(1,"B".to_string())]);
        expected.insert(Amount(1, "D".to_string()), vec![Amount(7,"A".to_string()),Amount(1,"C".to_string())]);
        expected.insert(Amount(1, "E".to_string()), vec![Amount(7,"A".to_string()),Amount(1,"D".to_string())]);
        expected.insert(Amount(1, "FUEL".to_string()), vec![Amount(7,"A".to_string()),Amount(1,"E".to_string())]);

        assert_eq!(parse(ex.lines().map(String::from).collect()), expected);

        assert_eq!(flatten_ore(&expected), 31);

        ex =
            "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        assert_eq!(flatten_ore(&parse(ex.lines().map(String::from).collect())), 165);

        ex =
"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        assert_eq!(flatten_ore(&parse(ex.lines().map(String::from).collect())), 13312 );

        //assert_eq!((23 as f64 / 3 as f64).ceil() as usize, 8);
    }
    #[test]
    fn test_flatten() {
        let unflattened  = vec![Amount(1,"A".to_string()),Amount(2,"A".to_string())];
        let expected  = vec![Amount(3,"A".to_string())];
        assert_eq!(flatten(&unflattened), expected);
    }
}