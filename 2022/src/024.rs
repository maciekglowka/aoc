use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fs,
    ops::{Add, AddAssign}
};

const INPUT_PATH: &str = "inputs/024.txt";
type Map = HashSet<Point>;
type Blizzards = HashMap<Point, Vec<Point>>;

fn main() {
    let lines: Vec<Vec<char>> = fs::read_to_string(INPUT_PATH)
        .unwrap()
        .split('\n')
        .map(|l|
            l.chars().collect()
        )
        .collect();

    let mut blizzards: Blizzards = HashMap::new();
    let mut map: Map = HashSet::new();

    let w = lines[0].len() as i64;
    let h = lines.len() as i64;
    
    for y in 0..h as usize {
        for x in 0..w as usize {
            let p = Point::new(x as i64, y as i64);
            match lines[y][x] {
                '#' => continue,
                _ => { map.insert(p); }
            };

            match lines[y][x] {
                '<' => blizzards.insert(p, vec!(Point::new(-1, 0))),
                '>' => blizzards.insert(p, vec!(Point::new(1, 0))),
                '^' => blizzards.insert(p, vec!(Point::new(0, -1))),
                'v' => blizzards.insert(p, vec!(Point::new(0, 1))),
                _ => None
            };
        }
    }

    let start = *map.iter().find(|p| p.y == 0).unwrap();
    let end = *map.iter().find(|p| p.y == lines.len() as i64 - 1).unwrap();

    let (score_0, cache) = run(&map, &blizzards, w, h, start, end, 0);
    println!("{}", score_0);
    let (score_1, cache) = run(&map, &cache[&score_0], w, h, end, start, score_0);
    // println!("{}", score_1 - score_0);

    let (score_2, cache) = run(&map, &cache[&score_1], w, h, start, end, score_1);
    // println!("{}", score_2 - score_1);
    println!("{}", score_2);
}

fn run(
    map: &Map,
    blizzards: &Blizzards,
    w: i64,
    h: i64,
    start: Point,
    end: Point,
    start_step: i64,
) -> (i64, HashMap<i64, Blizzards>) {
    let mut cache = HashMap::new();
    cache.insert(start_step, blizzards.clone());

    let mut queue = BinaryHeap::new();
    let start_node = Node {
        p: start,
        steps: start_step,
        dist: start.manhattan(end)
    };
    queue.push(start_node);

    let mut visits = HashMap::new();
    visits.insert(start, HashSet::from([start_step]));

    while let Some(cur) = queue.pop() {
        if cur.p == end {
            return (cur.steps, cache);
        }

        let blizz_state = match cache.get(&(cur.steps + 1)) {
            Some(b) => b.clone(),
            None => {
                let step = blizzard_step(&cache[&cur.steps], w, h);
                cache.insert(cur.steps + 1, step.clone());
                step
            }
        };

        for dir in DIRECTIONS {
            let p = dir + cur.p;
            if !map.contains(&p) { continue; }
            if blizz_state.contains_key(&p) { continue; }

            match visits.get_mut(&p) {
                None => { visits.insert(p, HashSet::from([cur.steps + 1])); },
                Some(v) => if v.contains(&(cur.steps + 1)) { continue; } else {
                    v.insert(cur.steps + 1);
                }
            };

            queue.push(Node {
                p,
                steps: cur.steps + 1,
                dist: end.manhattan(p)
            });
        }
    }
    (-1, HashMap::new())
}

fn blizzard_step(current: &Blizzards, w: i64, h: i64) -> Blizzards {
    let mut new: Blizzards = HashMap::new();
    for (p, blizzards) in current.iter() {
        for b in blizzards {
            let mut v = *p + *b;
            if v.x < 1 { v.x = w - 2; }
            if v.y < 1 { v.y = h - 2; }
            if v.x > w - 2 { v.x = 1; }
            if v.y > h - 2 { v.y = 1; }

            match new.get_mut(&v) {
                Some(vec) => vec.push(*b),
                None => { new.insert(v, vec!(*b)); }
            }
        }
    }
    new
}

fn draw_map(
    blizzards: &Blizzards,
    w: i64,
    h: i64,
    start: Point,
    end: Point
) {
    for y in 0..h {
        for x in 0..w {
            let p = Point::new(x, y);
            if x == 0 || x == w-1 {
                print!("{}", '#');
                continue;
            } 
            if (y == 0 || y == h-1) && p != start && p != end {
                print!("{}", '#');
                continue;
            }

            if blizzards.contains_key(&p) {
                print!("{}", 'b');
                continue;
            }

            print!("{}", '.');
        }
        print!("{}", '\n');
    }
    print!("{}", "\n\n");
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    pub p: Point,
    pub steps: i64,
    pub dist: i64
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
            .then(other.dist.cmp(&self.dist))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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
    pub fn manhattan(&self, other: Point) -> i64 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
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

const DIRECTIONS: [Point; 5] = [
    Point{x:0, y:0},
    Point{x:1, y:0}, Point{x:-1, y:0},
    Point{x:0, y:1}, Point{x:0, y:-1}
];