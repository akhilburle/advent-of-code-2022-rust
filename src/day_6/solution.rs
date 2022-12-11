use std::{
    collections::{HashSet, VecDeque},
    fs,
    path::Path,
};
#[allow(dead_code)]

pub fn solution_1() -> i32 {
    let path = Path::new("./src/day_6/input.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let mut deq = VecDeque::<char>::new();
    for (idx, c) in contents.chars().enumerate() {
        deq.push_back(c);
        if deq.len() > 4 {
            deq.pop_front();
        }
        let s = HashSet::<char>::from_iter(deq.iter().cloned());
        if s.len() == 4 {
            return (idx + 1) as i32;
        }
    }
    0
}

pub fn solution_2() -> i32 {
    let path = Path::new("./src/day_6/input.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let mut deq = VecDeque::<char>::new();
    for (idx, c) in contents.chars().enumerate() {
        deq.push_back(c);
        if deq.len() > 14 {
            deq.pop_front();
        }
        let s = HashSet::<char>::from_iter(deq.iter().cloned());
        if s.len() == 14 {
            return (idx + 1) as i32;
        }
    }
    0
}
