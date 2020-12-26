use regex::Regex;
use crate::util;
use std::io::BufRead;

pub fn part_1() {
    let file = util::read_input();
    // 5-6 d: ddddddb
    let re = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z0-9]+)").unwrap();

    let mut valid_passwords = 0;
    for line in file.lines().map(|result| result.unwrap()) {
        let cap = re.captures(&line).unwrap();
        let min_count = cap[1].parse::<i32>().unwrap();
        let max_count = cap[2].parse::<i32>().unwrap();
        let required_char = cap[3].chars().nth(0).unwrap();
        let password = &cap[4];
        let mut count = 0;

        for c in password.chars() {
            if c == required_char {
                count += 1;
            }
        }

        if count >= min_count && count <= max_count {
            valid_passwords += 1;
        }
    }
    println!("{}", valid_passwords);
}

pub fn part_2() {
    let file = util::read_input();
    // 5-6 d: ddddddb
    let re = Regex::new(r"([0-9]+)-([0-9]+) ([a-z]): ([a-z0-9]+)").unwrap();

    let mut valid_passwords = 0;
    for line in file.lines().map(|result| result.unwrap()) {
        let cap = re.captures(&line).unwrap();
        let pos_1 = cap[1].parse::<usize>().unwrap() - 1;
        let pos_2 = cap[2].parse::<usize>().unwrap() - 1;
        let required_char = cap[3].chars().nth(0).unwrap();
        let password = &cap[4];
        let mut count = 0;
        for (i, c) in password.chars().enumerate() {
            if (i == pos_1 || i == pos_2) && c == required_char {
                count += 1;
            }
            if i > pos_2 {
                break;
            }
        }
        if count == 1 {
            valid_passwords += 1;
        }
    }
    println!("{}", valid_passwords);
}

