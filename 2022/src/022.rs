use std::{
    collections::HashMap,
    fs,
    ops::{Add, AddAssign}
};

const INPUT_PATH: &str = "inputs/_022.txt";
const CUBE_SIZE: i64 = 4;
type Map = HashMap<Point, char>;
type Route = Vec<Step>;
type Cube = HashMap<Point, HashMap<Point, Point>>;

fn main() {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();

    let file_parts = file_str.split("\n\n").collect::<Vec<&str>>();

    let map: Map = file_parts[0].split('\n')
        .enumerate()
        .map(|(y, line)| line.chars()
            .enumerate()
            .map(move |(x, c)| (Point { x: x as i64, y: y as i64 }, c))
        )
        .flatten()
        .filter(|(_, c)| *c == '.' || *c == '#')
        .collect();

    let route = file_parts[1].replace('R', " R ").replace('L', " L ")
        .split(' ')
        .map(|a| match a.parse::<i64>() {
            Ok(n) => Step::Move(n),
            _ if a == "R" => Step::Turn('R'),
            _ if a == "L" => Step::Turn('L'),
            _ => panic!()
        })
        .collect::<Route>();

    let start_y = map.keys().map(|p| p.y).min().unwrap();
    let start = Point {
        x: map.iter()
            .filter(|(p, c)| p.y == start_y && **c == '.')
            .map(|(p, _)| p.x)
            .min()
            .unwrap(),
        y: start_y
    };
    let mut dir = Point { x: 1, y: 0};

    let cube = get_cube();

    let (p, dir) = traverse(&map, &route, start, dir);
    println!("{}", 1000 * (p.y + 1) + 4 * (p.x + 1) + dir_score(dir) );
}

fn traverse(map: &Map, route: &Route, start: Point, start_dir: Point) -> (Point, Point) {
    let mut cur = start;
    let mut dir = start_dir;
    for step in route {
        match step {
            Step::Move(n) => cur = forward(map, cur, dir, *n),
            Step::Turn(c) => dir = rotate(dir, *c)
        }
        println!("S: {:?}, P: {:?}", step, cur);
    }
    (cur, dir)
}

fn forward(map: &Map, p: Point, dir: Point, dist: i64) -> Point {
    let mut cur = p;
    for _ in 0..dist {
        let next = get_next_tile(map, cur, dir);
        if map[&next] == '#' { return cur; }
        cur = next;
    }
    cur
}

fn rotate(dir: Point, c: char) -> Point {
    match c {
        'R' => Point { x: -dir.y, y: dir.x },
        'L' => Point { x: dir.y, y: -dir.x },
        _ => panic!()
    }
}

fn get_next_tile(map: &Map, p: Point, dir: Point, cube: &Cube) -> Point {
    if map.contains_key(&(p+dir)) { return p + dir; }

    let current_side = Point { x: p.x / CUBE_SIZE, y: p.y / CUBE_SIZE };
    
    let next_side = cube[&current_side][&dir];
}

// fn get_next_tile(map: &Map, p: Point, dir: Point) -> Point {
//     if map.contains_key(&(p+dir)) { return p + dir; }
    
//     match dir.x {
//         0 => {
//             let i = map.keys().filter(|a| a.x == p.x).map(|a| a.y);
//             let y = match dir.y {
//                 1 => i.min().unwrap(),
//                 _ => i.max().unwrap()
//             };
//             Point { x: p.x, y }
//         },
//         _ => {
//             let i = map.keys().filter(|a| a.y == p.y).map(|a| a.x);
//             let x = match dir.x {
//                 1 => i.min().unwrap(),
//                 _ => i.max().unwrap()
//             };
//             Point { x, y: p.y }
//         }
//     }
// }

fn dir_score(dir: Point) -> i64 {
    match dir {
        p if p.x == 1 && p.y == 0 => 0,
        p if p.x == 0 && p.y == 1 => 1,
        p if p.x == -1 && p.y == 0 => 2,
        p if p.x == 0 && p.y == -1 => 3,
        _ => panic!(),
    }
}

fn get_cube() -> Cube {
    let mut cube = HashMap::new();

    let c1 = Point { x: 2 * CUBE_SIZE, y: 0 };
    let c2 = Point { x: 0, y: CUBE_SIZE };
    let c3 = Point { x: CUBE_SIZE, y: CUBE_SIZE };
    let c4 = Point { x: 2 * CUBE_SIZE, y: CUBE_SIZE };
    let c5 = Point { x: 2 * CUBE_SIZE, y: 2 * CUBE_SIZE };
    let c6 = Point { x: 3 * CUBE_SIZE, y: 2 * CUBE_SIZE };

    cube.insert(
        c1,
        HashMap::from([
            ( Point { x: -1 , y: 0 }, c3 ),
            ( Point { x: 1 , y: 0 }, c6 ),
            ( Point { x: 0 , y: -1 }, c2 ),
        ])
    );
    cube.insert(
        c2,
        HashMap::from([
            ( Point { x: -1 , y: 0 }, c6 ),
            ( Point { x: 0 , y: -1 }, c1 ),
            ( Point { x: 0 , y: 1 }, c5 ),
        ])
    );
    cube.insert(
        c3,
        HashMap::from([
            ( Point { x: 0 , y: -1 }, c1 ),
            ( Point { x: 0 , y: 1 }, c5 ),
        ])
    );
    cube.insert(
        c4,
        HashMap::from([
            ( Point { x: 1 , y: 0 }, c6 ),
        ])
    );
    cube.insert(
        c5,
        HashMap::from([
            ( Point { x: -1 , y: 0 }, c3 ),
            ( Point { x: 0 , y: 1 }, c2 ),
        ])
    );
    cube.insert(
        c6,
        HashMap::from([
            ( Point { x: 0 , y: -1 }, c4 ),
            ( Point { x: 1 , y: 0 }, c1 ),
            ( Point { x: 0 , y: 1 }, c2 ),
        ])
    );

    cube
}

#[derive(Debug)]
enum Step {
    Turn(char),
    Move(i64)
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    pub x: i64,
    pub y: i64
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        return Point{ x: self.x + other.x, y: self.y + other.y}
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = Self{x: self.x + other.x, y: self.y + other.y};
    }
}