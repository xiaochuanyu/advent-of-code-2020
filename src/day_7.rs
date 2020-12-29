use crate::util;
use std::io::BufRead;
use std::collections::HashMap;

type Graph = HashMap<String, Vec<String>>;
type Memory = HashMap<String, bool>;

fn dfs(node: &str, graph: &Graph, mem: & mut Memory) -> bool{
    if let Some(&result) = mem.get(node) {
        return result;
    }
    if let Some(neighbors) = graph.get(node) {
        for n in neighbors {
            if n == "shinygold" || dfs(n, graph, mem) {
                mem.insert(node.to_owned(), true);
                return true;
            }
        }
    }
    mem.insert(node.to_owned(), false);
    false
}

pub fn part_1() {
    let file = util::read_input();
    let mut mem: Memory = HashMap::new();
    let mut graph: Graph = HashMap::new();
    for line in file.lines().map(|result| result.unwrap()) {
        let split: Vec<&str> = line.split(" ").collect();
        let from_bag = &split[0..2].join("");
        if split[4] == "no" {
            // Example:
            // pale yellow bags contain no other bags.
            continue;
        } else {
            // Example:
            // shiny lime bags contain 3 muted magenta bags, 3 clear cyan bags.
            let mut i = 4;
            let mut to_bags: Vec<String> = Vec::new();
            while i < split.len() {
                // 3 muted magenta bags,
                // i +1    +2      +3
                let to_bag = &split[i+1..i+3].join("");
                to_bags.push(to_bag.to_owned());
                i += 4;
            }
            graph.insert(from_bag.to_owned(), to_bags);
        }
    }
    let mut answer = 0;
    for from_bag in graph.keys() {
        if dfs(from_bag, &graph, & mut mem) {
            answer += 1;
        }
    }
    println!("{}", answer);
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

