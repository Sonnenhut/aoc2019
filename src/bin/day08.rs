use aoc2019::read_lines;
use std::iter::repeat;

fn main() {
    let layers: Vec<Vec<String>> = split_layers(25,6,read_lines(8)[0].clone());
    println!("pt1: {}", checksum(&layers)); // 1820
    println!("pt2: {}", merge(&layers).join("")); // ZUKCJ
    merge(&layers).iter()
        .map(|s| s.replace("0"," "))
        .map(|s| s.replace("1","█"))
        .inspect(|row| println!("{:?}", row)).collect::<Vec<String>>().join("");
}

fn merge(layers: &Vec<Vec<String>>) -> Vec<String> {
    let width = layers[0][0].len();
    let height = layers[0].len();
    let transparent_row: String = repeat("2").take(width).collect::<Vec<&str>>().join("");
    let res : Vec<String> = layers.iter().fold(vec![transparent_row; height], |acc, layer| {
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
    let (_,layer) = layers.iter()
        .map(|layer| (cnt_all('0',&layer), layer.clone()))
        .min_by(|a, b|a.0.cmp(&b.0)).unwrap();
    cnt_all('1', &layer) * cnt_all('2', &layer)
}

fn cnt_all(search: char, layer: &Vec<String>) -> usize {
    layer.iter().map(|row| row.chars().filter(|c| *c == search).count()).sum()
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


        layers = split_layers(2,2,String::from("0222112222120000"));
        assert_eq!(merge(&layers), vec![String::from("01"), String::from("10")]);
    }

}