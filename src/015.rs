use std::fs;

const INPUT_PATH: &str = "inputs/015.txt";
// const TARGET_LINE: i64 = 2000000;
// const TARGET_LINE: i64 = 10;
// const MAX_DIM: i64 = 20;
const MAX_DIM: i64 = 4000000;

fn main () {
    let file_str = fs::read_to_string(INPUT_PATH).unwrap();
    // let mut excluded = HashSet::new();
    let points: Vec<[i64; 4]> = file_str.split('\n')
        .map(|l| {
            let [x0, y0, x1, y1] = parse_line(l);
            // if y1 == TARGET_LINE { excluded.insert(x1); }
            [x0, y0, x1, y1]
        })
        .collect();

    // let mut elements = Vec::new();

    // for [x0, y0, x1, y1] in points {
    //     let dist = manhattan(x0, y0, x1, y1);
    //     let dy = (y0-TARGET_LINE).abs();
    //     if dy > dist { continue; }

    //     let dx = dist - dy;
    //     elements.push((x0-dx, x0+dx));
    // }

    // let range = Range::new(elements);
    // println!("Range: {:?}", range);

    // let s: i64 = range.elements.iter()
    //     .map(|e| e.1 - e.0)
    //     .sum();

    // println!("Sum: {:?}", s);

    for y in 0..MAX_DIM {
        let mut elements = Vec::new();
        
        for [x0, y0, x1, y1] in points.iter() {
            let dist = manhattan(*x0, *y0, *x1, *y1);
            let dy = (y0-y).abs();
            if dy > dist { continue }
    
            let dx = dist - dy;
            elements.push((x0-dx, x0+dx));
        }
        if elements.len() == 0 { continue; }
        let range = Range::new(elements);

        for e in range.elements {
            if e.1 <= 0 { continue; }
            if e.0 >= MAX_DIM { continue; }
            if e.0 <= 0 && e.1 >= MAX_DIM { continue; }
            if e.1 >= MAX_DIM { continue; }
            if e.1 < -1 { continue; }

            println!("End: {} {} for {}", e.1 + 1, 4000000 * (e.1 + 1) + y, y);
        }
    }
}

fn parse_line(line: &str) -> [i64; 4] {
    // Ugly, ugly, ugly
    let parts: Vec<&str> = line.split(',').collect();

    let x0 = parts[0].split('=').collect::<Vec<&str>>()[1].parse::<i64>().unwrap();

    let middle = parts[1].split(':').collect::<Vec<&str>>();
    let y0 = middle[0].split('=').collect::<Vec<&str>>()[1].parse::<i64>().unwrap();

    let x1 = middle[1].split('=').collect::<Vec<&str>>()[1].parse::<i64>().unwrap();
    let y1 = parts[2].split('=').collect::<Vec<&str>>()[1].parse::<i64>().unwrap();

    [x0, y0, x1, y1]
}

pub fn manhattan(x0: i64, y0: i64, x1: i64, y1: i64) -> i64 {
    (x0 - x1).abs() + (y0 - y1).abs()
}

#[derive(Debug)]
struct Range {
    pub elements: Vec<(i64, i64)>
}

impl Range {
    pub fn new(mut input: Vec<(i64, i64)>) -> Range {
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