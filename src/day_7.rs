use crate::util;
use std::io::BufRead;
use std::collections::HashMap;


pub fn part_1() {
    let file = util::read_input();
    let mut graph = HashMap::new();
    for line in file.lines().map(|result| result.unwrap()) {
        let split: Vec<&str> = line.split(" ").collect();
        let from_bag = &split[0..2].join(" ");
        if split[3] == "no" {
            let empty: Vec<&str> = vec![];
            graph.insert(from_bag.to_owned(), empty);
        } else {

        }
        println!("{:?}", from_bag);
    }
}

pub fn part_2() {
    let file = util::read_input();
    let mut yes_map = HashMap::new();
    let mut num_members = 0;
    let mut answer = 0;
    for line in file.lines().map(|result| result.unwrap()) {
        if line.len() == 0 {
            for val in yes_map.values() {
                if *val == num_members {
                    answer += 1;
                }
            }
            yes_map.clear();
            num_members = 0;
        } else {
            for c in line.chars() {
                if let Some(count_ref) = yes_map.get_mut(&c) {
                    *count_ref += 1;
                } else {
                    yes_map.insert(c, 1);
                }
            }
            num_members += 1;
        }
    }

    for val in yes_map.values() {
        if *val == num_members {
            answer += 1;
        }
    }
    println!("{}", answer);
}

