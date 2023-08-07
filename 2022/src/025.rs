use std::{
    fs
};

const INPUT_PATH: &str = "inputs/025.txt";

fn main() {
    let lines: Vec<String> = fs::read_to_string(INPUT_PATH)
        .unwrap()
        .split('\n')
        .map(|a| a.to_string())
        .collect();

    let mut sum = 0;
    for l in lines {
        sum += quin_from_str(l);
    }
    println!("{}", str_from_quin(sum));
}

fn str_from_quin(n: i64) -> String {
    let mut cur = n;
    let mut s = String::new();

    while cur > 0 {
        let mut r = cur % 5;
        let mut d = cur / 5;

        if r > 2 {
            r = r - 5;
            d += 1;
        }
        s.insert(0, dig_to_char(r));
        cur = d;
    }
    s
}

fn quin_from_str(s: String) -> i64 {
    s.chars()
        .rev()
        .enumerate()
        .map(|(i, c)| {
            5_i64.pow(i as u32) * char_to_dig(c)
        })
        .sum()
}

fn dig_to_char(d: i64) -> char {
    match d {
        2 => '2',
        1 => '1',
        0 => '0',
        -1 => '-',
        -2 => '=',
        _ => panic!()
    }
}

fn char_to_dig(c: char) -> i64 {
    match c {
        '2' => 2,
        '1' => 1,
        '0' => 0,
        '-' => -1,
        '=' => -2,
        _ => panic!()
    }
}