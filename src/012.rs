use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fs
};

const INPUT_PATH: &str = "inputs/012.txt";

fn main () {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();

    let tiles: HashMap<Point, RefCell<Tile>> = file_str.split('\n')
        .enumerate()
        .map(move |(y, l)| l.chars()
            .enumerate()
            .map(move |(x, c)| {
                let height = match c.is_lowercase() {
                    true => c,
                    false => match c {
                        'S' => 'a',
                        'E' => 'z',
                        _ => panic!()
                    }
                };
                (
                    Point::from_u(x, y),
                    RefCell::new(
                        Tile { c, height: height as i32, ..Default::default() }
                    )
                )
            })
        )
        .flatten()
        .collect();

    let start = tiles.iter().find(|(k, v)| v.borrow().c == 'S').unwrap().0;
    let end = tiles.iter().find(|(k, v)| v.borrow().c == 'E').unwrap().0;

    let mut queue = VecDeque::new();
    tiles[&end].borrow_mut().score = Some(0);
    queue.push_back(*end);

    while queue.len() > 0 {
        let cur = queue.pop_front().unwrap();
        let tile = &tiles[&cur].borrow();

        for p in [
            Point::new(cur.x-1, cur.y), Point::new(cur.x+1, cur.y),
            Point::new(cur.x, cur.y-1), Point::new(cur.x, cur.y+1)
        ] {
            if !tiles.contains_key(&p) { continue; }
            if let Some(score) = tiles[&p].borrow().score {
                if score <= 1+ tile.score.unwrap() { continue; } 
            }
            if tile.height - tiles[&p].borrow().height > 1 { continue; }

            let mut n = tiles[&p].borrow_mut();

            n.score = Some(tile.score.unwrap() + 1);
            n.came_from = Some(cur);
            queue.push_back(p);
        }
    }

    println!("Start {:?}", tiles[&start]);

    let mut end_scores: Vec<usize> = tiles.values()
        .filter(|t| t.borrow().c == 'a')
        .filter_map(|t| t.borrow().score)
        .collect();

    // let e = tiles[&end].borrow();
    // if let Some(score) = e.score {
    //     end_scores.push(score);
    // }

    end_scores.sort();
    println!("{:?}", end_scores);
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Point {
    pub x: i32,
    pub y: i32
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    pub fn from_u(x: usize, y: usize) -> Point {
        Point { x: x as i32, y: y as i32 }
    }
}

#[derive(Clone, Debug, Default)]
struct Tile {
    pub c: char,
    pub height: i32,
    pub score: Option<usize>,
    pub came_from: Option<Point>
}