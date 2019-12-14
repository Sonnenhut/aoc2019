use std::collections::HashMap;


use aoc2019::read_lines;
use std::hash::Hash;
use std::iter::successors;

fn main() {
    //let mem: Vec<i64> = read_lines(14)[0].split(',').map(|s| s.parse().unwrap()).collect();
    //println!("pt1: {}", pt1(&mem)); //
    //println!("pt2: {}", pt2(&mem, true)); //
}


fn pt1(reactions: HashMap<String, Vec<Chem>>) -> usize{
0
}


fn explode(to_explode: &Vec<Chem>, reactions: &HashMap<Chem, Vec<Chem>>) {
    let mut rest : HashMap<String, i64> = HashMap::new();

    let mut res : Vec<Chem> = to_explode.to_vec();
    let mut i = 0;
    while res.len() > 1 {
        res = res.iter_mut()
            .flat_map(|chem| {
                let existing_rest = *rest.get(&chem.name).unwrap_or(&0);
                if existing_rest > 0 {
                    println!("chem: {:?} rest exists{:?}", chem, existing_rest);
                    let sub = (chem.amount - existing_rest.max(chem.amount));

                    chem.amount -= sub;
                    rest.insert(chem.name.clone(), existing_rest - sub);
                }
                if chem.amount == 0 {
                    return vec![]
                }
                return if let Some((chem_yield, needed)) = find_reaction(&chem.name, &reactions) {
                    let amount_to_create = next_highest_multiple(chem.amount, chem_yield.amount);
                    let modulo = amount_to_create % chem_yield.amount;
                    let multiply = (amount_to_create) / chem_yield.amount;
                    let xploded : Vec<Chem> = needed.clone().iter_mut().map(|needed_chem| Chem::new(needed_chem.amount * multiply, needed_chem.name.as_ref())).collect();
                    if modulo > 0 {
                        rest.insert(chem.name.clone(), rest.get(&chem.name).unwrap_or(&0) + modulo);
                    }
                    println!("chem: {:?} reaction into: {:?}; rest: {:?}", chem, xploded, modulo);
                    xploded
                } else {
                    vec![chem.clone()]
                }
            }).collect();
        res = flatten(&res);
        println!("==> {:?}; rest: {:?}", res, rest);
        i += 1;
        if i== 10 {
            break;
        }
    }
    //(vec![], vec![])
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

        //assert_eq!(flatten_ore(&expected), 31);
/*
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
*/
        //assert_eq!((23 as f64 / 3 as f64).ceil() as usize, 8);
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
    fn test_explode() {

        /*

        let ex =
"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        let reactions = parse(ex.lines().map(String::from).collect());
        explode(&vec![Chem::new(2,"AB"),Chem::new(3,"BC"),Chem::new(4,"CA")], &reactions);
        */


        let ex =
"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        let reactions = parse(ex.lines().map(String::from).collect());
        explode(&find_reaction(&"FUEL".to_string(), &reactions).unwrap().1, &reactions);

        //assert_eq!(explode(&Chem::new(2,"AB"), &reactions), vec![Chem::new(6,"A"), Chem::new(8,"B")]);
        //assert_eq!(explode(&Chem::new(3,"A"), &reactions), vec![Chem::new(18,"ORE"), Chem::new(1,"A")]);
    }
}



/*
fn flatten_ore(reactions: &HashMap<Chem, Vec<Chem>>) -> usize {
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
    println!("res => {:?}", needed);
    needed[0].0 as usize
}


fn explode_once(v: &Vec<Chem>, exact: bool, reactions: &HashMap<Chem, Vec<Chem>>) -> Vec<Chem> {
    let with_dupes = v.iter()
        .flat_map( |need| {
            let mut res = vec![need.clone()];

            if let Some((Chem(need_react_amount, _),react)) = find_reaction(&need.1, &reactions) {
                println!("possible reaction {:?} => {:?}", need.1, react);
                // only resolve, when possible without loss
                if need.0 % need_react_amount == 0 { // exact (without loss)
                    let times = need.0 / need_react_amount;
                    res = react.into_iter().map(move |r| Chem(r.0 * times, r.1.to_string())).collect();
                    println!("used {:?} => {:?}", need, res);
                } else if !exact {
                    let times = (need.0 as f64 / need_react_amount as f64).ceil() as i64;
                    res = react.into_iter().map(move |r| Chem(r.0 * times, r.1.to_string())).collect();
                    println!("used {:?} => {:?}", need, res);
                }
            }
            res
        }).collect::<Vec<Chem>>();
    flatten(&with_dupes)
}
*/