use std::{
    fs,
};

const INPUT_PATH: &str = "inputs/010.txt";


enum Command {
    Noop,
    AddX(u32, i32)
}


fn main () {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();
    
    let mut commands: Vec<Command> = file_str.split('\n')
        .map(|l| {
            let parts: Vec<&str> = l.split_whitespace().collect();
            match parts[0] {
                "addx" => Command::AddX(2, parts[1].parse::<i32>().unwrap()),
                _ => Command::Noop,
            }
        })
        .rev()
        .collect();

    let mut cycle = 1;
    let mut reg_x = 1;
    let mut cur = commands.pop().unwrap();
    // let mut v = Vec::new();

    let mut screen: Vec<Vec<char>> = Vec::new();
    let mut buf: Vec<char> = vec!('S');

    while commands.len() > 0 {
        match cur {
            Command::Noop => cur = commands.pop().unwrap(),
            Command::AddX(1, val) => {
                reg_x += val;
                cur = commands.pop().unwrap();
            },
            Command::AddX(2, val) => cur = Command::AddX(1, val),
            _ => panic!()
        }

        let buf_x = cycle % 40;
        if buf_x == 0 {
            screen.push(buf.clone());
            buf = Vec::new();
        }

        if buf_x >= reg_x - 1 && buf_x <= reg_x + 1 {
            buf.push('#')
        } else {
            buf.push('.')
        }

        cycle += 1;

        // if cycle >= 20 && (cycle - 20) % 40 == 0 {
        //     v.push(cycle * reg_x);
        // }
    }

    screen.push(buf);

    // println!("{}", v.iter().sum::<i32>());
    for l in screen {
        for c in l{
            print!("{}", c);
        }
        print!("\n");
    }
}