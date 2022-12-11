use std::{fs, path::Path};
#[allow(dead_code)]

pub fn solution_1() -> i32 {
    let path = Path::new("./src/day_4/input.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let split_contents: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();

    let mut count = 0;
    for assignments in split_contents {
        let assignments: Vec<(&str, &str)> = assignments
            .split(',')
            .map(|x| x.split_once('-').unwrap())
            .collect();
        let assignments: Vec<(i32, i32)> = assignments
            .iter()
            .map(|pair| (pair.0.parse().unwrap(), pair.1.parse().unwrap()))
            .collect();
        let ends: Vec<i32> = assignments.iter().map(|x| x.1).collect();
        let starts: Vec<i32> = assignments.iter().map(|x| x.0).collect();
        if (starts[0] == starts[1])
            | (ends[0] == ends[1])
            | ((starts[0] < starts[1]) & (ends[0] > ends[1]))
            | ((starts[0] > starts[1]) & (ends[0] < ends[1]))
        {
            count += 1;
        }
    }
    count
}

pub fn solution_2() -> i32 {
    let path = Path::new("./src/day_4/input.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let split_contents: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();

    let mut count = 0;
    for assignments in split_contents {
        let assignments: Vec<(&str, &str)> = assignments
            .split(',')
            .map(|x| x.split_once('-').unwrap())
            .collect();
        let assignments: Vec<(i32, i32)> = assignments
            .iter()
            .map(|pair| (pair.0.parse().unwrap(), pair.1.parse().unwrap()))
            .collect();
        let ends: Vec<i32> = assignments.iter().map(|x| x.1).collect();
        let starts: Vec<i32> = assignments.iter().map(|x| x.0).collect();
        if !((starts[0] > ends[1]) | (starts[1] > ends[0])) {
            count += 1;
        }
    }
    count
}
