use std::{
    collections::HashSet,
    fs,
    iter::FromIterator
};

const INPUT_PATH: &str = "inputs/008.txt";

#[derive(Clone, Copy, Debug)]
struct Tree  {
    pub height: u32,
    pub id: u32
}

fn main () {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();
    
    let mut id = 0;
    let forest: Vec<Vec<Tree>> = file_str.split('\n')
        .map(|line| {
            line.chars()
                .map(|c| {
                    id += 1;
                    Tree { 
                        height: c.to_string().parse::<u32>().unwrap(),
                        id
                    }
                })
                .collect()
        })
        .collect();

    let mut visible = Vec::new();

    for y in 0..forest.len() {
        // println!("Y: {}", y);
        visible.extend(get_visible(get_row(&forest, y, false)));
        visible.extend(get_visible(get_row(&forest, y, true)));
    }
    for x in 0..forest[0].len() {
        // println!("X: {}", x);
        visible.extend(get_visible(get_col(&forest, x, false)));
        visible.extend(get_visible(get_col(&forest, x, true)));
    }

    let res: HashSet<u32> = HashSet::from_iter(visible.into_iter());
    println!("{:?}", res.len());

    // println!("Score {}", get_tree_score(&forest, 2, 3));
    let mut scores = Vec::new();
    for y in 0..forest.len() {
        for x in 0..forest[0].len() {
            scores.push(get_tree_score(&forest, x, y));
        }
    }
    scores.sort();
    println!("{:?}", scores.iter().max());
}

fn get_row(forest: &Vec<Vec<Tree>>, idx: usize, rev: bool) -> Vec<Tree> {
    let v = forest[idx].clone();
    if rev {
        v.into_iter().rev().collect()
    } else { v }
}

fn get_col(forest: &Vec<Vec<Tree>>, idx: usize, rev: bool) -> Vec<Tree> {
    let v: Vec<Tree> = forest.iter()
        .map(|line| 
            line[idx].to_owned()
        )
        .collect();
    if rev { v.into_iter().rev().collect() } else { v }
}

fn get_visible(line: Vec<Tree>) -> Vec<u32> {
    let mut top = line[0].height;
    let mut v: Vec<u32> = vec![line[0].id];
    for idx in 1..line.len() - 1 {
        if line[idx].height > top {
            top = line[idx].height;
            v.push(line[idx].id);
        }
    }
    v
}

fn get_ray_len(line: Vec<Tree>, start: usize) -> u32 {
    let mut idx = start + 1;
    let mut count = 0;
    while idx < line.len() {
        count += 1;
        if line[idx].height >= line[start].height { return count }
        idx += 1;
    }
    count
}

fn get_tree_score(forest: &Vec<Vec<Tree>>, col: usize, row: usize) -> u32 {
    let h = forest.len() - 1;
    let w = forest[0].len() - 1;
    let lines = vec!(
        (get_row(&forest, row, false), col),
        (get_row(&forest, row, true), w - col),
        (get_col(&forest, col, false), row),
        (get_col(&forest, col, true), h - row),
    );

    let mut score = 1;
    for line in lines {
        score *= get_ray_len(line.0, line.1);
    }
    score
}