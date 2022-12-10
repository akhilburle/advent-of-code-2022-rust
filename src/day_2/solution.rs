use std::{fs, path::Path};
#[allow(dead_code)]

pub fn solution_1() -> i32 {
    let path = Path::new("./src/day_2/input.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let split_contents: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();

    let mut shape_score = 0;
    let mut round_score = 0;

    for round in split_contents {
        let (opponent, me) = round.split_once(" ").unwrap();
        let opponent = (opponent.chars().nth(0).unwrap() as i32) - ('A' as i32);
        let me = (me.chars().nth(0).unwrap() as i32) - ('X' as i32);
        shape_score += me + 1;
        let result = (me - opponent).rem_euclid(3);
        match result {
            1 => round_score += 6,
            2 => round_score += 0,
            0 => round_score += 3,
            _ => {}
        }
    }
    shape_score + round_score
}

pub fn solution_2() -> i32 {
    let path = Path::new("./src/day_2/input.txt");
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");
    let split_contents: Vec<&str> = contents.split("\n").collect::<Vec<&str>>();

    let mut shape_score = 0;
    let mut round_score = 0;

    for round in split_contents {
        let (opponent, result) = round.split_once(" ").unwrap();
        let opponent = (opponent.chars().nth(0).unwrap() as i32) - ('A' as i32);
        let result = (result.chars().nth(0).unwrap() as i32) - ('X' as i32);
        let me;
        match result {
            0 => {
                round_score += 0;
                me = (opponent - 1).rem_euclid(3)
            }
            1 => {
                round_score += 3;
                me = opponent;
            }
            2 => {
                round_score += 6;
                me = (opponent + 1).rem_euclid(3)
            }
            _ => me = 0,
        }
        shape_score += me + 1;
    }
    shape_score + round_score
}
