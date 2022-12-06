use std::fs;

const INPUT_PATH: &str = "inputs/005.txt";

fn main() {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();
    let parts: Vec<&str> = file_str.split("\n\n").collect();

    let stack_count = parts[0].split('\n')
        .last()
        .unwrap()
        .split_whitespace()
        .map(|a| a.parse::<usize>().unwrap())
        .last()
        .unwrap();

    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_count];

    let container_lines: Vec<String> = parts[0].split('\n')
        .collect::<Vec<&str>>()
        .split_last()
        .unwrap()
        .1
        .iter()
        .map(|a| a.to_string())
        .collect();
    
    for idx in 0..stack_count {
        let s: Vec<char> = container_lines.iter()
            .filter_map(|l| {
                let chars: Vec<char> = l.chars().collect();
                let c = chars[4*idx+1];
                match c.is_alphabetic() {
                    true => Some(c),
                    false => None
                }           
            })
            .rev()
            .collect();
        stacks[idx] = s;
    }

    let commands: Vec<Vec<usize>> = parts[1].split('\n')
        .map(|l| {
            l.split_whitespace()
                .filter_map(|a| match a.parse::<usize>() {
                    Ok(n) => Some(n),
                    _ => None
                })
                .collect()
        })
        .collect();

    for command in commands {
        if command.len() > 0 {
            move_containers(&mut stacks, &command);
        }
    }

    let output: String = stacks.iter()
        .map(|s| s.last().unwrap())
        .collect();

    println!("{:?}", output);
}

fn move_containers(stacks: &mut Vec<Vec<char>>, command: &Vec<usize>) {
    move_combined(stacks, command[0], command[1] - 1, command[2] - 1);
    // for _ in 0..command[0] {
    //     move_single(stacks, command[1] - 1, command[2] - 1);
    // }
}

fn move_single(stacks: & mut Vec<Vec<char>>, from: usize, to: usize) {
    let c = stacks[from].pop().unwrap();
    stacks[to].push(c);
}

fn move_combined(
    stacks: & mut Vec<Vec<char>>,
    count: usize,
    from: usize,
    to: usize
) {
    let split = stacks[from].len() - count;
    let v = &stacks[from][split..].to_owned();
    stacks[from].truncate(split);
    stacks[to].extend(v);
}
