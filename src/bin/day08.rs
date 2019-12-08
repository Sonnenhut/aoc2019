
use aoc2019::intcode::IntCode;
use aoc2019::read_lines;

fn main() {
    let layers: Vec<Vec<String>> = split_layers(25,6,read_lines(8)[0].clone());
    println!("pt1: {}", checksum(&layers)); // 1820
    //println!("pt2: {}", max_feedback_loop(&nums)); //
}

fn checksum(layers: &Vec<Vec<String>>) -> usize {
    let zcnt_layers: Vec<(usize, Vec<String>)> = layers
        .iter()
        .map(|layer| {
            let zcnt = layer.iter().map(|row| cnt('0', row)).sum::<usize>();
            (zcnt, layer.clone())
        })
        .collect();
    let min0cnt = zcnt_layers.iter().map(|tuple| tuple.0).min().unwrap();
    let layer = zcnt_layers.iter()
        .filter(|tuple| tuple.0 == min0cnt)
        .map(|tuple| tuple.1.clone())
        .next()
        .unwrap();
    let onecnt :usize = layer
        .iter()
        .map(|row| cnt('1', row))
        .sum();
    let twocnt:usize = layer
        .iter()
        .map(|row| cnt('2', row))
        .sum();
    onecnt * twocnt
}

fn cnt(search: char, s: &String) -> usize {
    let res= s.chars()
        .filter(|c| *c == search)
        .count();
    res
}

fn split_layers(w: usize, h: usize, image: String) -> Vec<Vec<String>> {
    let s = image.to_string();
    let rows : Vec<String> = s.chars()
        .collect::<Vec<char>>()
        .chunks(w)
        .map(|row| row.iter().collect())
        .collect();

    rows.chunks(h)
        .map(|layer| layer.to_vec())
        .collect()
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_ex() {
        let  layers = split_layers(3,2,String::from("123456789012"));
        assert_eq!(layers, vec![vec!["123","456"],vec!["789","012"]]);
    }

}