use std::{
    collections::{HashSet, VecDeque},
    fs,
    ops::{Add, Mul}
};

const INPUT_PATH: &str = "inputs/018.txt";

fn main () {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();

    let cubes: HashSet<Point> = file_str.split('\n')
        .map(|l| {
            let coords: Vec<i64> = l.split(',')
                .map(|a| a.parse::<i64>().unwrap())
                .collect();
            Point::new(coords[0], coords[1], coords[2])
        })
        .collect();

    let mut count = 0;

    for cube in cubes.iter() {
        count += 6 - ORTHO_DIRECTIONS.iter()
            .filter(|p| cubes.contains(&(*cube + **p)))
            .count();        
    }
    println!("Rock sides: {}", count);

    let max_x = cubes.iter().map(|p| p.x).max().unwrap();
    let max_y = cubes.iter().map(|p| p.y).max().unwrap();
    let max_z = cubes.iter().map(|p| p.z).max().unwrap();

    let mut air = HashSet::new();
    let mut queue = VecDeque::new();
    let mut outsides = HashSet::new();

    queue.push_back(Point::new(0,0,0));

    while queue.len() > 0 {
        let cur = queue.pop_front().unwrap();

        for dir in ORTHO_DIRECTIONS {
            let p = cur + dir;
            if cubes.contains(&p)  { 
                outsides.insert((dir * -1, p));
                continue;
            };
            if air.contains(&p) { continue };
            if p.x < -1 || p.y < -1 || p.z < -1 || p.x>max_x + 1 || p.y>max_y + 1 || p.z>max_z + 1 { continue; }

            air.insert(p);
            queue.push_back(p);
        }
    }

    println!("Outsides: {}", outsides.iter().count());
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    pub x: i64,
    pub y: i64,
    pub z: i64
}

impl Point {
    pub fn new(x: i64, y: i64, z: i64) -> Point {
        Point { x, y, z }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Point{ 
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl Mul<i64> for Point {
    type Output = Self;

    fn mul(self, other: i64) -> Self {
        return Point::new(self.x * other, self.y * other, self.z * other)
    }
}

const ORTHO_DIRECTIONS: [Point; 6] = [
    Point{ x: -1, y: 0, z: 0 }, Point{ x: 1, y: 0, z: 0 },
    Point{ x: 0, y: -1, z: 0 }, Point{ x: 0, y: 1, z: 0 },
    Point{ x: 0, y: 0, z: -1 }, Point{ x: 0, y: 0, z: 1 }
];