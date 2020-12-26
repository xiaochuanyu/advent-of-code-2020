use std::io::BufRead;
use crate::util;

pub fn part_1() {
    let file = util::read_input();
    let mut col = 0;
    let mut trees = 0;
    for line in file.lines().map(|result| result.unwrap()) {
        let length = line.len();
        if line.chars().nth(col).unwrap() == '#' {
            trees += 1;
        }
        col = (col + 3) % length;
    }
    println!("{}", trees);
}

fn check_trees(map: &Vec<Vec<char>>, right: usize, down: usize) -> usize {
    let mut row = 0;
    let mut col = 0;
    let mut trees = 0;
    while row < map.len() {
        let line = &map[row];
        if line[col] == '#' {
            trees += 1;
        }
        col = (col + right) % line.len();
        row = row + down;
    }
    trees
}

pub fn part_2() {
    let file = util::read_input();
    let lines: Vec<Vec<char>> = file
        .lines()
        .map(|line| line.unwrap().chars().collect()) // line -> Vec<char>
        .collect();
    let slopes: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut trees = 1;
    for (right, down) in &slopes {
        trees *= check_trees(&lines, *right, *down);
    }
    println!("{}", trees);
}

