use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

struct MapEntry {
    dst: u64,
    src: u64,
    len: u64,
}

struct Map {
    entries: Vec<MapEntry>,
}

fn main() {
    let file = File::open("../../input").unwrap_or_else(|_| panic!("file not found"));
    let reader = BufReader::new(file);
    let mut iterator = reader.lines().map(|ln| ln.unwrap());
    let start_seeds = parse_start_seeds(&iterator.next().expect("file is empty"));
    println!("start_seeds: {start_seeds:?}");

    let mut maps = Vec::<Map>::new();

    for ln in iterator {
        if ln.is_empty() {
            // ignore empty lines
        } else if ln.ends_with(':') {
            // start new map
            maps.push(Map::new());
        } else {
            maps.last_mut().expect("map entry before first map start").entries.push(MapEntry::parse(&ln));
        }
    }

    let mut min_location = u64::MAX;
    for seed in start_seeds {
        let mut result = seed;
        for map in &maps {
            result = map.lookup(result);
        }
        if result < min_location {
            min_location = result;
        }
    }
    println!("min_location: {min_location}");
}

fn parse_start_seeds(ln: &str) -> Vec<u64> {
    let (_, seeds) = ln.split_once(": ").expect("failed to split first line");
    let mut seeds_iterator = seeds.split(' ');
    let mut out = Vec::new();
    while let Some(seed_str) = seeds_iterator.next() {
        let start = seed_str.parse::<u64>().expect("failed to parse start seed");
        let len = seeds_iterator.next().expect("missing length in start seeds range").parse::<u64>().expect("failed to parse start seed length");
        for i in start..start+len {
            out.push(i);
        }
    }
    out
}

impl MapEntry {
    fn parse(ln: &str) -> Self {
        let mut s = ln.split(' ');
        let dst = s.next().expect("missing destination range start").parse::<u64>().expect("failed to parse destination range start");
        let src = s.next().expect("missing source range start").parse::<u64>().expect("failed to parse source range start");
        let len = s.next().expect("missing range length").parse::<u64>().expect("failed to parse range length");
        if let Some(_value) = s.next() {
            panic!("unexpected value after range length in line {ln}");
        }
        Self {dst, src, len}
    }
}

impl Map {
    fn new() -> Self {
        Self { entries: Vec::new() }
    }
    fn lookup(&self, val: u64) -> u64 {
        for entry in &self.entries {
            if entry.src <= val && val < entry.src + entry.len {
                return val - entry.src + entry.dst;
            }
        }
        val
    }
}
