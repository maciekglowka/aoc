use std::{
    cell::RefCell,
    collections::HashMap,
    convert::TryInto,
    fs,
    iter::zip
};

const INPUT_PATH: &str = "inputs/019.txt";
const MAX_TIME: u32 = 24;

fn main() {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();

    let factories: Vec<Factory> = file_str.split('\n')
        .map(|l| {
            let values = parse_line(l);
            let mut cost = Vec::new();
            // ore bot
            cost.push([values[0][0], 0, 0, 0]);
            // clay bot
            cost.push([values[1][0], 0, 0, 0]);
            // obsidian bot
            cost.push([values[2][0], values[2][1], 0, 0]);
            // geode bot
            cost.push([values[3][0], 0, values[3][1], 0]);

            let max_res_use = (0..4).map(|i| 
                    cost.iter().map(|c| c[i]).max().unwrap()
                )
                .collect::<Vec<i32>>()
                .try_into()
                .unwrap();

            Factory { cost, max_res_use }
        })
        .collect();

    let start = State {
        robots: [1,0,0,0],
        stock: [0,0,0,0],
    };
    
    // FIRST
    let mut score = 0;
    for (i, factory) in factories.iter().enumerate() {
        let cache = Cache { 
            states: RefCell::new(HashMap::new()),
            best_so_far: RefCell::new(0) 
        };
        let sub_score = get_state_score(&start, factory, &cache, 1);
        println!("Sub: {}", sub_score);
        score += (i as i32 + 1) * sub_score;
    };
    println!("Final: {}", score);
}

fn get_state_score(
    state: &State,
    factory: &Factory,
    cache: &Cache,
    step: u32
) -> i32 {
    if step > MAX_TIME { 
        if *cache.best_so_far.borrow() < state.stock[3] {
            *cache.best_so_far.borrow_mut() = state.stock[3];
        }
        return state.stock[3]; 
    }
    if cache.states.borrow().contains_key(state) {
        return cache.states.borrow()[state];
    }

    if get_theoretical_score(state, step) < *cache.best_so_far.borrow() {
        return 0;
    }

    let new_states = get_possible_actions(state, factory);
    new_states.iter()
        .map(|s| {
            let score = get_state_score(s, factory, &cache, step + 1);
            cache.states.borrow_mut().insert(s.clone(), score);
            score
        })
        .max()
        .unwrap()
}

fn pay(stock: [i32; 4], cost: [i32; 4]) -> [i32; 4] {
    zip(stock, cost)
        .map(|(s, c)| s - c)
        .collect::<Vec<i32>>()
        .try_into()
        .unwrap()
}

fn gather(robots: [i32; 4], stock: [i32; 4]) -> [i32; 4] {
    zip(robots, stock)
        .map(|(r, s)| r + s)
        .collect::<Vec<i32>>()
        .try_into()
        .unwrap()
}

fn get_theoretical_score(state: &State, step: u32) -> i32 {
    let left = 1 + (MAX_TIME - step) as i32;

    let geodes = state.stock[3];
    let geode_robots = state.robots[3];
    let possible_geodes = left * (left - 1) / 2;
    geodes + left * geode_robots + possible_geodes 
}

fn get_possible_actions(
    state: &State,
    factory: &Factory
) -> Vec<State> {
    let mut actions: Vec<State> = factory.cost.iter()
        .enumerate()
        .filter_map(|(i, c)| {
            if i != 3 && state.robots[i] >= factory.max_res_use[i] { return None; }

            let stock = pay(state.stock, *c);
            match stock.iter().any(|a| *a < 0) {
                true => None,
                false => {
                    let mut robots = state.robots.clone();
                    robots[i] += 1;
                    Some(State { 
                        stock: gather(state.robots, stock),
                        robots 
                    })
                }
            }
        })
        .collect();
    actions.push(State { 
        stock: gather(state.robots, state.stock),
        robots: state.robots
    });
    actions
}

struct Cache {
    pub states: RefCell<HashMap<State, i32>>,
    pub best_so_far: RefCell<i32>
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct State {
    robots: [i32; 4],
    stock: [i32; 4],
}

#[derive(Debug)]
struct Factory {
    cost: Vec<[i32; 4]>,
    max_res_use: [i32; 4]
}

fn parse_line(line: &str) -> Vec<Vec<i32>> {
    let body: Vec<&str> = line.split(':').collect();
    let parts: Vec<&str> = body[1].split('.').collect();

    parts.iter()
        .map(|part| part.split(' ')
            .filter_map(|a| a.parse::<i32>().ok())
            .collect()
        )
        .collect()
}