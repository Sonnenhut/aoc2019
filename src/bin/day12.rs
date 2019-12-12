use aoc2019::read_lines;

fn main() {
    let mem: Vec<Planet> = parse(&read_lines(12));
    println!("pt1: {}", pt1(&mem,1000)); // 6423
    println!("pt2: {}", pt2(&mem)); // 327636285682704
}

fn pt1(planets: &Vec<Planet>, steps: isize) -> isize{
    let mut  p_mut = planets.to_vec();
    for _ in 0..steps {
        step(&mut p_mut)
    }
    p_mut.iter().map(Planet::energy).sum::<isize>()
}

fn pt2(planets: &Vec<Planet>) -> isize {
    let mut p_mut = planets.to_vec();
    let mut axis_cycle_times: Vec<Option<isize>> = vec![None, None, None];
    let mut cycle_cnt = 1;
    while axis_cycle_times.contains(&None) {
        step(&mut p_mut);

        if planets.iter().zip(p_mut.iter()).all(|(a, b)| a.pos.x == b.pos.x && a.vel.x == b.vel.x) {
            axis_cycle_times[0] = axis_cycle_times[0].or(Some(cycle_cnt));
        }
        if planets.iter().zip(p_mut.iter()).all(|(a, b)| a.pos.y == b.pos.y && a.vel.y == b.vel.y) {
            axis_cycle_times[1] = axis_cycle_times[1].or(Some(cycle_cnt));
        }
        if planets.iter().zip(p_mut.iter()).all(|(a, b)| a.pos.z == b.pos.z && a.vel.z == b.vel.z) {
            axis_cycle_times[2] = axis_cycle_times[2].or(Some(cycle_cnt));
        }
        cycle_cnt += 1;
    }
    lcm(axis_cycle_times[0].unwrap(), lcm(axis_cycle_times[1].unwrap(), axis_cycle_times[2].unwrap()))
}

fn step(planets: &mut Vec<Planet>) {
    for split_idx in 1..planets.len() {
        let (left, right) = planets.split_at_mut(split_idx);
        let p1 = left.last_mut().unwrap();
        for p2 in right {
            p1.mod_gravity_from(p2);
            p2.mod_gravity_from(p1);
        }
    }
    for p in planets { p.apply_velocity(); }
}

fn parse(input: &Vec<String>) -> Vec<Planet> {
    input.iter().map(|line| Planet {pos: Coord::new(line),vel: Coord {x:0,y:0,z:0}}).collect()
}

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
struct Coord { x: isize, y: isize, z: isize}

#[derive(Debug, Clone, PartialOrd, PartialEq, Ord, Eq, Hash)]
struct Planet { pos: Coord, vel: Coord }

impl Coord {
    fn new(s: &String) -> Coord {
        let parsed : Vec<isize> = s.split(['=',',','>'].as_ref()).map(|s| s.trim())
            .filter_map(|part| part.parse().ok())
            .collect();
        Coord {x: parsed[0], y: parsed[1], z: parsed[2]}
    }
    fn energy(&self) -> isize {
        vec![self.x,self.y,self.z].iter().map(|n|n.abs()).sum()
    }
}

impl Planet {
    fn mod_gravity_from(&mut self, other: &Planet) {
        self.vel.x += self.pos.x.cmp(&other.pos.x) as isize * -1;
        self.vel.y += self.pos.y.cmp(&other.pos.y) as isize * -1;
        self.vel.z += self.pos.z.cmp(&other.pos.z) as isize * -1;
    }
    fn apply_velocity(&mut self) {
        self.pos.x += self.vel.x;
        self.pos.y += self.vel.y;
        self.pos.z += self.vel.z;
    }
    fn energy(&self) -> isize {
        self.pos.energy() * self.vel.energy()
    }
}

fn gcd(mut m: isize, mut n: isize) -> isize {
    while m != 0 {
        let old_m = m;
        m = n % m;
        n = old_m;
    }
    n.abs()
}

fn lcm(a: isize, b: isize) -> isize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod test {
    use super::*;

    fn planet_from_str(s: &String) -> Planet{
        let coords : Vec<Coord> = s.replace("pos=","S").replace(", vel=", "S")
            .split('S').filter(|s| s.trim().len() > 0)
            .map(|c| Coord::new(&String::from(c)))
            .collect();
        Planet {pos: coords[0].clone(), vel: coords[1].clone()}
    }

    #[test]
    fn test_parse_coord() {
        assert_eq!(Coord::new(&String::from("<x=14, y=4, z=5>")), Coord{x: 14, y: 4, z: 5});
        assert_eq!(Coord::new(&String::from("<x=1, y=-2, z=-3>")), Coord{x: 1, y: -2, z: -3});
    }

    #[test]
    fn test_step() {
        let ex =
            "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        let mut  planets: Vec<Planet> = parse(&ex.split('\n').map(String::from).collect());
        let mut expected : Vec<Planet>= vec![
            "pos=<x= 2, y=-1, z= 1>, vel=<x=3, y=-1, z=-1>",
            "pos=<x= 3, y=-7, z=-4>, vel=<x=1, y=3, z= 3>",
            "pos=<x= 1, y=-7, z= 5>, vel=<x=-3, y= 1, z=-3>",
            "pos=<x= 2, y= 2, z= 0>, vel=<x=-1, y=-3, z= 1>"
        ].iter().map(|s| planet_from_str(&String::from(*s))).collect();
        step(&mut planets);
        assert_eq!(planets, expected);

        expected = vec![
            "pos=<x= 5, y=-3, z=-1>, vel=<x= 3, y=-2, z=-2>",
            "pos=<x= 1, y=-2, z= 2>, vel=<x=-2, y= 5, z= 6>",
            "pos=<x= 1, y=-4, z=-1>, vel=<x= 0, y= 3, z=-6>",
            "pos=<x= 1, y=-4, z= 2>, vel=<x=-1, y=-6, z= 2>"
        ].iter().map(|s| planet_from_str(&String::from(*s))).collect();
        step(&mut planets);
        assert_eq!(planets, expected);

        expected = vec![
            "pos=<x= 5, y=-6, z=-1>, vel=<x= 0, y=-3, z= 0>",
            "pos=<x= 0, y= 0, z= 6>, vel=<x=-1, y= 2, z= 4>",
            "pos=<x= 2, y= 1, z=-5>, vel=<x= 1, y= 5, z=-4>",
            "pos=<x= 1, y=-8, z= 2>, vel=<x= 0, y=-4, z= 0>"
        ].iter().map(|s| planet_from_str(&String::from(*s))).collect();
        step(&mut planets); // 3rd step
        assert_eq!(planets, expected);

        step(&mut planets);
        step(&mut planets);
        step(&mut planets);
        step(&mut planets);
        step(&mut planets);
        step(&mut planets); // 9th step

        expected = vec![
            "pos=<x= 2, y= 1, z=-3>, vel=<x=-3, y=-2, z= 1>",
            "pos=<x= 1, y=-8, z= 0>, vel=<x=-1, y= 1, z= 3>",
            "pos=<x= 3, y=-6, z= 1>, vel=<x= 3, y= 2, z=-3>",
            "pos=<x= 2, y= 0, z= 4>, vel=<x= 1, y=-1, z=-1>"
        ].iter().map(|s| planet_from_str(&String::from(*s))).collect();
        step(&mut planets); // 10th step
        assert_eq!(planets, expected);
        assert_eq!(planets.iter().map(|p|p.energy()).sum::<isize>(), 179);
    }
    #[test]
    fn test_pt1() {
        let ex =
            "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        let planets: Vec<Planet> = parse(&ex.split('\n').map(String::from).collect());
        assert_eq!(pt1(&planets,10), 179);
    }

    #[test]
    fn test_pt2() {
        let mut ex =
            "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        let mut planets: Vec<Planet> = parse(&ex.split('\n').map(String::from).collect());
        assert_eq!(pt2(&planets), 2772);


        ex =
"<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

        planets = parse(&ex.split('\n').map(String::from).collect());
        assert_eq!(pt2(&planets), 4686774924);
    }
}