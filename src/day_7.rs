use crate::util;
use std::io::BufRead;
use std::collections::HashMap;

type Graph = HashMap<String, Vec<String>>;
type BoolMemory = HashMap<String, bool>;

fn dfs(node: &str, graph: &Graph, mem: & mut BoolMemory) -> bool{
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
    let mut mem: BoolMemory = HashMap::new();
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

struct Edge {
    to_node: String,
    weight: i32
}
type WeightedGraph = HashMap<String, Vec<Edge>>;
type IntMemory = HashMap<String, i32>;

fn count_bags(node: &str, graph: &WeightedGraph, mem: & mut IntMemory) -> i32{
    if let Some(&result) = mem.get(node) {
        return result;
    }
    if let Some(edges) = graph.get(node) {
        let mut count = 0;
        for e in edges {
            count += count_bags(&e.to_node, graph, mem) * e.weight + e.weight;
        }
        mem.insert(node.to_owned(), count);
        count
    } else {
        return 0;
    }
}

pub fn part_2() {
    let file = util::read_input();
    let mut mem: IntMemory = HashMap::new();
    let mut graph: WeightedGraph = HashMap::new();
    for line in file.lines().map(|result| result.unwrap()) {
        let split: Vec<&str> = line.split(" ").collect();
        let from_bag = &split[0..2].join("");
        if split[4] == "no" {
            // Example:
            // pale yellow bags contain no other bags.
            continue;
        } else {
            // Example:
            // 0     1    2    3       4 5     6       7     8 9     10   11
            // shiny lime bags contain 3 muted magenta bags, 3 clear cyan bags.
            let mut i = 4;
            let mut to_bags: Vec<Edge> = Vec::new();
            while i < split.len() {
                // 3 muted magenta bags,
                // i +1    +2      +3
                let amount: i32 = split[i].parse().unwrap();
                let to_bag = &split[i+1..i+3].join("");
                let edge = Edge {
                    to_node: to_bag.to_owned(),
                    weight: amount,
                };
                to_bags.push(edge);
                i += 4;
            }
            graph.insert(from_bag.to_owned(), to_bags);
        }
    }
    let answer = count_bags("shinygold", &graph, & mut mem);
    println!("{}", answer);
}

