use std::{
    collections::{HashSet, HashMap, VecDeque},
    fmt,
    fs,
    ops::{Add, Div, Mul, Sub}
};

const INPUT_PATH: &str = "inputs/021.txt";

fn main() {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();

    let mut root_a = "";
    let mut root_b = "";

    let mut monkeys: VecDeque<Monkey> = file_str.split('\n')
        .map(|l| {
            let parts = l.split(": ").collect::<Vec<&str>>();
            match parts[1].parse::<f64>() {
                Ok(val) => {
                    let elements = match parts[0] {
                        "humn" => HashMap::from([(1, 1.)]),
                        _ => HashMap::from([(0, val)])
                    };
                    Monkey::Numeric(
                        parts[0],
                        Value { elements }
                    )
                },
                _ => {
                    let op = parts[1].split(' ').collect::<Vec<&str>>();
                    if parts[0] == "root" {
                        root_a = op[0];
                        root_b = op[2];
                    }
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
                if name == "root" { continue; }
                match (cache.get(op.a), cache.get(op.b)) {
                    (Some(a), Some(b)) => { cache.insert(name, op.calc(a, b)); },
                    _ => monkeys.push_back(cur)
                }
            }
        }
    }

    // solve the equation manually, it's only x^2 :p
    println!("Root: {} = {}", cache[root_a], cache[root_b]);
}

#[derive(Debug)]
enum Monkey<'a> {
    Numeric(&'a str, Value),
    Operator(&'a str, Operation<'a>)
}

#[derive(Clone, Copy, Debug)]
struct Operation<'a> {
    pub a: &'a str,
    pub b: &'a str,
    pub op: char
}

impl Operation<'_> {
    pub fn calc(&self, a: &Value, b: &Value) -> Value {
        match self.op {
            '+' => a + b,
            '-' => a - b,
            '*' => a * b,
            '/' => a / b,
            _ => panic!()
        }
    }
}

#[derive(Debug)]
struct Value {
    pub elements: HashMap<i64, f64>
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.elements.iter()
            .map(|(k, v)| format!("{}x^{}", v, k))
            .collect::<Vec<String>>()
            .join(" + ");
        write!(f, "{}", s)
    }
}

impl Value {
    fn combine_keys<'a>(&'a self, other: &'a Value) -> HashSet<&'a i64> {
        let mut keys: HashSet<&i64> = self.elements
            .keys().collect();
        keys.extend(
            other.elements.keys().collect::<HashSet<&i64>>()
        );
        keys
    }
}

impl<'a, 'b>  Add<&'a Value> for &'b Value {
    type Output = Value;

    fn add(self, other: &Value) -> Value {
        let mut output = HashMap::new();
        let keys = self.combine_keys(&other);

        for key in keys {
            let a = self.elements.get(key).unwrap_or(&0.);
            let b = other.elements.get(key).unwrap_or(&0.);
            output.insert(*key, *a + *b);
        }
        
        Value { elements: output }
    }
}

impl<'a, 'b>  Sub<&'a Value> for &'b Value {
    type Output = Value;

    fn sub(self, other: &Value) -> Value {
        let mut output = HashMap::new();
        let keys = self.combine_keys(&other);

        for key in keys {
            let a = self.elements.get(key).unwrap_or(&0.);
            let b = other.elements.get(key).unwrap_or(&0.);
            output.insert(*key, *a - *b);
        }
        
        Value { elements: output }
    }
}

impl<'a, 'b>  Mul<&'a Value> for &'b Value {
    type Output = Value;

    fn mul(self, other: &Value) -> Value {
        let mut output = HashMap::new();

        for key_a in self.elements.keys() {
            for key_b in other.elements.keys() {
                let key = key_a + key_b;

                let a = self.elements.get(key_a).unwrap_or(&1.);
                let b = other.elements.get(key_b).unwrap_or(&1.);
                let val = a * b;

                output.insert(
                    key,
                    *output.get(&key).unwrap_or(&0.) + val
                );
            }
        }
        Value { elements: output }
    }
}

impl<'a, 'b>  Div<&'a Value> for &'b Value {
    type Output = Value;

    fn div(self, other: &Value) -> Value {
        let mut output = HashMap::new();

        for key_a in self.elements.keys() {
            for key_b in other.elements.keys() {
                let key = key_a - key_b;

                let a = self.elements.get(key_a).unwrap_or(&1.);
                let b = other.elements.get(key_b).unwrap_or(&1.);
                let val = a / b;

                output.insert(
                    key,
                    *output.get(&key).unwrap_or(&0.) + val
                );
            }
        }
        Value { elements: output }
    }
}
