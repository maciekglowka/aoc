use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs
};

const INPUT_PATH: &str = "inputs/016.txt";

fn main () {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();
    
    let valves: HashMap<String, Valve> = file_str.split('\n')
        .map(|l| {
            let (name, flow, connections) = parse_line(l);
            (name.to_string(), Valve { flow, connections })
        })
        .collect();

    let mut starting_pairs = HashSet::new();

    for a in valves[&"AA".to_string()].connections.iter() {
        for b in valves[&"AA".to_string()].connections.iter() {
            if a == b { continue; }
            if a <= b { starting_pairs.insert((a, b)); }
            else { starting_pairs.insert((b, a));}
        }
    }
    // println!("{:?}", starting_pairs);
    let mut total_scores = Vec::new();

    for starting_pair in starting_pairs {


        let start = State {
            positions: [starting_pair.0.clone(), starting_pair.1.clone()],
            open_valves: HashSet::new(),
            score_so_far: 0
        };

        // println!("{}", dist(&valves, "JJ".to_string(), "HH".to_string()));

        let mut current_gen = vec!(start);

        for step in 0..26-1 {
            let mut next_gen = Vec::new();
            for state in current_gen.iter() {
                let new_score = get_current_score(
                    &valves,
                    &state.open_valves,
                    state.score_so_far
                );

                let mut actions: Vec<Vec<(String, bool)>> = state.positions.iter()
                    .map(|p| {
                        get_possible_actions(
                            &valves,
                            &state.open_valves,
                            p
                        ).iter().map(|a| (a.clone(), a == p)).collect()
                    })
                    .collect();

                let combinations: Vec<((String, bool),(String, bool))> = actions[0].iter()
                    .map(
                        |a| actions[1].iter()
                            .map(|b| (a.clone(), b.clone()))
                            .collect::<Vec<((String, bool),(String, bool))>>()
                    )
                    .flatten()
                    .collect();

                for combination in combinations.into_iter() {
                    let mut open = state.open_valves.clone();

                    if combination.0.1 { 
                        if valves[&combination.0.0].flow == 0 { continue; }
                        else { open.insert(combination.0.0.clone()); }
                    }
                    if combination.1.1 { 
                        if valves[&combination.1.0].flow == 0 { continue; }
                        else {open.insert(combination.1.0.clone()); }
                    }

                    let new_state = State {
                        positions: [combination.0.0, combination.1.0],
                        open_valves: open,
                        score_so_far: new_score
                    };
                    next_gen.push(new_state);
                }
            }
            next_gen.sort_by(|a, b| {
                let score_a = get_current_score(
                    &valves,
                    &a.open_valves,
                    a.score_so_far
                );
                let score_b = get_current_score(
                    &valves,
                    &b.open_valves,
                    b.score_so_far
                );
                score_a.cmp(&score_b)
            });
            current_gen = next_gen.into_iter().rev().take(50000).collect();
            println!("Step {}", step + 1);
        }

        let score: u32 = current_gen.iter()
            .map(|a| a.score_so_far)
            .max()
            .unwrap();

            total_scores.push(score);
            println!("{:?}", total_scores);
    }

    total_scores.sort();
    println!("{:?}", total_scores);
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

fn dist(
    valves: &HashMap<String, Valve>,
    a: &String,
    b: &String
) -> u32 {
    let mut scores: HashMap<&String, Option<u32>> = valves.keys()
        .map(|a| (a, None))
        .collect();
    let mut queue = VecDeque::new();
    queue.push_back(b);
    scores.insert(b, Some(0));

    while queue.len() > 0 {
        let cur = queue.pop_front().unwrap();
        let cur_score = scores[cur].unwrap();
        for n in valves[cur].connections.iter() {
            if let Some(score) = scores[n] {
                if score <= cur_score + 1 { continue; }
            }
            scores.insert(n, Some(cur_score + 1));
            queue.push_back(n);
        }
    }
    scores[&a].unwrap()
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

#[derive(Debug)]
struct State {
    pub positions: [String; 2],
    pub open_valves: HashSet<String>,
    pub score_so_far: u32
}

// fn main () {
//     let file_str = fs::read_to_string(INPUT_PATH).unwrap();
    
//     let valves: HashMap<String, Valve> = file_str.split('\n')
//         .map(|l| {
//             let (name, flow, connections) = parse_line(l);
//             (name.to_string(), Valve { flow, connections })
//         })
//         .collect();

//     let start = State {
//         current_valve: "AA".to_string(),
//         open_valves: HashSet::new(),
//         score_so_far: 0
//     };

//     let mut current_gen = vec!(start);

//     for step in 0..30 {
//         let mut next_gen = Vec::new();
//         for state in current_gen.iter() {
//             let actions = get_possible_actions(
//                 &valves,
//                 &state.open_valves,
//                 &state.current_valve
//             );
//             let new_score = get_current_score(
//                 &valves,
//                 &state.open_valves,
//                 state.score_so_far
//             );
            
//             for action in actions {
//                 if action == state.current_valve {
//                     let mut open = state.open_valves.clone();
//                     open.insert(action);
//                     let new_state = State {
//                         current_valve: state.current_valve.clone(),
//                         open_valves: open,
//                         score_so_far: new_score
//                     };
//                     next_gen.push(new_state);
//                     continue;
//                 }

//                 let new_state = State {
//                     current_valve: action,
//                     open_valves: state.open_valves.clone(),
//                     score_so_far: new_score
//                 };
//                 next_gen.push(new_state);
//             }
//         }
//         // if step > 3 {
//         //     // let avg = next_gen.iter().map(|a| a.score_so_far).sum::<u32>() / next_gen.len() as u32;
//         //     // next_gen.retain(|a| a.score_so_far > 3 * avg / 4);
//         // }
//         next_gen.sort_by(|a, b| a.score_so_far.cmp(&b.score_so_far));
//         current_gen = next_gen.into_iter().rev().take(1000).collect();
//         println!("Step {}", step + 1);
//         println!("{:?}", current_gen.iter().map(|a| a.score_so_far).collect::<Vec<u32>>());
//     }

//     let score: u32 = current_gen.iter()
//         .map(|a| a.score_so_far)
//         .max()
//         .unwrap();

//     println!("{}", score);
// }

// fn get_current_score(
//     valves: &HashMap<String, Valve>,
//     open_valves: &HashSet<String>,
//     score_so_far: u32
// ) -> u32 {
//     let mut output = score_so_far;
//     for valve in open_valves {
//         output += valves[valve].flow;
//     }
//     output
// }

// fn get_possible_actions(
//     valves: &HashMap<String, Valve>,
//     open_valves: &HashSet<String>,
//     current_valve: &String
// ) -> Vec<String> {
//     let actions = &mut valves[current_valve].connections
//         .clone()
//         .to_owned();
//     if !open_valves.contains(current_valve) {
//         actions.push(current_valve.clone());
//     }
//     actions.to_vec()
// }

// #[derive(Debug)]
// struct State {
//     pub current_valve: String,
//     pub open_valves: HashSet<String>,
//     pub score_so_far: u32
// }

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