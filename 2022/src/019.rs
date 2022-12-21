use std::{
    cell::RefCell,
    collections::{BTreeMap, HashMap, HashSet},
    fs,
    hash::{Hash, Hasher}
};

const INPUT_PATH: &str = "inputs/019.txt";

fn main() {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();

    let factories: Vec<Factory> = file_str.split('\n')
        .map(|l| {
            let values = parse_line(l);
            let mut cost = HashMap::new();
            cost.insert(
                Res::Ore,
                HashMap::from([(Res::Ore, values[0][0])])
            );
            cost.insert(
                Res::Clay,
                HashMap::from([(Res::Ore, values[1][0])])
            );
            cost.insert(
                Res::Obsidian,
                HashMap::from([
                    (Res::Ore, values[2][0]), (Res::Clay, values[2][1])
                ])
            );
            cost.insert(
                Res::Geode,
                HashMap::from([
                    (Res::Ore, values[3][0]), (Res::Obsidian, values[3][1])
                ])
            );
            Factory { cost }
        })
        .collect();

    let start = State {
        robots: HashMap::from([(Res::Ore, 1)]),
        stock: HashMap::new(),
    };
    
    // FIRST
    // let mut score = 0;
    // for (i, factory) in factories.iter().enumerate() {
    //     score += (i as u32 + 1) * run(24, &factory, &start);
    // };
    // println!("Final: {}", score);


    //SECOND
    let mut score = 1;
    for factory in &factories[..3] {
        score *= run(32, &factory, &start);
    };
    println!("Final: {}", score);
}

fn run(steps: u32, factory: &Factory, start: &State) -> u32 {
    let mut current_gen = vec!(start.to_owned());
    for step in 0..steps {
        println!("Step {}", step);
        let mut next_gen = HashSet::new();

        for state in current_gen {
            let actions = get_possible_actions(&state, &factory);
            
            let new_stock = gather(&state);

            for action in actions {
                let stock_left = pay(&new_stock, action.1);
                let new_state = State {
                    robots: add_robot(&state.robots, action.0),
                    stock: stock_left,
                };
                next_gen.insert(new_state);
            }

            let idle_state = State {
                robots: state.robots.clone(),
                stock: new_stock,
            };
            next_gen.insert(idle_state);
        }

        let mut next_v: Vec<State> = next_gen.into_iter().collect();
        next_v.sort_by_key(|a| a.robots.len());
        next_v.sort_by_key(|a| *a.stock.get(&Res::Geode).unwrap_or(&0));
        current_gen = next_v.into_iter().rev().take(200000).collect();
    }

    let score: &u32 = current_gen.iter()
        .map(|a| a.stock.get(&Res::Geode).unwrap_or(&0))
        .max()
        .unwrap();

    println!("{}", score);
    *score
}

fn add_robot(robots: &HashMap<Res, u32>, res: &Res) -> HashMap<Res, u32> {
    let current = robots.get(res).unwrap_or(&0);
    let mut new = robots.clone();
    new.insert(*res, current + 1);
    new
}

fn pay(stock: &HashMap<Res, u32>, cost: &HashMap<Res, u32>) -> HashMap<Res, u32> {
    let mut left = stock.clone();
    for (res, value) in cost.iter() {
        left.insert(*res, left[&res] - value);
    }
    left
}

fn gather(state: &State) -> HashMap<Res, u32> {
    state.robots.iter()
        .map(|(res, count)| 
            (*res, count + state.stock.get(res).unwrap_or(&0))
        )
        .collect()
}

fn get_possible_actions<'a>(
    state: &'a State,
    factory: &'a Factory
) -> Vec<(&'a Res, &'a HashMap<Res, u32>)> {
    factory.cost.iter()
        .filter(|(res, cost)| {
            cost.iter()
                .all(|(k, v)| 
                    state.stock.get(k).unwrap_or(&0) >= v
                )
        })
        .collect()
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct State {
    robots: HashMap<Res, u32>,
    stock: HashMap<Res, u32>,
}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut r: Vec<_> = self.robots.iter().collect();
        r.sort_by_key(|a| a.0);

        let mut s: Vec<_> = self.stock.iter().collect();
        s.sort_by_key(|a| a.0);

        Hash::hash(&r, state);
        Hash::hash(&s, state);
    }
}

#[derive(Debug)]
struct Factory {
    cost: HashMap<Res, HashMap<Res, u32>>
}

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Res {
    Ore,
    Clay,
    Obsidian,
    Geode
}

fn parse_line(line: &str) -> Vec<Vec<u32>> {
    let body: Vec<&str> = line.split(':').collect();
    let parts: Vec<&str> = body[1].split('.').collect();

    parts.iter()
        .map(|part| part.split(' ')
            .filter_map(|a| a.parse::<u32>().ok())
            .collect()
        )
        .collect()
}