use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fs,
    time::SystemTime
};

const INPUT_PATH: &str = "inputs/011.txt";
const PRIMES: [usize; 9] = [2, 3, 5, 7, 11, 13, 17, 19, 23];

fn main () {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();
    let start_ts = SystemTime::now();

    let monkeys: Vec<RefCell<Monkey>> = file_str.split("\n\n")
        .map(|s| RefCell::new(Monkey::from_str(s)))
        .collect();

    for _ in 0..10000 {
        for m in monkeys.iter() {
            let mut monkey = m.borrow_mut();
            loop {
                match monkey.handle_next() {
                    None => break,
                    Some((item, idx)) => {
                        let mut target = monkeys[idx].borrow_mut();
                        target.insert(item);
                    }
                }
            }
        }
    }

    let mut counters: Vec<usize> = monkeys.iter()
        .map(|m| m.borrow().counter)
        .collect();

    counters.sort();
    println!(
        "{:?}",
        counters[counters.len() - 1] * counters[counters.len() - 2]
    );
    println!(
        "Exec time: {:?}",
        start_ts.elapsed()
    );
}

struct Monkey {
    pub counter: usize,
    // queue: VecDeque<usize>,
    queue: VecDeque<HashMap<usize,usize>>,
    operator: Box::<dyn Fn(HashMap<usize, usize>) -> HashMap<usize, usize>>,
    div: usize,
    target: HashMap<bool, usize>
}

impl Monkey {
    pub fn handle_next(&mut self) -> Option<(HashMap<usize, usize>, usize)> {
        // returns item, target
        let mut item = self.queue.pop_front()?;

        self.counter += 1;
        // item = (self.operator)(item) / 3;
        item = (self.operator)(item);

        let test = item[&self.div] == 0;
        // let target = self.target[&(item % self.div == 0)];
        let target = self.target[&test];

        Some((item, target))
    }
    pub fn insert(&mut self, item: HashMap<usize, usize>) {
        self.queue.push_back(item);
    }
    pub fn from_str(s: &str) -> Monkey {
        let lines: Vec<&str> = s.split('\n').collect();

        let queue = lines[1].replace(',', "")
            .split_whitespace()
            .filter_map(|a| match a.parse::<usize>() {
                Ok(a) => {
                    let mut h = HashMap::new();
                    for p in PRIMES {
                        h.insert(p, a % p);
                    }
                    Some(h)
                },
                Err(_) => None
            })
            .collect();

        let operator = get_operator(lines[2]);
        let div = lines[3].split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let mut target = HashMap::new();
        target.insert(true, lines[4].split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap()
        );
        target.insert(false, lines[5].split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap()
        );

        Monkey {
            counter: 0,
            queue,
            operator,
            div,
            target
        }
    }
}

// fn get_operator(s: &str) -> Box::<dyn Fn(usize) -> usize> {
//     let parts: Vec<&str> = s.split_whitespace().collect();

//     let val = parts[parts.len() - 1];
//     let op = parts[parts.len() - 2];

//     match val {
//         "old" => match op {
//             "+" => Box::new(|old| old + old),
//             "*" => Box::new(|old| old * old),
//             _ => panic!()
//         },
//         a => {
//             let v = a.parse::<usize>().unwrap();
//             match op {
//                 "+" => Box::new(move |old| old + v),
//                 "*" => Box::new(move |old| old * v),
//                 _ => panic!()
//             }
//         }
//     }
// }

fn get_operator(s: &str) -> Box::<dyn Fn(HashMap<usize, usize>) -> HashMap<usize, usize>> {
    let parts: Vec<&str> = s.split_whitespace().collect();

    let val = parts[parts.len() - 1];
    let op = parts[parts.len() - 2];

    match val {
        "old" => match op {
            "+" => Box::new(|old| {
                // old + old
                old.iter()
                    .map(|(k, v)| {
                        let r = (v + v) % k;
                        (*k, r)
                    })
                    .collect()
            }),
            "*" => Box::new(|old| {
                // old * old
                old.iter()
                    .map(|(k, v)| {
                        let r = (v * v) % k;
                        (*k, r)
                    })
                    .collect()
            }),
            _ => panic!()
        },
        a => {
            let i = a.parse::<usize>().unwrap();
            match op {
                "+" => Box::new(move |old| {
                    // old + i
                    old.iter()
                        .map(|(k, v)| {
                            let r = (v + i) % k;
                            (*k, r)
                        })
                        .collect()
                }),
                "*" => Box::new(move |old| {
                    // old * i
                    old.iter()
                        .map(|(k, v)| {
                            let r = (v * i) % k;
                            (*k, r)
                        })
                        .collect()
                }),
                _ => panic!()
            }
        }
    }
}