use std::{
    collections::{HashMap, VecDeque},
    fs
};

const INPUT_PATH: &str = "inputs/_021.txt";

fn main() {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();

    let mut monkeys: VecDeque<Monkey> = file_str.split('\n')
        .map(|l| {
            let parts = l.split(": ").collect::<Vec<&str>>();
            match parts[1].parse::<u64>() {
                Ok(val) => Monkey::Numeric(parts[0], val),
                _ => {
                    let op = parts[1].split(' ').collect::<Vec<&str>>();
                    Monkey::Operator(
                        parts[0],
                        Operation { a: op[0], b: op[2], op: op[1].chars().next().unwrap() }
                    )
                } 
            }
        })
        .collect();

    let mut cache = HashMap::new();
    
    while monkeys.len() > 0 {
        let cur = monkeys.pop_front().unwrap();
        match cur {
            Monkey::Numeric(name, val) => { cache.insert(name, val); },
            Monkey::Operator(name, op) => {
                match (cache.get(op.a), cache.get(op.b)) {
                    (Some(a), Some(b)) => { cache.insert(name, op.calc(*a, *b)); },
                    _ => monkeys.push_back(cur)
                }
            }
        }
    }

    println!("Root: {}", cache["root"]);
}

#[derive(Debug)]
enum Monkey<'a> {
    Numeric(&'a str, u64),
    Operator(&'a str, Operation<'a>)
}

#[derive(Clone, Copy, Debug)]
struct Operation<'a> {
    pub a: &'a str,
    pub b: &'a str,
    pub op: char
}

impl Operation<'_> {
    pub fn calc(&self, a: u64, b: u64) -> u64 {
        match self.op {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => a / b,
            _ => panic!()
        }
    }
}
