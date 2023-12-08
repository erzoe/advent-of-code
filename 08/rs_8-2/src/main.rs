use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;
use std::fmt;

use regex::Regex;
use once_cell::sync::Lazy;

static RE_NODE_LINE: Lazy::<Regex> = Lazy::new(|| Regex::new(r"^(?<name>[A-Z1-9]+)\s*=\s*\((?<left>[A-Z1-9]+),\s*(?<right>[A-Z1-9]+)\)$").unwrap());


struct Node {
    name: String,
    left: String,
    right: String,
}

struct Nodes {
    nodes: HashMap<String, Node>,
}

struct CurrentNodes<'a> {
    nodes: Vec<&'a Node>,
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    Left,
    Right,
}
struct Directions {
    directions: Vec<Direction>,
    index: usize,
}


fn main() {
    let file = BufReader::new(File::open("../../input").expect("input file does not exist"));
    let mut lines = file.lines().map(|ln| ln.unwrap());
    let mut directions = Directions::parse(&lines.next().unwrap());
    let nodes = Nodes::parse(lines);

    let mut current_nodes = nodes.get_start();
    let mut count: u64 = 0;
    while !current_nodes.is_goal() {
        let direction = directions.next();
        //println!("{current_nodes} {direction:?}");
        current_nodes.step(&nodes, direction);
        count += 1;
    }
    println!("{}", current_nodes);
    println!("number of required steps: {count}");
}


impl Direction {
    fn parse(symbol: char) -> Self {
        match symbol {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("unknown direction '{symbol}'"),
        }
    }
}

impl Directions {
    fn parse(ln: &str) -> Self {
        Self {
            index: 0,
            directions: ln.chars().map(Direction::parse).collect(),
        }
    }

    fn next(&mut self) -> Direction {
        let out = self.directions[self.index];
        self.index += 1;
        if self.index >= self.directions.len() {
            self.index = 0;
        }
        out
    }
}

impl Node {
    fn parse(ln: &str) -> Self {
        let caps = RE_NODE_LINE.captures(ln).expect("Invlid line for node");
        Node {
            name: caps.name("name").unwrap().as_str().to_string(),
            left: caps.name("left").unwrap().as_str().to_string(),
            right: caps.name("right").unwrap().as_str().to_string(),
        }
    }

    fn get_next(&self, direction: Direction) -> String {
        match direction {
            Direction::Left => self.left.to_string(),
            Direction::Right => self.right.to_string(),
        }
    }
}
impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Nodes {
    fn parse(lines: impl Iterator<Item=String>) -> Self {
        Self { nodes: lines.filter(|ln| !ln.is_empty()).map(|ln| Node::parse(&ln)).map(|n| (n.name.to_string(), n)).collect() }
    }

    fn get_start(&self) -> CurrentNodes {
        CurrentNodes { nodes: self.nodes.values().filter(|n| n.name.ends_with('A')).collect() }
    }

    fn get_next(&self, current_node: &Node, direction: Direction) -> &Node {
        &self.nodes[&current_node.get_next(direction)]
    }
}

impl<'a> CurrentNodes<'a> {
    fn is_goal(&self) -> bool {
        self.nodes.iter().all(|n| n.name.ends_with('Z'))
    }

    fn step(&mut self, nodes: &'a Nodes, direction: Direction) {
        for i in 0..self.nodes.len() {
            self.nodes[i] = nodes.get_next(self.nodes[i], direction);
        }
    }
}
impl fmt::Display for CurrentNodes<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.nodes.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(", "))
    }
}
