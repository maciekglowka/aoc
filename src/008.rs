use std::{
    collections::HashSet,
    fs,
    iter::FromIterator
};

const INPUT_PATH: &str = "inputs/_008.txt";

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
        println!("Y: {}", y);
        visible.extend(get_visible(get_row(&forest, y, false)));
        visible.extend(get_visible(get_row(&forest, y, true)));
    }
    for x in 0..forest[0].len() {
        println!("X: {}", x);
        visible.extend(get_visible(get_col(&forest, x, false)));
        visible.extend(get_visible(get_col(&forest, x, true)));
    }

    let res: HashSet<u32> = HashSet::from_iter(visible.into_iter());
    println!("{:?}", res.len());
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
    for idx in 1..line.len() {
        if line[idx].height <= line[idx-1].height {
            return v
        }
        v.push(line[idx].id);
        println!("Adding: {} at {}", line[idx].height, idx);
    }
    v
}