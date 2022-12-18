use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    hash::{Hash, Hasher}
};

const INPUT_PATH: &str = "inputs/016.txt";
const MAX_TIME: u32 = 30;

fn main() {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();

    let mut initial_valves: HashMap<&str, Valve> = file_str.split('\n')
        .map(|l| {
            let (name, flow, connections) = parse_line(l);

            (name, Valve { 
                flow, 
                connections: connections.iter()
                    .map(|c| Connection { valve: c.to_string(), dist: 1})
                    .collect()
            })
        })
        .collect();

    clean_connections(&mut initial_valves);
    println!("Cleaned!");

    let start = State {
        current_valve: "AA".to_string(),
        open_valves: Vec::new(),
        elapsed: 0,
        score_so_far: 0
    };

    let cache = Cache { states: RefCell::new(HashMap::new()) };

    let score = get_state_score(
        start,
        &cache,
        &initial_valves,
        1
    );

    println!("{}", score);
}

fn get_state_score<'a>(
    state: State,
    cache: &'a Cache,
    valves: &'a HashMap<&'a str, Valve>,
    time_step: u32
) -> u32 {
    if state.elapsed >= MAX_TIME { return state.score_so_far; }
    if cache.states.borrow().contains_key(&state) {
        return cache.states.borrow()[&state];
    }

    let actions = get_possible_actions(
        &valves,
        &state.open_valves,
        &state.current_valve
    );

    let new_score = state.score_so_far + time_step * get_current_score(
        &valves,
        &state.open_valves
    );

    let mut child_scores = Vec::new();

    for action in actions {
        let mut open_valves = state.open_valves.clone();

        if action.valve == state.current_valve {
            if valves[&action.valve as &str].flow == 0 { continue; }
            open_valves.push(action.valve.clone());
            open_valves.sort();
        }
        
        let new_state = State {
            current_valve: action.valve.clone(),
            open_valves: open_valves,
            elapsed: state.elapsed + action.dist,
            score_so_far: new_score
        };

        let score = get_state_score(
            new_state.clone(),
            cache,
            valves,
            action.dist
        );
        cache.states.borrow_mut().insert(new_state, score);
        child_scores.push(score);
    }

    *child_scores.iter().max().unwrap()
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

fn get_possible_actions<'a> (
    valves: &'a HashMap<&'a str, Valve>,
    open_valves: &Vec<String>,
    current_valve: &'a str
) -> Vec<Connection> {
    let mut actions: Vec<Connection> = valves[&current_valve as &str].connections
        .iter()
        .map(|c| c.clone())
        .collect();

    if !open_valves.contains(&current_valve.to_string()) {
        actions.push(Connection {valve: current_valve.to_string(), dist: 1 });
    }
    actions
}

struct Cache {
    pub states: RefCell<HashMap<State, u32>>
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    pub elapsed: u32,
    pub current_valve: String,
    pub open_valves: Vec<String>,
    pub score_so_far: u32
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.elapsed.hash(state);
        self.current_valve.hash(state);
        self.open_valves.hash(state);
    }
}

#[derive(Clone)]
struct Valve {
    pub flow: u32,
    pub connections: Vec<Connection>
}

#[derive(Clone)]
struct Connection {
    pub valve: String,
    pub dist: u32
}

fn clean_connections(valves: &mut HashMap<&str, Valve>) {
    let cached_valves = valves.clone();
    for (key, mut valve) in valves.iter_mut() {
        valve.connections = get_non_zero_connections(&cached_valves, key, 1, "");
    }
}

fn get_non_zero_connections(valves: &HashMap<&str, Valve>, valve: &str, dist: u32, prev: &str ) -> Vec<Connection> {
    let mut output = Vec::new();

    for connection in valves[&valve].connections.iter() {
        if connection.valve == prev { continue; }
        if valves[&connection.valve as &str].flow == 0 {
            output.extend(
                get_non_zero_connections(valves, &connection.valve, dist + 1, valve)
            );
        } else {
            output.push(Connection { valve: connection.valve.clone(), dist: dist });
        }
    }
    output
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