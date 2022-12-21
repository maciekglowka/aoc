use std::{
    fs,
    io::{BufReader, BufRead}
};

const INPUT_PATH: &str = "inputs/003.txt";

fn main() {
    if let Ok(file) = fs::File::open(INPUT_PATH) {
        // let s0: u32 = BufReader::new(file).lines()
        //     .map(|line| {
        //         let v = line.unwrap().chars().collect::<Vec<char>>();
        //         let l = v.len() / 2;
        //         let halfs = (&v[..l], &v[l..]);
        //         let c = halfs.0.iter()
        //             .find(|a| halfs.1.iter().find(|b| a == b).is_some()).unwrap();
                
        //         match c.is_lowercase() {
        //             true => *c as u32 - 96,
        //             false => *c as u32 - 38
        //         }
        //     })
        //     .sum();
        // println!("FIRST: {:?}", s0);

        let s1: u32 = BufReader::new(file).lines()
            .map(|l| l.unwrap().chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>()
            .chunks(3)
            .map(|lines| {
                let c = lines[0].iter()
                    .find(
                        |x| lines[1].iter()
                            .find(|y| 
                                lines[2].iter()
                                    .find(|z|
                                        z == y && z == x
                                    )
                                    .is_some()
                            )
                            .is_some()
                    ).unwrap();
                    match c.is_lowercase() {
                        true => *c as u32 - 96,
                        false => *c as u32 - 38
                    } 
            })
            .sum();
        println!("SECOND: {:?}", s1);
    }
}