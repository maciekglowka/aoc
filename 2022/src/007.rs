use std::{
    cell::RefCell,
    collections::HashMap,
    fs,
    rc::{Rc, Weak}
};

#[derive(Debug)]
struct Node {
    pub parent: RefCell<Weak<Node>>,
    pub children: RefCell<HashMap<String, Rc<Node>>>,
    pub size: u32
}

impl Node {
    pub fn new(size: u32) -> Node {
        Node {
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(HashMap::new()),
            size
        }
    }
    pub fn insert(&self, name: String, node: &Rc<Node>) {
        self.children.borrow_mut().insert(name, Rc::clone(node));
    }
    pub fn get_parent(&self) -> Weak<Node> {
        Weak::clone(&self.parent.borrow())
    }
    pub fn get_child(&self, name: &str) -> Weak<Node> {
        Weak::clone(
            &Rc::downgrade(self.children.borrow().get(name).unwrap())
        )
    }
}

const INPUT_PATH: &str = "inputs/007.txt";

fn main () {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();
    let lines: Vec<&str> = file_str.split('$').collect();

    let mut root = Rc::new(Node::new(0));
    let mut cur = Rc::clone(&root);

    for line in lines {
        cur = command(line, cur);
    }

    let mut sizes = Vec::new();
    let res = find_sizes_below(&root, sizes);
    sizes = res.1;
    let root_size = res.0;
    
    let free_size = 70000000 - root_size;
    let needed_size = 30000000 - free_size;
    println!("Low sum: {}", sizes.iter().sum::<u32>());

    let mut above_sizes = Vec::new();
    let above_res = find_sizes_above(&root, above_sizes, needed_size);
    above_sizes = above_res.1;
    above_sizes.sort();
    println!("{:?}", above_sizes);
}

fn command(line: &str, cur: Rc<Node>) -> Rc<Node> {
    if line.len() < 3 { return cur }
    match &line[1..3] {
        "cd" => return cd(line, &cur),
        "ls" => ls(line, &cur),
        _ => ()
    };
    cur
}

fn cd(line: &str, cur: &Rc<Node>) -> Rc<Node> {
    let args: Vec<&str> = line.split_whitespace().collect();
    if args.len() < 2 { return Rc::clone(cur) }

    match args[1] {
        "/" => get_root(cur).upgrade().unwrap(),
        ".." => cur.get_parent().upgrade().unwrap(),
        a => cur.get_child(a).upgrade().unwrap()
    }
}

fn ls(line: &str, cur: &Rc<Node>) {
    let args: Vec<&str> = line.split('\n').collect();
    if args.len() < 2 { return }

    for arg in &args[1..] {
        let parts: Vec<&str> = arg.split_whitespace().collect();
        if parts.len() < 2 { continue }
        let size = match parts[0] {
            "dir" => 0,
            a => a.parse::<u32>().unwrap()
        };
        let n = Rc::new(Node::new(size));
        *n.parent.borrow_mut() = Rc::downgrade(cur);
        cur.insert(parts[1].to_string(), &n);
    }
}

fn get_root(node: &Rc<Node>) -> Weak<Node> {
    let mut cur = Rc::clone(node);
    loop {
        cur = match cur.get_parent().upgrade() {
            Some(n) => n,
            None => return Rc::downgrade(&cur)
        }
    }
}

fn find_sizes_below(node: &Rc<Node>, mut sizes: Vec<u32>) -> (u32, Vec<u32>) {
    let mut size = 0;
    
    for child in node.children.borrow().values() {
        match child.size {
            0 => {
                let res = find_sizes_below(child, sizes);
                size += res.0;
                sizes = res.1;
            },
            a => size += a
        }
    }
    if size < 100000 { sizes.push(size); }
    (size, sizes)
}

fn find_sizes_above(node: &Rc<Node>, mut sizes: Vec<u32>, thresh: u32) -> (u32, Vec<u32>) {
    let mut size = 0;
    
    for child in node.children.borrow().values() {
        match child.size {
            0 => {
                let res = find_sizes_above(child, sizes, thresh);
                size += res.0;
                sizes = res.1;
            },
            a => size += a
        }
    }
    if size >= thresh { sizes.push(size); }
    (size, sizes)
}