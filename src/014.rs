use std::{
    cmp::max,
    collections::HashSet,
    fs,
    ops::{Add, AddAssign, Sub}
};

const INPUT_PATH: &str = "inputs/014.txt";

fn main () {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();
    let points: Vec<Vec<Point>> = file_str.split('\n')
        .map(|l| {
            l.replace(' ', "")
            .split("->")
            .map(|p| {
                let parts: Vec<&str> = p.split(',').collect();
                Point {
                    x: parts[0].parse::<i32>().unwrap(),
                    y: parts[1].parse::<i32>().unwrap(),
                }
            })
            .collect()
        })
        .collect();

    let mut map = HashSet::new();

    for line in points {
        for i in 1..line.len() {
            let a = line[i-1];
            let b = line[i];

            let mut step = b - a;
            step.clamp();
            
            let mut cur = a;
            loop {
                map.insert(cur);
                if cur == b { break; }
                cur += step;
            }
        }
    }

    let max_y = map.iter().fold(0, |acc, p| max(acc, p.y));

    let mut counter = 0;

    // 'outer: loop {
    //     let mut cur = Point { x: 500, y: 0 };

    //     'inner: loop {

    //         let next = [Point { x: 0, y: 1 }, Point { x: -1, y: 1 }, Point { x: 1, y: 1 }].iter()
    //             .find(|p| !map.contains(&(cur + **p)));

    //         if let Some(step) = next {
    //             cur += *step;

    //             if cur.y >= max_y { break 'outer; }
    //         } else {
    //             map.insert(cur);
    //             counter += 1;
    //             break 'inner;
    //         }
    //     }
    // }

    'outer: loop {
        let mut cur = Point { x: 500, y: 0 };

        'inner: loop {

            let next = [Point { x: 0, y: 1 }, Point { x: -1, y: 1 }, Point { x: 1, y: 1 }].iter()
                .find(|p| !map.contains(&(cur + **p)));

            let mut settled = true;

            if let Some(step) = next {
                cur += *step;
                if cur.y < max_y + 1 { settled = false; }
            }

            if settled {
                map.insert(cur);
                counter += 1;
                if cur.y != 0 { break 'inner; } else { break 'outer; }
            }
        }
    }

    println!("{}", counter);
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn dist(&self, other: &Point) -> i32 {
        max(
            (self.x-other.x).abs(),
            (self.y-other.y).abs(),
        )
    }
    pub fn clamp(&mut self) {
        if self.x != 0 {
            self.x = self.x / self.x.abs();
        }
        if self.y != 0 {
            self.y = self.y / self.y.abs();
        }
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

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        return Point{ x: self.x - other.x, y: self.y - other.y}
    }
}