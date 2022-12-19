use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    fs,
};

const INPUT_PATH: &str = "inputs/016.txt";

fn main() {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();

    let valves: HashMap<&str, Valve> = file_str.split('\n')
        .map(|l| {
            let (name, flow, connections) = parse_line(l);
            (name, Valve { flow, connections })
        })
        .collect();


    let start = State {
        positions: vec!("AA".to_string(), "AA".to_string()),
        open_valves: Vec::new(),
        score_so_far: 0
    };

    let score = run(26, &valves, start);

    println!("{}", score);
}

fn run(steps: u32, valves: &HashMap<&str, Valve>, start: State) -> u32 {
    let mut current_gen = vec!(start);

    for step in 0..steps {
        let mut next_gen = HashSet::new();
        for state in current_gen.iter() {
            let new_score = state.score_so_far + get_current_score(
                &valves,
                &state.open_valves
            );

            let actions: Vec<Vec<String>> = state.positions.iter()
                .map(|p|
                    get_possible_actions(
                        &valves,
                        &state.open_valves,
                        p
                    )
                )
                .collect();

            let combinations: Vec<Vec<String>> = actions[0].iter()
                .map(
                    |a| actions[1].iter()
                        .map(|b| vec!(a.clone(), b.clone()))
                        .collect::<Vec<Vec<String>>>()
                )
                .flatten()
                .collect();

            for combination in combinations {
                let mut open_valves = state.open_valves.clone();
                
                let mut valid = true;
                for i in 0..2 {
                    if combination[i] != state.positions[i] { continue; }
                    if open_valves.contains(&combination[i]) { valid = false; }
                    if valves[&combination[i] as &str].flow == 0 { valid = false; }
                    open_valves.push(combination[i].clone());
                    open_valves.sort();
                }
                if !valid { continue; }

                let mut positions = combination.clone();
                positions.sort();

                let new_state = State {
                    positions: positions,
                    open_valves: open_valves,
                    score_so_far: new_score
                };
                next_gen.insert(new_state);
            }
        }
        let mut next_v: Vec<State> = next_gen.into_iter().collect();
        next_v.sort_by(|a, b| a.score_so_far.cmp(&b.score_so_far));
        current_gen = next_v.into_iter().rev().take(20000).collect();
    }

    current_gen.iter()
        .map(|a| a.score_so_far)
        .max()
        .unwrap()
}

fn get_current_score(
    valves: &HashMap<&str, Valve>,
    open_valves: &Vec<String>
) -> u32 {
    let mut output = 0;
    for valve in open_valves {
        output += valves[valve as &str].flow;
    }
    output
}

fn get_possible_actions(
    valves: &HashMap<&str, Valve>,
    open_valves: &Vec<String>,
    current_valve: &String
) -> Vec<String> {
    let actions = &mut valves[current_valve as &str].connections
        .clone()
        .to_owned();
    if !open_valves.contains(current_valve) {
        actions.push(current_valve.clone());
    }
    actions.to_vec()
}

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct State {
    pub positions: Vec<String>,
    pub open_valves: Vec<String>,
    pub score_so_far: u32
}

#[derive(Clone)]
struct Valve {
    pub flow: u32,
    pub connections: Vec<String>
}

fn parse_line(line: &str) -> (&str, u32, Vec<String>) {
    let parts: Vec<&str> = line.split(';').collect();

    let valve_parts: Vec<&str> = parts[0].split(' ').collect();
    let name = valve_parts[1];
    let flow = valve_parts[4].split('=')
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    let connections = &parts[1].split(' ')
        .map(|a| a.replace(',', ""))
        .collect::<Vec<String>>()[5..];

    (name, flow, connections.to_vec())
}