use std::{collections::HashMap, collections::BTreeMap, fs, path::Path};

use regex::Regex;
#[allow(dead_code)]

fn parse_header(header: &[&str], stacks: &mut HashMap<i32, Vec<char>>) {
    for row in header {
        for (idx, c) in row.chars().enumerate() {
            if c.is_ascii_uppercase() {
                stacks
                    .entry(((idx + 3) / 4) as i32)
                    .or_insert_with(|| Vec::<char>::new())
                    .insert(0, c);
            }
        }
    }
}

pub fn solution_1() -> String {
    let path = Path::new("./src/day_5/input.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let split_contents: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();
    let (header, mut body) =
        split_contents.split_at(split_contents.iter().position(|x| *x == "").unwrap());

    let mut data = HashMap::<i32, Vec<char>>::new();
    parse_header(header, &mut data);
    body = &body[1..];
    let instr_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for instr in body {
        let matches = instr_regex.captures(instr).unwrap();
        let (num, src, dst) = (
            matches[1].parse().unwrap(),
            matches[2].parse().unwrap(),
            matches[3].parse().unwrap(),
        );
        for _ in 0..num {
            let payload = data.get_mut(&src).unwrap().pop().unwrap();
            data.get_mut(&dst).unwrap().push(payload);
        }
    }
    let mut ans: String = "".to_string();
    let data: BTreeMap<_, _> = data.into_iter().collect();
    for (_, val) in data {
        ans.push(*val.last().unwrap());
    }
    ans
}

pub fn solution_2() -> String {
    let path = Path::new("./src/day_5/input.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let split_contents: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();
    let (header, mut body) =
        split_contents.split_at(split_contents.iter().position(|x| *x == "").unwrap());

    let mut data = HashMap::<i32, Vec<char>>::new();
    parse_header(header, &mut data);
    body = &body[1..];
    let instr_regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for instr in body {
        let matches = instr_regex.captures(instr).unwrap();
        let (num, src, dst) = (
            matches[1].parse().unwrap(),
            matches[2].parse().unwrap(),
            matches[3].parse().unwrap(),
        );
        let mut temp = Vec::<char>::new();
        for _ in 0..num {
            let payload = data.get_mut(&src).unwrap().pop().unwrap();
            temp.push(payload);
        }
        for _ in 0..num {
            let payload = temp.pop().unwrap();
            data.get_mut(&dst).unwrap().push(payload);
        }
    }
    let mut ans: String = "".to_string();
    let data: BTreeMap<_, _> = data.into_iter().collect();
    for (_, val) in data {
        ans.push(*val.last().unwrap());
    }
    ans
}
