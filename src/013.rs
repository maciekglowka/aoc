use std::{
    cmp::{max, Ordering},
    fs
};

const INPUT_PATH: &str = "inputs/013.txt";

fn main () {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();
    let pairs = file_str.split("\n\n").collect::<Vec<&str>>();

    let mut sum = 0;
    let mut packets = Vec::new();

    for (i, pair) in pairs.iter().enumerate() {
        let vals = pair.split('\n').collect::<Vec<&str>>();
        if let Ordering::Less = list_cmp(&as_list(vals[0]), &as_list(vals[1])) {
            sum += i + 1 
        }

        packets.push(as_list(vals[0]));
        packets.push(as_list(vals[1]));
    }

    println!("{}", sum);

    let p0 = Value::List(vec!(Value::List(vec!(Value::Number(2)))));
    let p1 = Value::List(vec!(Value::List(vec!(Value::Number(6)))));
    packets.push(p0.clone());
    packets.push(p1.clone());

    packets.sort_by(|a, b| list_cmp(a, b));
    
    let mut key = 1;
    for (i, p) in packets.iter().enumerate() {
        if list_cmp(p, &p0) == Ordering::Equal || list_cmp(p, &p1) == Ordering::Equal
            { key *= i + 1 }
    }
    println!("{}", key);
}

fn list_cmp(val_a: &Value, val_b: &Value) -> Ordering {
    let a = match val_a {
        Value::List(v) => v,
        _ => panic!()
    };
    let b = match val_b {
        Value::List(v) => v,
        _ => panic!()
    };
    let length = max(a.len(), b.len());

    for i in 0..length {
        if i >= a.len() { return Ordering::Less; }
        if i >= b.len() { return Ordering::Greater; }

        match (&a[i], &b[i]) {
            (&Value::Number(na), &Value::Number(nb)) => {
                if na < nb { return Ordering::Less; }
                if na > nb { return Ordering::Greater; }
            },
            (Value::Number(na), Value::List(_)) => {
                let va = Value::List(vec!(a[i].clone()));
                let res = list_cmp(&va, &b[i].clone());
                match res {
                    Ordering::Less | Ordering::Greater => return res,
                    _ => ()
                }
            },
            (Value::List(_), Value::Number(nb)) => {
                let vb = Value::List(vec!(b[i].clone()));
                let res = list_cmp(&a[i].clone(), &vb);
                match res {
                    Ordering::Less | Ordering::Greater => return res,
                    _ => ()
                }
            },
            (&Value::List(_), &Value::List(_)) => {
                let res = list_cmp(&a[i].clone(), &b[i].clone());
                match res {
                    Ordering::Less | Ordering::Greater => return res,
                    _ => ()
                }
            }
            _ => ()
        }
    }
    Ordering::Equal
}

#[derive(Clone, Debug)]
enum Value {
    List(Vec<Value>),
    Number(usize)
}

fn from_str(s: &str) -> Value {
    as_list(&s.replace(' ', ""))
}

fn as_list(s: &str) -> Value {
    if s.len() < 3 { return Value::List(Vec::new()); }
    let trim = &s[1..s.len()-1];
    let mut bracket_counter = 0;
    let mut str_values = Vec::new();
    let mut buf = String::new();

    for c in trim.chars() {
        if c == ',' && bracket_counter == 0 {
            str_values.push(buf);
            buf = String::new();
        } else {
            buf.push(c);
            match c {
                '[' => bracket_counter += 1,
                ']' => bracket_counter -= 1,
                _ => ()
            }
        }
    }
    str_values.push(buf);

    let values: Vec<Value> = str_values.iter()
        .map(|v| {
            if let Ok(n) = v.parse::<usize>() {
                Value::Number(n)
            } else {
                as_list(v)
            }
        })
        .collect();
    Value::List(values)
}