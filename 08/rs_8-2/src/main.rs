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
    is_start: bool,
    is_end: bool,
}

struct Nodes<'a> {
    nodes: Vec<Node>,
    left: HashMap<&'a Node, &'a Node>,
    right: HashMap<&'a Node, &'a Node>,
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
    len: usize,
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
        let directions: Vec<Direction> = ln.chars().map(Direction::parse).collect();
        let len = directions.len();
        Self {
            directions,
            len,
            index: 0,
        }
    }

    fn next(&mut self) -> Direction {
        let out = self.directions[self.index];
        self.index += 1;
        if self.index >= self.len {
            self.index = 0;
        }
        out
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

impl Nodes<'_> {
    fn parse(lines: impl Iterator<Item=String>) -> Self {
        let lines: Vec<String> = lines.filter(|ln| !ln.is_empty()).collect();
        let mut nodes = Vec::<Node>::new();
        let mut left = HashMap::<&Node, &Node>::new();
        let mut right = HashMap::<&Node, &Node>::new();
        for ln in lines {
            let caps = RE_NODE_LINE.captures(&ln).expect("Invlid line for node");
            let name = caps.name("name").unwrap().as_str().to_string();
            nodes.push(Node { name, is_start: name.ends_with('A'), is_end: name.ends_with('Z') });
        }
        for ln in lines {
            let caps = RE_NODE_LINE.captures(&ln).expect("Invlid line for node");
            let name = caps.name("name").unwrap().as_str().to_string();
            let name_left = caps.name("left").unwrap().as_str().to_string();
            let name_right = caps.name("right").unwrap().as_str().to_string();

            let this =  nodes.iter().find(|n| n.name == name).unwrap();
            let node_left =  nodes.iter().find(|n| n.name == name_left).unwrap();
            let node_right = nodes.iter().find(|n| n.name == name_right).unwrap();

            left[&this] = &node_left;
            right[&this] = &node_right;
        }
        Self { nodes, left, right }
    }

    fn get_start(&self) -> CurrentNodes {
        CurrentNodes { nodes: self.nodes.iter().filter(|n| n.is_start).collect() }
    }

    fn get_next(&self, current_node: &Node, direction: Direction) -> &Node {
        match direction {
            Direction::Left => self.left[&current_node],
            Direction::Right => self.right[&current_node],
        }
    }
}

impl<'a> CurrentNodes<'a> {
    fn is_goal(&self) -> bool {
        self.nodes.iter().all(|n| n.is_end)
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
