use std::{
    fs,
    io::{BufReader, BufRead}
};

const INPUT_PATH: &str = "inputs/001.txt";

fn main() {
    if let Ok(file) = fs::File::open(INPUT_PATH) {
        let mut cur = 0;
        // let mut max = 0;
        let mut sums = Vec::new();

        for line in BufReader::new(file).lines() {
            match line {
                Ok(l) if l.len() == 0 => {
                    // if cur > max { max = cur; }
                    sums.push(cur);
                    cur = 0;
                },
                Ok(l) => cur += l.parse::<u32>().unwrap(),
                _ => ()
            }
        }
        sums.sort();
        println!("{:?}", sums);
    }

}