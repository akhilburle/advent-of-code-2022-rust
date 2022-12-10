use std::{fs, path::Path};
#[allow(dead_code)]

pub fn solution_1() -> i32 {
    let path = Path::new("./src/day_1/input.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let split_contents: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();
    let mut accumulator = 0;
    let mut max = 0;
    for food in split_contents {
        if food.is_empty() {
            if accumulator > max {
                max = accumulator;
            }
            accumulator = 0;
            continue;
        }
        accumulator += food.parse::<i32>().unwrap();
    }
    max
}

pub fn solution_2() -> i32 {
    let path = Path::new("./src/day_1/input.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    let mut split_contents: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();
    split_contents.push("");
    let mut accumulator = 0;
    let mut max = [0, 0, 0];
    let size = max.len();
    for food in split_contents {
        if !food.is_empty() {
            accumulator += food.parse::<i32>().unwrap();
            continue;
        }
        // for (i, max_elem) in max.iter_mut().enumerate().rev() {
        for idx in (0..size).rev() {
            // let idx = size - i - 1;
            let max_elem = max[idx];
            if accumulator >= max_elem {
                for idx_to_move in 0..idx {
                    max[idx_to_move] = max[idx_to_move + 1]
                }
                max[idx] = accumulator;
                break;
            }
        }
        accumulator = 0;
    }
    max.iter().sum::<i32>()
}
