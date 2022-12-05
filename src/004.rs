use std::{
    fs,
    io::{BufReader, BufRead}
};

const INPUT_PATH: &str = "inputs/004.txt";

fn main() {
    if let Ok(file) = fs::File::open(INPUT_PATH) {
        // let s0: u32 = BufReader::new(file).lines()
        //     .map(|l| {
        //         let parts: Vec<Vec<u32>> = l.unwrap()
        //             .split(',')
        //             .map(|h| {
        //                 h.split('-')
        //                 .map(|a| a.parse::<u32>().unwrap())
        //                 .collect()
        //             })
        //             .collect();
                
        //         if (parts[0][0] >= parts[1][0] && parts[0][1] <= parts[1][1]) ||
        //             (parts[0][0] <= parts[1][0] && parts[0][1] >= parts[1][1]) {
        //                 1
        //             } else {
        //                 0
        //             }
        //     })
        //     .sum();

        // println!("FIRST: {}", s0);

        let s1: u32 = BufReader::new(file).lines()
            .map(|l| {
                let parts: Vec<Vec<u32>> = l.unwrap()
                    .split(',')
                    .map(|h| {
                        h.split('-')
                        .map(|a| a.parse::<u32>().unwrap())
                        .collect()
                    })
                    .collect();
                
                if (parts[0][0] >= parts[1][0] && parts[0][0] <= parts[1][1]) ||
                    (parts[0][1] >= parts[1][0] && parts[0][1] <= parts[1][1]) ||
                    (parts[1][0] >= parts[0][0] && parts[1][0] <= parts[0][1]) ||
                    (parts[1][1] >= parts[0][0] && parts[1][1] <= parts[0][1]) {
                        1
                    } else {
                        0
                    }
                    
            })
            .sum();

        println!("SECOND: {}", s1);
    }
}