use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs,
    iter::FromIterator,
    ops::{Add, AddAssign}
};
    

const INPUT_PATH: &str = "inputs/017.txt";
const MAX_DIM: i64 = 7;

fn main () {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();

    let mut jets: VecDeque<Point> = file_str.chars()
        .map(|c| match c {
            '>' => Point { x: 1, y: 0},
            '<' => Point { x: -1, y: 0},
            _ => panic!()
        })
        .collect();

    let cycle = jets.len();

    let mut shapes: VecDeque<HashSet<Point>> = vec!(
        HOR.to_vec(), CROSS.to_vec(), IL.to_vec(), VER.to_vec(), SQR.to_vec()
    ).into_iter()
        .map(|a| HashSet::from_iter(a))
        .collect();

    let mut map = HashSet::new();
    let mut top = 0;
    let mut jet_count = 0;
    let mut cycle_steps = Vec::new();
    
    let mut top_mem = HashMap::new();
    let mut modulo = None;

    for block in 1..10000000000 {
        let mut shape = get_next(&mut shapes);

        shape = get_move(shape, Point { x: 2, y: top + 3 });
        let mut fall = false;

        loop {
            let dir = match fall {
                true => Point { x: 0, y: -1 },
                false => {
                    jet_count += 1;
                    get_next(&mut jets)
                }
            };
            let expected_shape = get_move(shape.clone(), dir);

            fall = !fall;
            if check_collision(&map, &expected_shape) || check_bounds(&expected_shape) {
                if !fall { break; } else { continue; }
            }                
            shape = expected_shape;
        }

        merge(&mut map, &shape);
        top = get_top(&map);
        top_mem.insert(block, top);
        // println!("Top: {}", top);
        // print_map(&map);

        if jet_count >= cycle && modulo.is_none() {
            modulo = Some(jet_count % cycle)
        } else {
            if Some(jet_count % cycle) == modulo {
                cycle_steps.push((block, top));
                if cycle_steps.len() == 2 { break; }
            }
        }

        if block % 10000 == 0 {
            println!("{}", block);
        }
    }
    println!{"Cycle steps {:?}", cycle_steps};
    println!("Top: {}", top);

    let cycle_step = cycle_steps[1].0 - cycle_steps[0].0;
    let height_gain = cycle_steps[1].1 - cycle_steps[0].1;
    let target = 1000000000000_i64;
    let cycle_count = target / cycle_step;
    println!("Cycle count: {}", cycle_count);
    let mut height = height_gain * cycle_count;
    let step_mod = target % cycle_count;
    println!("Height: {}", height + top_mem[&step_mod]);
    println!("Mod: {}", step_mod);

    // for (k, v) in top_mem {
    //     if v == 25 { println!("{}", k)};
    //     // println!("Mem: {:?}", top_mem);
    // }
}

fn print_map(map: &HashSet<Point>) {
    let max_y = map.iter().map(|p| p.y).max().unwrap();
    for y in (0..=max_y).rev() {
        for x in 0..MAX_DIM {
            if map.contains(&Point { x, y }) { print!("{}", '#'); } else { print!("{}", '.'); }
        }
        print!("{}", '\n');
    }
    print!("{}", '\n');
}

fn get_base_x(shape: &HashSet<Point>) -> i64 {
    shape.iter().map(|p| p.x).min().unwrap() + 1
}

fn merge(map: &mut HashSet<Point>, shape: &HashSet<Point>) {
    for p in shape {
        map.insert(*p);
    }
}

fn get_top(map: &HashSet<Point>) -> i64 {
    map.iter().map(|p| p.y).max().unwrap() + 1
}

fn check_bounds(shape: &HashSet<Point>) -> bool {
    for p in shape.iter() {
        if p.x < 0 || p.x >= MAX_DIM || p.y < 0 { return true; }
    }
    false
}

fn check_collision(map: &HashSet<Point>, shape: &HashSet<Point>) -> bool {
    for p in shape.iter() {
        if map.contains(p) { return true; }
    }
    false
}

fn get_move(shape: HashSet<Point>, dir: Point) -> HashSet<Point> {
    shape.into_iter()
        .map(|p| p + dir)
        .collect()
}

fn get_next<T: Clone>(v: &mut VecDeque<T>) -> T {
    let next = v.pop_front().unwrap();
    v.push_back(next.clone());
    next
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

const HOR: [Point; 4] = [
    Point { x: 0, y: 0 }, Point { x: 1, y: 0 },
    Point { x: 2, y: 0 }, Point { x: 3, y: 0 }
];
const CROSS: [Point; 5] = [
    Point { x: 1, y: 0 },
    Point { x: 0, y: 1 },  Point { x: 1, y: 1 }, Point { x: 2, y: 1 },
    Point { x: 1, y: 2 }
];
const IL: [Point; 5] = [
    Point { x: 0, y: 0 }, Point { x: 1, y: 0 }, Point { x: 2, y: 0 },
    Point { x: 2, y: 1 }, Point { x: 2, y: 2 }
];
const VER: [Point; 4] = [
    Point { x: 0, y: 0 }, Point { x: 0, y: 1 },
    Point { x: 0, y: 2 }, Point { x: 0, y: 3 }
];
const SQR: [Point; 4] = [
    Point { x: 0, y: 0 }, Point { x: 1, y: 0 },
    Point { x: 0, y: 1 }, Point { x: 1, y: 1 }
];