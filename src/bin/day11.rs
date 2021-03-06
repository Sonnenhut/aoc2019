use std::collections::HashMap;

use aoc2019::intcode::{IntCode, IntCodeClient};
use aoc2019::read_lines;
use std::time::Duration;

fn main() {
    let mem: Vec<i64> = read_lines(11)[0].split(',').map(|s| s.parse().unwrap()).collect();
    println!("pt1: {}", pt1(&mem,0).len()); // 2539
    println!("pt2: ", ); // ZLEBKJRA
    pt2(&mem);
}

fn pt1(mem: &Vec<i64>, start_panel: i64) -> HashMap<(i64,i64),i64> {
    let IntCodeClient {snd , rcv, idle: _} = IntCode::run_async(&mem);
    let mut colors : HashMap<(i64,i64), i64> = HashMap::new();
    let mut loc : (i64,i64,char) = (0,0,'^');


    let mut last_tile = Some(start_panel);
    while let Some(tile_color) = last_tile {
        let _ = snd.send(tile_color);
        let new_tile = rcv.recv();

        last_tile = if new_tile.is_ok() {
            colors.insert((loc.0, loc.1),new_tile.unwrap());

            let direction = rcv.recv().unwrap();
            loc = mod_loc(&loc, direction);
            colors.get(&(loc.0,loc.1)).cloned().or(Some(0))
        } else {None}
    }

    colors
}

fn pt2(mem: &Vec<i64>) {
    let colors = pt1(&mem,1);
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
    use super::*;


    #[test]
    fn regression() {
        let mem: Vec<i64> = read_lines(11)[0].split(',').map(|s| s.parse().unwrap()).collect();
        assert_eq!(pt1(&mem,0).len(), 2539);
        pt2(&mem);// ZLEBKJRA
    }

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
        let ex_mem = vec![1,0,0,0,1,0,1,0,0,1,1,0,1,0,99].iter()
            .flat_map(|n| if *n == 1 {vec![1104,1]} else if *n == 0 {vec![1104,0]} else {vec![99]})
            .collect();

        let mut expected = HashMap::new();
        expected.insert((0,0),0);
        expected.insert((-1,0),0);
        expected.insert((-1,-1),1);
        expected.insert((0,-1),1);
        expected.insert((1,0),1);
        expected.insert((1,1),1);

        assert_eq!(pt1(&ex_mem,0), expected);
        assert_eq!(pt1(&ex_mem,0).len(), 6);
    }
}
