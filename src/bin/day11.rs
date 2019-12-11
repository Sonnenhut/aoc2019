use aoc2019::read_lines;
use std::iter::successors;
use std::cmp::Ordering::Equal;
use aoc2019::intcode::IntCode;
use std::collections::HashMap;
use std::hash::Hash;

fn main() {
    let nums: Vec<i64> = read_lines(11)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", pt1(&nums).len()); // 2539
    //println!("pt2: {}", pt2(&input)); //
}

fn pt1(nums: &Vec<i64>) -> HashMap<(i64,i64),i64> {
    let mut int_code = IntCode::create(&vec![], &nums);
    let mut colors : HashMap<(i64,i64), i64> = HashMap::new();
    let mut loc : (i64,i64,char) = (0,0,'^');

    let mut optional = Some(0_i64);
    while let Some(panel_color) = optional {
        int_code.push_input(panel_color as i64);
        let new_color = int_code.next();

        optional = if new_color.is_some() {
            colors.insert((loc.0, loc.1),new_color.unwrap());

            let direction = int_code.next().unwrap();
            loc = mod_loc(&loc, direction);
            colors.get(&(loc.0,loc.1)).cloned().or(Some(0))
        } else {
            None
        }
    }
    colors
}

fn mod_loc(loc : &(i64,i64,char), modifier: i64) -> (i64,i64,char) {
    match loc.2 {
        '^' => if modifier == 0 { (loc.0 - 1, loc.1,'<')} else {(loc.0 + 1, loc.1,'>')},
        'v' => if modifier == 0 { (loc.0 + 1, loc.1,'>')} else {(loc.0 - 1, loc.1,'<')},
        '<' => if modifier == 0 { (loc.0, loc.1 - 1,'v')} else {(loc.0 , loc.1 + 1,'^')},
        '>' => if modifier == 0 { (loc.0, loc.1 + 1,'^')} else {(loc.0 , loc.1 - 1,'v')}
        _ => {panic!("Cannot modifiy location")}
    }
}


#[cfg(test)]
mod test {
    use super::*;
    use std::process::exit;

    #[test]
    fn test_mod_loc() {
        assert_eq!(mod_loc(&(0,0,'^'),0), (-1,0,'<'));
        assert_eq!(mod_loc(&(0,0,'^'),1), (1,0,'>'));
        assert_eq!(mod_loc(&(0,0,'<'),0), (0,-1,'v'));
        assert_eq!(mod_loc(&(0,0,'<'),1), (0,1,'^'));
        assert_eq!(mod_loc(&(0,0,'>'),0), (0,1,'^'));
        assert_eq!(mod_loc(&(0,0,'>'),1), (0,-1,'v'));
        assert_eq!(mod_loc(&(0,0,'v'),0), (1,0,'>'));
        assert_eq!(mod_loc(&(0,0,'v'),1), (-1,0,'<'));
    }

    #[test]
    fn test_ex() {
        let ex_pgm = vec![1,0,0,0,1,0,1,0,0,1,1,0,1,0,99].iter()
            .flat_map(|n| if *n == 1 {vec![1104,1]} else if *n == 0 {vec![1104,0]} else {vec![99]})
            .collect();

        let mut expected =HashMap::new();
        expected.insert((0,0),0);
        expected.insert((-1,0),0);
        expected.insert((-1,-1),1);
        expected.insert((0,-1),1);
        expected.insert((1,0),1);
        expected.insert((1,1),1);

        assert_eq!(pt1(&ex_pgm), expected);
        assert_eq!(pt1(&ex_pgm).len(), 6);
    }
}