use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


struct Card {
    name: String,
    winning_numbers: Vec<u8>,
    drawn_numbers: Vec<u8>,
}

struct Copies {
    copies: Vec<u32>,
}

fn main() {
    let filename = "../../input";
    let file = File::open(filename).unwrap_or_else(|_| panic!("input file '{filename}' does not exist"));
    let reader = BufReader::new(file);
    let mut number_scratch_cards: u32 = 0;
    let mut copies = Copies::new();
    for ln in reader.lines() {
        let card = Card::parse(&ln.unwrap());
        let n = card.count_winning_numbers();
        for _ in 0..copies.pop_copies() {
            number_scratch_cards += 1;
            copies.add_copies(n);
            //println!("{card} => {n}");
            //println!("        copies: {0:?}", copies.copies);
        }
    }
    println!("number of scratch cards: {number_scratch_cards}");
}

impl Card {
    fn parse(ln: &str) -> Self {
        let (name, numbers) = ln.split_once(": ").expect("missing separator ': '");
        let (winning_numbers, drawn_numbers) = numbers.split_once(" | ").expect("missing separator ' | '");
        Self {
            name: name.to_string(),
            winning_numbers: Self::parse_numbers(winning_numbers),
            drawn_numbers: Self::parse_numbers(drawn_numbers),
        }
    }
    fn parse_numbers(numbers: &str) -> Vec<u8> {
        numbers.trim().replace("  ", " ").split(' ').map(|n| n.parse().expect("failed to parse number")).collect()
    }

    fn count_winning_numbers(&self) -> u8 {
        let mut count: u8 = 0;
        for n in &self.drawn_numbers {
            if self.winning_numbers.contains(n) {
                count += 1;
            }
        }
        count
    }
}

impl Copies {
    fn new() -> Self {
        Self { copies: Vec::new() }
    }

    fn del(&mut self) {
        if !self.copies.is_empty() {
            self.copies.remove(0);
        }
    }

    fn add_copies(&mut self, n: u8) {
        for i in 0..n as usize {
            if self.copies.len() > i {
                self.copies[i] += 1;
            } else {
                self.copies.push(1)
            }
        }
    }

    fn get_copies(&self) -> u32 {
        self.copies.first().unwrap_or(&0) + 1
    }

    fn pop_copies(&mut self) -> u32 {
        let out = self.get_copies();
        self.del();
        out
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {:?} | {:?}", self.name, self.winning_numbers, self.drawn_numbers)
    }
}
