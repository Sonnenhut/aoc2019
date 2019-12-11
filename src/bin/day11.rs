use std::collections::HashMap;

use aoc2019::intcode::IntCode;
use aoc2019::read_lines;

fn main() {
    let nums: Vec<i64> = read_lines(11)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", pt1(&nums,0).len()); // 2539
    println!("pt2: ", ); // ZLEBKJRA
    pt2(&nums);
}

fn pt1(nums: &Vec<i64>, start_panel: i64) -> HashMap<(i64,i64),i64> {
    let mut int_code = IntCode::create(&vec![], &nums);
    let mut colors : HashMap<(i64,i64), i64> = HashMap::new();
    let mut loc : (i64,i64,char) = (0,0,'^');

    let mut optional = Some(start_panel);
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

fn pt2(nums: &Vec<i64>) {
    let colors = pt1(&nums,1);
    let min_x = *colors.iter().map(|((x,_), _)| x).min().unwrap_or(&0);
    let max_x = *colors.iter().map(|((x,_), _)| x).max().unwrap_or(&0);
    let min_y = *colors.iter().map(|((_,y), _)| y).min().unwrap_or(&0);
    let max_y = *colors.iter().map(|((_,y), _)| y).max().unwrap_or(&0);
    for y in (min_y..=max_y).rev() {
        for x  in min_x..max_x {
            match colors.get(&(x as i64,y as i64)).or(Some(&0_i64)).unwrap() {
                1 => print!("█"),
                0 => print!(" "),
                _ => {}
            }
        }
        print!("\n");
    }
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
    use std::process::exit;

    use super::*;

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

        let mut expected = HashMap::new();
        expected.insert((0,0),0);
        expected.insert((-1,0),0);
        expected.insert((-1,-1),1);
        expected.insert((0,-1),1);
        expected.insert((1,0),1);
        expected.insert((1,1),1);

        assert_eq!(pt1(&ex_pgm,0), expected);
        assert_eq!(pt1(&ex_pgm,0).len(), 6);
    }
}