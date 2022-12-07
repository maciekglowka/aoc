use std::{
    collections::HashMap,
    fs
};

#[derive(Debug)]
enum Entity {
    Dir(HashMap<String, Entity>),
    File(u32)
}

impl Entity {
    pub fn insert(&mut self, name: String, entity: Entity) {
        match self {
            Entity::Dir(dir) => {
                if !dir.contains_key(&name) { dir.insert(name, entity); }
            },
            _ => ()
        };
    }
}

const INPUT_PATH: &str = "inputs/_007.txt";

fn main () {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();
    let lines: Vec<&str> = file_str.split('$').collect();

    let mut root = Entity::Dir(HashMap::new());
    let mut path: Vec<&str> = Vec::new();
    // root.insert("a.txt".to_string(), Entity::File(8));
    // root.insert("d".to_string(), Entity::Dir(HashMap::new()));

    // println!("{:?}", root);
    for line in lines {
        command(line, &mut path, &mut root);
    }
}

fn command<'a>(line: &'a str, path: &mut Vec<&'a str>, root: &'a mut Entity) {
    if line.len() < 3 { return }
    match &line[1..3] {
        "cd" => cd(line, path),
        "ls" => ls(line, path, root),
        _ => return
    }
}

fn cd<'a>(line: &'a str, path: &mut Vec<&'a str>) {
    let args: Vec<&str> = line.split_whitespace().collect();
    if args.len() < 2 { return }

    match args[1] {
        "/" => path.clear(),
        ".." => {path.pop(); },
        a => path.push(a)
    }

    println!("CD {:?}", path);
}

fn ls<'a>(line: &'a str, path: &mut Vec<&'a str>, root: &'a mut Entity) {
    let args: Vec<&str> = line.split('\n').collect();
    if args.len() < 2 { return }
    let cur = traverse(path, root);
    println!("LS {:?}", &args[1..])
}

fn traverse<'a>(path: &mut Vec<&'a str>, root: &'a mut Entity) -> &'a mut Entity {
    root
}