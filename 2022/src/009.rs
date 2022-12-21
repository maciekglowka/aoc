use std::{
    cmp::max,
    collections::HashSet,
    fs,
    ops::{AddAssign, Sub}
};

const INPUT_PATH: &str = "inputs/009.txt";

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Position {
    pub x: i32,
    pub y: i32
}

impl Position {
    pub fn dist(&self, other: &Position) -> i32 {
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

impl Sub for Position {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        return Position{ x: self.x - other.x, y: self.y - other.y}
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, other: Self) {
        *self = Self{x: self.x + other.x, y: self.y + other.y};
    }
}


fn main () {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();
    
    let commands: Vec<(&str, u32)> = file_str.split('\n')
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            (parts[0], parts[1].parse::<u32>().unwrap())
        })
        .collect();

    // let mut head = Position{ x: 0, y: 0 };
    // let mut tail = Position{ x: 0, y: 0 };
    // let mut visited = HashSet::new();
    // visited.insert(tail);

    // for command in commands {
    //     for _ in 0..command.1 {
    //         move_single(command.0, &mut head);
    //         follow(&head, &mut tail);
    //         visited.insert(tail);
    //     }
    // }

    // println!("{:?}", visited.len());

    let mut rope = [Position{ x: 0, y: 0 }; 10];
    let mut visited = HashSet::new();
    visited.insert(rope[9]);

    for command in commands {
        for _ in 0..command.1 {
            move_single(command.0, &mut rope[0]);
            for idx in 1..10 {
                follow(&rope[idx-1].clone(), &mut rope[idx]);
            }
            visited.insert(rope[9]);
        }
    }
    println!("{:?}", visited.len());
}

fn move_single(dir: &str, head: &mut Position) {
    match dir {
        "R" => head.x += 1,
        "U" => head.y += 1,
        "L" => head.x -= 1,
        "D" => head.y -= 1,
        _ => return
    };
}

fn follow(head: &Position, tail: &mut Position) {
    if head.dist(tail) < 2 { return }

    let mut d = *head - *tail;
    d.clamp();
    *tail += d;
}