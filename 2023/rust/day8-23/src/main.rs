use core::fmt;
use num_integer::Integer;
use std::{collections::HashMap, str::FromStr};

fn main() {
    let mut lines = include_str!("input.txt").lines();
    let mut instructions = lines.next().unwrap().chars().cycle();
    let node_map: HashMap<String, Node> = lines
        .skip(1)
        .map(|line| {
            let node: Node = line.parse().unwrap();
            (node.clone().id, node)
        })
        .collect();

    // let mut current_node_id = "AAA".to_string();
    // let mut steps = 0;
    // while current_node_id != "ZZZ" {
    //     current_node_id = node_map
    //         .get(&current_node_id)
    //         .unwrap()
    //         .next(instructions.next().unwrap());
    //     steps += 1;
    // }
    // println!("Part 1: {}", steps);

    let mut node_ids: Vec<String> = node_map
        .iter()
        .filter(|&(k, _)| k.ends_with('A'))
        .map(|(_, v)| v.clone().id)
        .collect();
    println!("{:?}", node_ids);

    let mut steps = 0;
    let mut node_loops: Vec<u64> = Vec::new();
    println!("Node IDs: {:?}", node_ids);
    println!("Node loops: {:?}", node_loops);
    while !node_ids.is_empty() {
        steps += 1;
        let instruction = instructions.next().unwrap();
        node_ids = node_ids
            .iter()
            .map(|id| node_map.get(id).unwrap().next(instruction))
            .collect();
        for id in node_ids.clone() {
            if id.ends_with('Z') {
                node_loops.push(steps)
            }
        }
        node_ids.retain(|id| !id.ends_with('Z'));
        println!("Node IDs: {:?}", node_ids);
        println!("Node loops: {:?}", node_loops);
    }
    println!(
        "Part 2: {}",
        node_loops
            .iter()
            .fold(1, |lcm_so_far, &next_number| lcm_so_far.lcm(&next_number))
    );
}

#[derive(Clone)]
struct Node {
    id: String,
    left: String,
    right: String,
}

impl Node {
    fn next(&self, instruction: char) -> String {
        match instruction {
            'L' => self.left.clone(),
            'R' => self.right.clone(),
            _ => panic!("We've done a bad thing"),
        }
    }
}

#[derive(Debug)]
struct ParseNodeError;

impl FromStr for Node {
    type Err = ParseNodeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut strings = s.split_whitespace();
        let id = strings.next().ok_or(ParseNodeError)?.to_string();
        strings.next();
        let left = strings.next().ok_or(ParseNodeError)?[1..4].to_string();
        let right = strings.next().ok_or(ParseNodeError)?[0..3].to_string();

        Ok(Node { id, left, right })
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.left, self.right)
    }
}
