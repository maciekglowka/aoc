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

    let initial_valves: HashMap<&str, Valve> = file_str.split('\n')
        .map(|l| {
            let (name, flow, connections) = parse_line(l);
            (name, Valve { flow, connections })
        })
        .collect();

    let start = State {
        current_valve: "AA",
        open_valves: Vec::new(),
        elapsed: 0,
        score_so_far: 0
    };

    let cache = Cache { states: RefCell::new(HashMap::new()) };

    let score = get_state_score(
        start,
        &cache,
        &initial_valves,
    );

    println!("{}", score);
}

fn get_state_score<'a>(
    state: State<'a>,
    cache: &'a Cache<'a>,
    valves: &'a HashMap<&'a str, Valve>,
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

    let new_score = state.score_so_far + get_current_score(
        &valves,
        &state.open_valves
    );

    let mut child_scores = Vec::new();

    for action in actions {
        let mut open_valves = state.open_valves.clone();

        if action == state.current_valve {
            if valves[&action as &str].flow == 0 { continue; }
            open_valves.push(action.clone());
            open_valves.sort();
        }
        
        let new_state = State {
            current_valve: action.clone(),
            open_valves: open_valves,
            elapsed: state.elapsed + 1,
            score_so_far: new_score
        };

        let score = get_state_score(
            new_state.clone(),
            cache,
            valves,
        );
        cache.states.borrow_mut().insert(new_state, score);
        child_scores.push(score);
    }

    *child_scores.iter().max().unwrap()
}

fn get_current_score(
    valves: &HashMap<&str, Valve>,
    open_valves: &Vec<&str>
) -> u32 {
    let mut output = 0;
    for valve in open_valves {
        output += valves[valve].flow;
    }
    output
}

fn get_possible_actions<'a> (
    valves: &'a HashMap<&'a str, Valve>,
    open_valves: &Vec<&'a str>,
    current_valve: &'a str
) -> Vec<&'a str> {
    let mut actions: Vec<&str> = valves[&current_valve as &str].connections
        .iter()
        .map(|a| a.as_str())
        .collect();

    if !open_valves.contains(&current_valve) {
        actions.push(current_valve);
    }
    actions
}

struct Cache<'a> {
    pub states: RefCell<HashMap<State<'a>, u32>>
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State<'a> {
    pub elapsed: u32,
    pub current_valve: &'a str,
    pub open_valves: Vec<&'a str>,
    pub score_so_far: u32
}

impl Hash for State<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.elapsed.hash(state);
        self.current_valve.hash(state);
        self.open_valves.hash(state);
    }
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
        .map(|a| a.replace(',', ""))
        .collect::<Vec<String>>()[5..];

    (name, flow, connections.to_vec())
}