use std::{collections::HashSet, fs, path::Path};
#[allow(dead_code)]

pub fn solution_1() -> i32 {
    let path = Path::new("./src/day_3/input.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let split_contents: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();

    let mut sum = 0;
    for rucksack in split_contents {
        let (compartment_1, compartment_2) = rucksack.split_at(rucksack.len() / 2);
        let compartment_1: HashSet<char> = compartment_1.chars().collect();
        let compartment_2: HashSet<char> = compartment_2.chars().collect();
        let common = compartment_1.intersection(&compartment_2);
        let mut common = *Vec::from_iter(common)[0] as i32;
        if common > 96 {
            common -= 96;
        } else {
            common -= 38;
        }
        sum += common;
    }
    sum
}

pub fn solution_2() -> i32 {
    let path = Path::new("./src/day_3/input.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let split_contents: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();

    let mut sum = 0;
    let mut accumulator: HashSet<char> = HashSet::new();
    for (idx, rucksack) in split_contents.iter().enumerate() {
        let elem: HashSet<char> = rucksack.chars().collect();
        if idx % 3 == 0 {
            accumulator = elem;
        } else {
            accumulator = &accumulator & &elem;
        }
        if idx % 3 == 2 {
            let mut val = *Vec::from_iter(&accumulator)[0] as i32;
            if val > 96 {
                val -= 96;
            } else {
                val -= 38;
            }
            sum += val;
        }
    }
    sum
}
