use std::{
    collections::{HashSet, VecDeque},
    fs,
    iter::FromIterator,
    ops::{Add, AddAssign}
};

const INPUT_PATH: &str = "inputs/023.txt";

fn main() {
    let lines: Vec<Vec<char>> = fs::read_to_string(INPUT_PATH)
        .unwrap()
        .split('\n')
        .map(|l|
            l.chars().collect()
        )
        .collect();

    let starting_points: HashSet<Point> = lines.iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .flat_map(move |(x, c)| match c {
                    '#' => Some(Point::new(x as i64, y as i64)),
                    _ => None
                })
        })
        .flatten()
        .collect();

    // let new = run(10, &starting_points);

    // let bounds = get_bounds(&new);

    // let a = (bounds[1] - bounds[0] + 1) * (bounds[3] - bounds[2] + 1);
    // println!("{}", a);
    // println!("First: {}", a - new.len() as i64);
    println!("Second: {}", run_2(&starting_points));
}

fn run_2(start: &HashSet<Point>) -> usize {
    let mut queue = VecDeque::from_iter([
        Point::new(0, -1), Point::new(0, 1),
        Point::new(-1, 0), Point::new(1, 0)
    ]);
    let mut cur = start.clone();
    let mut step = 1;
    loop {
        println!("Step: {}", step);
        let next = next_state(&cur, &queue);
        if cur == next { return step; }
        cur = next;
        let q = queue.pop_front().unwrap();
        queue.push_back(q);
        step += 1;
    }
}

fn run(steps: usize, start: &HashSet<Point>) -> HashSet<Point> {
    let mut queue = VecDeque::from_iter([
        Point::new(0, -1), Point::new(0, 1),
        Point::new(-1, 0), Point::new(1, 0)
    ]);
    let mut cur = start.clone();
    for step in 1..=steps {
        cur = next_state(&cur, &queue);
        let q = queue.pop_front().unwrap();
        queue.push_back(q);
    }
    cur
}

fn print_state(state: &HashSet<Point>) {
    let bounds = get_bounds(state);

    for y in bounds[2]..=bounds[3] {
        for x in bounds[0]..=bounds[1] {
            if state.contains(&Point::new(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    print!("\n\n");
}

fn next_state(
    start: &HashSet<Point>,
    dir_queue: &VecDeque<Point>
) -> HashSet<Point> {
    let attempts: Vec<(Point, Point)> = start.iter()
        .map(|p| {
            if !has_neighbours(&start, *p, ALL_DIRECTIONS.to_vec()) {
                return (*p, *p);
            }
            for dir in dir_queue {
                if !has_neighbours(&start, *p, with_diagonals(*dir)) {
                    return (*p + *dir, *p);
                }
            }
            (*p, *p)
        })
        .collect();

    attempts.iter()
        .map(|(target, source)| {
            if attempts.iter()
                .filter(|(k, _)| *k == *target)
                .count() > 1 { *source } else { *target }
        })
        .collect()
}

fn get_bounds(state: &HashSet<Point>) -> [i64; 4] {
    let min_x = state.iter().map(|p| p.x).min().unwrap();
    let max_x = state.iter().map(|p| p.x).max().unwrap();
    let min_y = state.iter().map(|p| p.y).min().unwrap();
    let max_y = state.iter().map(|p| p.y).max().unwrap();

    [min_x, max_x, min_y, max_y]
}

fn has_neighbours(
    state: &HashSet<Point>,
    p: Point,
    dirs: Vec<Point>
) -> bool {
    match dirs.iter().find(|d| state.contains(&(**d+p))) {
        Some(_) => true,
        _ => false
    }
} 

fn with_diagonals(dir: Point) -> Vec<Point> {
    match dir.x {
        0 => vec!(
            dir, Point::new(-1, dir.y), Point::new(1, dir.y)
        ),
        _ => vec!(
            dir, Point::new(dir.x, -1), Point::new(dir.x, 1)
        )
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    pub x: i64,
    pub y: i64
}

impl Point {
    pub fn new(x: i64, y: i64) -> Point {
        Point { x, y }
    }
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

// const ORTHO_DIRECTIONS: [Point; 4] = [
//     Point{x:1, y:0}, Point{x:-1, y:0},
//     Point{x:0, y:1}, Point{x:0, y:-1}
// ];

const ALL_DIRECTIONS: [Point; 8] = [
    Point{x:1, y:0}, Point{x:-1, y:0},
    Point{x:0, y:1}, Point{x:0, y:-1},
    Point{x:1, y:-1}, Point{x:1, y:1},
    Point{x:-1, y:-1}, Point{x:-1, y: 1},
];