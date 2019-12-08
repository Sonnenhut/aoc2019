use aoc2019::read_lines;
use std::iter::repeat;

fn main() {
    let layers: Vec<Vec<String>> = split_layers(25,6,read_lines(8)[0].clone());
    println!("pt1: {}", checksum(&layers)); // 1820
    println!("pt2: {}", merge(&layers).join("")); // ZUKCJ
    merge(&layers)
        .iter()
        .map(|s| s.replace("0"," "))
        .map(|s| s.replace("1","█"))
        .inspect(|row| println!("{:?}", row)).collect::<Vec<String>>().join("");
}

fn merge(layers: &Vec<Vec<String>>) -> Vec<String> {
    let width = layers[0][0].len();
    let height = layers[0].len();
    let initial : String = repeat("2").take(width).collect::<Vec<&str>>().join("");
    let res : Vec<String> = layers.iter().fold( vec![initial;height], |acc, layer| {
        acc.iter()
            .zip(layer.iter())
            .map(|tuple| merge_row(tuple.0, tuple.1))
            .collect()
    });
    res
}

fn merge_row(above: &str, below: &str) -> String {
    above.chars().zip(below.chars())
        .map(|tuple| if tuple.0 == '2' { tuple.1} else {tuple.0})
        .collect()
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
        let mut layers = split_layers(3,2,String::from("123456789012"));
        assert_eq!(layers, vec![vec!["123","456"],vec!["789","012"]]);
        assert_eq!(as_msg(&vec![String::from("123"),String::from("456")]), "123456");


        layers = split_layers(2,2,String::from("0222112222120000"));
        assert_eq!(merge(&layers), vec![String::from("01"), String::from("10")]);
    }

}