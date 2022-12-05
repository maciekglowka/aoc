use std::{
    collections::HashMap,
    fs,
    io::{BufReader, BufRead}
};

const INPUT_PATH: &str = "inputs/002.txt";

fn main() {
    let WINNING_GAMES: HashMap<char, char> = HashMap::from([
        ('A', 'B'),
        ('B', 'C'),
        ('C', 'A')
    ]);

    let LOSING_GAMES: HashMap<char, char> = HashMap::from([
        ('B', 'A'),
        ('C', 'B'),
        ('A', 'C')
    ]);

    if let Ok(file) = fs::File::open(INPUT_PATH) {
        let mut score = 0;
        for l in BufReader::new(file).lines() {
            // let line = l.unwrap();
            // if WINNING_GAMES.contains(&&line[..]) {
            //     score += 6;
            // } else if DRAW_GAMES.contains(&&line[..]) {
            //     score += 3;
            // }
            // score += match line.chars().collect::<Vec<char>>()[2] {
            //     'X' => 1,
            //     'Y' => 2,
            //     'Z' => 3,
            //     _ => 0,
            // }

            let line_chars = l.unwrap().chars().collect::<Vec<char>>();

            let c = match line_chars[2] {
                'X' => {
                    LOSING_GAMES.get(&line_chars[0]).unwrap().to_owned()
                },
                'Y' => {
                    score += 3;
                    line_chars[0]
                },
                'Z' => {
                    score += 6;
                    WINNING_GAMES.get(&line_chars[0]).unwrap().to_owned()
                },
                _ => panic!(),
            };
            score += figure_score(c);
        }
        println!("{}", score);
    }
}

// A, X Rock
// B, Y Paper
// C, Z Scissors

// const WINNING_GAMES: [&str; 3] = [
//     "A Y",
//     "B Z",
//     "C X"
// ];

// const DRAW_GAMES: [&str; 3] = [
//     "A X",
//     "B Y",
//     "C Z"
// ];

fn figure_score(c: char) -> u32 {
    match c {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => 0
    }
}