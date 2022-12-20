use std::{
    fs
};

const INPUT_PATH: &str = "inputs/020.txt";

fn main() {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();

    let nums: Vec<i64> = file_str.split('\n')
        .flat_map(|l| l.parse::<i64>())
        .collect();

    let mut nodes: Vec<Node> = nums.iter()
        .enumerate()
        .map(|(i, val)| {
            let prev = match i {
                a if a == 0 => nums.len() - 1,
                _ => i - 1
            };
            let next = match i {
                a if a == nums.len() - 1 => 0,
                _ => i + 1
            };
            Node { val: *val, next, prev }
        })
        .collect();

    let mut nodes_2: Vec<Node> = nodes.iter()
        .map(|a| Node { val: a.val * 811589153, prev: a.prev, next: a.next})
        .collect();

    mix(&mut nodes);
    get_score(&nodes);
    
    for step in 0..10 {
        mix(&mut nodes_2);
    }
    get_score(&nodes_2);
}

#[derive(Clone, Debug)]
struct Node {
    pub val: i64,
    pub next: usize,
    pub prev: usize,
}

fn get_score(nodes: &Vec<Node>) {
    let n0 = nodes.iter()
        .find(|a| a.val == 0)
        .unwrap();

    let mut sum = 0;
    for shift in [1000, 2000, 3000] {
        let (p, _) = traverse((n0.prev, n0.next), nodes, shift, true);
        sum += nodes[p].val;
    }
    println!("{}", sum);
}

fn mix(nodes: &mut Vec<Node>) {
    for idx in 0..nodes.len() {
        displace(idx, nodes[idx].val, nodes);
    }
}

fn displace(idx: usize, shift: i64, nodes: &mut Vec<Node>) {
    if shift == 0 { return; }
    let mut n = reduce_shift(shift.abs(), nodes.len() as i64);
    let forward = shift > 0;

    // pop - we never go around more than the total count so it is safe to do so now
    let a_prev = nodes[idx].prev;
    let a_next = nodes[idx].next;
    nodes[a_prev].next = a_next;
    nodes[a_next].prev = a_prev;

    let (prev, next) = traverse((a_prev, a_next), nodes, n, forward);

    // insert
    nodes[prev].next = idx;
    nodes[next].prev = idx;
    nodes[idx].prev = prev;
    nodes[idx].next = next;
}

fn reduce_shift(shift: i64, len: i64) -> i64 {
    if shift <= len { return shift; }

    let r = shift % len;
    let n = r + shift / len;
    reduce_shift(n, len)
}

fn traverse(
    start: (usize, usize),
    nodes: &Vec<Node>,
    n: i64,
    forward: bool
) -> (usize, usize) {
    let mut prev = start.0;
    let mut next = start.1;

    for _ in 0..n {
        if forward {
            prev = next;
            next = nodes[next].next;
        } else {
            next = prev;
            prev = nodes[prev].prev;
        }
    }
    (prev, next)
}

fn linked_print(nodes: &Vec<Node>, start: usize) {
    let mut cur = start;

    loop {
        print!("{}, ", nodes[cur].val);
        cur = nodes[cur].next;
        if cur == start { break; }
    }
    print!("{}", '\n');
}