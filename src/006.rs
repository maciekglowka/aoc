use std::{
    collections::{HashSet, VecDeque},
    fs,
    iter::FromIterator
};

const INPUT_PATH: &str = "inputs/006.txt";

fn main() {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();

    let mut v: VecDeque<char> = VecDeque::new();

    for (idx, c) in file_str.chars().enumerate() {
        if v.len() >= 14 {
            v.pop_front();
        } 
        v.push_back(c);
        let s: HashSet<char> = HashSet::from_iter(v.iter().cloned());  
        if s.len() == 14 { 
            println!("{}", idx + 1);
            break; 
        }
    }
}