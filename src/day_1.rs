use crate::util;
use std::collections::HashSet;
use std::io::BufRead;

pub fn part_1() {
    let file = util::read_input();
    let mut numbers = HashSet::new();
    for line in file.lines().map(|result| result.unwrap()) {
        let number = line.parse::<i32>().expect("could not parse line to in");
        let candidate = 2020 - number;
        if numbers.contains(&candidate) {
            println!("{}", number * candidate);
            return;
        } else {
            numbers.insert(number);
        }
    }
}

pub fn part_2() {
    let file = util::read_input();
    let mut numbers = Vec::new();
    for line in file.lines().map(|result| result.unwrap()) {
        let number = line.parse::<i32>().expect("could not parse line to in");
        numbers.push(number);
    }
    numbers.sort();
    let mut l = 0;
    let mut r = numbers.len() - 1;
    while l < r - 1 {
        let mut m = l + 1;
        while m < r {
            let sum = numbers[l] + numbers[m] + numbers[r];
            if sum == 2020 {
                println!("{}", numbers[l] * numbers[m] * numbers[r]);
                return;
            } else if sum > 2020 {
                r -= 1;
                break;
            }
            m += 1;
        }
        if m == r - 1 {
            l += 1;
        }
    }
}
