use std::{
    collections::{HashMap, HashSet},
    fs,
    iter::zip
};

const INPUT_PATH: &str = "inputs/016.txt";

fn main() {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();

    let initial_valves: HashMap<String, Valve> = file_str.split('\n')
        .map(|l| {
            let (name, flow, connections) = parse_line(l);
            (name.to_string(), Valve { flow, connections })
        })
        .collect();

    let best_0 = run(&initial_valves, &HashSet::new(), 26);
    let best_1 = run(&initial_valves, &best_0.open_valves, 26);

    let mut score = 0;
    let mut open_valves = HashSet::new();
    let mut pos_0 = "AA".to_string();
    let mut pos_1 = "AA".to_string();

    for (s0, s1) in zip(best_0.path, best_1.path) {
        println!("{}, {}", s0, s1);

        if pos_0 == s0 {
            open_valves.insert(pos_0);
        }
        if pos_1 == s1 {
            open_valves.insert(pos_1);
        }

        pos_0 = s0;
        pos_1 = s1;

        score = get_current_score(
            &initial_valves,
            &open_valves,
            score
        );
        println!("{}", score);
    }

    println!("{}", score);
}

fn run(
    valves: &HashMap<String, Valve>,
    open_valves: &HashSet<String>,
    iterations: u32
) -> State {
    let start = State {
        current_valve: "AA".to_string(),
        open_valves: open_valves.clone(),
        score_so_far: 0,
        path: Vec::new()
    };

    let mut current_gen = vec!(start);

    for step in 0..iterations {
        let mut next_gen = Vec::new();
        for state in current_gen.iter() {
            let actions = get_possible_actions(
                &valves,
                &state.open_valves,
                &state.current_valve
            );
            let new_score = get_current_score(
                &valves,
                &state.open_valves,
                state.score_so_far
            );
            
            for action in actions {
                let mut new_path = state.path.clone();
                new_path.push(action.clone());

                if action == state.current_valve {
                    if valves[&action].flow == 0 { continue; }
                    let mut open = state.open_valves.clone();
                    open.insert(action);
                    let new_state = State {
                        current_valve: state.current_valve.clone(),
                        open_valves: open,
                        score_so_far: new_score,
                        path: new_path
                    };
                    next_gen.push(new_state);
                    continue;
                }
                
                let new_state = State {
                    current_valve: action,
                    open_valves: state.open_valves.clone(),
                    score_so_far: new_score,
                    path: new_path
                };
                next_gen.push(new_state);
            }
        }

        next_gen.sort_by(|a, b| a.score_so_far.cmp(&b.score_so_far));
        current_gen = next_gen.into_iter().rev().take(10000).collect();
        // println!("Step {}", step + 1);
        // println!("{:?}", current_gen.iter().map(|a| a.score_so_far).collect::<Vec<u32>>());
    }

    // let score: u32 = current_gen.iter()
    //     .map(|a| a.score_so_far)
    //     .max()
    //     .unwrap();
    current_gen.sort_by_key(|a| a.score_so_far);
    let best = current_gen.last().unwrap();

    best.clone()
}

fn get_current_score(
    valves: &HashMap<String, Valve>,
    open_valves: &HashSet<String>,
    score_so_far: u32
) -> u32 {
    let mut output = score_so_far;
    for valve in open_valves {
        output += valves[valve].flow;
    }
    output
}

fn get_possible_actions(
    valves: &HashMap<String, Valve>,
    open_valves: &HashSet<String>,
    current_valve: &String
) -> Vec<String> {
    let actions = &mut valves[current_valve].connections
        .clone()
        .to_owned();
    if !open_valves.contains(current_valve) {
        actions.push(current_valve.clone());
    }
    actions.to_vec()
}

#[derive(Clone, Debug)]
struct State {
    pub current_valve: String,
    pub open_valves: HashSet<String>,
    pub score_so_far: u32,
    pub path: Vec<String>
}

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
        .map(|a| a.replace(',', "").to_string())
        .collect::<Vec<String>>()[5..];

    (name, flow, connections.to_vec())
}