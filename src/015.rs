use std::fs;

const INPUT_PATH: &str = "inputs/015.txt";
const TARGET_LINE: i32 = 2000000;
// const TARGET_LINE: i32 = 10;

fn main () {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();
    let mut excluded = HashSet::new();
    let points: Vec<[i32; 4]> = file_str.split('\n')
        .map(|l| {
            let [x0, y0, x1, y1] = parse_line(l);
            if y1 == TARGET_LINE { excluded.insert(x1); }
            [x0, y0, x1, y1]
        })
        .collect();

    let mut elements = Vec::new();

    for [x0, y0, x1, y1] in points {
        let dist = manhattan(x0, y0, x1, y1);
        let dy = (y0-TARGET_LINE).abs();
        if dy > dist { continue; }

        let dx = dist - dy;
        elements.push((x0-dx, x0+dx));
    }

    let range = Range::new(elements);
    println!("Range: {:?}", range);

    let s: i32 = range.elements.iter()
        .map(|e| e.1 - e.0)
        .sum();

    println!("Sum: {:?}", s);

    // for x in excluded {
    //     set.remove(&x);
    // }
    // println!("{:?}", set.len());
}

fn parse_line(line: &str) -> [i32; 4] {
    // Ugly, ugly, ugly
    let parts: Vec<&str> = line.split(',').collect();

    let x0 = parts[0].split('=').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

    let middle = parts[1].split(':').collect::<Vec<&str>>();
    let y0 = middle[0].split('=').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

    let x1 = middle[1].split('=').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
    let y1 = parts[2].split('=').collect::<Vec<&str>>()[1].parse::<i32>().unwrap();

    [x0, y0, x1, y1]
}

pub fn manhattan(x0: i32, y0: i32, x1: i32, y1: i32) -> i32 {
    (x0 - x1).abs() + (y0 - y1).abs()
}

#[derive(Debug)]
struct Range {
    pub elements: Vec<(i32, i32)>
}

impl Range {
    pub fn new(mut input: Vec<(i32, i32)>) -> Range {
        input.sort_by_key(|a| a.0);
        let mut elements = vec!(input[0]);

        for element in input[1..].iter() {
            if elements.last().unwrap().1 < element.0 {
                elements.push(*element);
                continue;
            }
            if element.1 <= elements.last().unwrap().1 { continue; }

            elements.last_mut().unwrap().1 = element.1;
        }

        Range { elements }
    }
}