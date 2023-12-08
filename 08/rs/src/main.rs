use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use std::collections::HashMap;

use regex::Regex;
use once_cell::sync::Lazy;

static RE_NODE_LINE: Lazy::<Regex> = Lazy::new(|| Regex::new(r"^(?<name>[A-Z]+)\s*=\s*\((?<left>[A-Z]+),\s*(?<right>[A-Z]+)\)$").unwrap());


struct Node {
    name: String,
    left: String,
    right: String,
}

struct Nodes {
    nodes: HashMap<String, Node>,
}

#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}
struct Directions {
    directions: Vec<Direction>,
    index: usize,
}


fn main() {
    let file = BufReader::new(File::open("../../exp").expect("input file does not exist"));
    let mut lines = file.lines().map(|ln| ln.unwrap());
    let mut directions = Directions::parse(&lines.next().unwrap());
    let nodes = Nodes::parse(lines);

    let mut node = nodes.get_start();
    let mut count: u8 = 0;
    while !node.is_goal() {
        node = nodes.get_next(node, directions.next());
        count += 1;
    }
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

    fn is_goal(&self) -> bool {
        self.name == "ZZZ"
    }
}

impl Nodes {
    fn parse(lines: impl Iterator<Item=String>) -> Self {
        Self { nodes: lines.filter(|ln| !ln.is_empty()).map(|ln| Node::parse(&ln)).map(|n| (n.name.to_string(), n)).collect() }
    }

    fn get_start(&self) -> &Node {
        &self.nodes["AAA"]
    }

    fn get_next(&self, current_node: &Node, direction: Direction) -> &Node {
        &self.nodes[&current_node.get_next(direction)]
    }
}
