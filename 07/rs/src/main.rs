use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::cmp::Ordering;
use std::fmt;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct Card {
    symbol: char,
    index: usize,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Debug)]
enum Type {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(PartialEq, Eq)]
struct Hand {
    cards: [Card; 5],
}

struct HandAndBit {
    hand: Hand,
    bit: u32,
}


fn main() {
    let file = File::open("../../exp").expect("input file does not exist");
    let reader = BufReader::new(file);
    let mut hands_and_bits = Vec::new();
    for ln in reader.lines().map(|ln| ln.unwrap()) {
        hands_and_bits.push(HandAndBit::parse(&ln));
    }
    hands_and_bits.sort_by(|a,b| a.hand.cmp(&b.hand));
    let mut result: u32 = 0;
    for (index, item) in hands_and_bits.into_iter().enumerate() {
        let rank = index + 1;
        let bit = item.bit;
        let win = rank as u32 * bit;
        println!("{}: {rank} x {bit} = {win}", item.hand);
        result += win;
    }
    println!("result = {result}");
}


impl HandAndBit {
    fn parse(ln: &str) -> Self {
        let (hand, bit) = ln.split_once(' ').expect("Invalid input line, should contain a space to separate hand and bit");
        Self { hand: Hand::parse(hand), bit: bit.parse().expect("Failed to parse bit") }
    }
}

impl Hand {
    fn parse(cards: &str) -> Self {
        if cards.len() != 5 {
            panic!("Invalid hand '{cards}', should be 5 cards exactly");
        }
        let mut cards = cards.chars();
        Self {
            cards: [
                Card::parse(cards.next().unwrap()),
                Card::parse(cards.next().unwrap()),
                Card::parse(cards.next().unwrap()),
                Card::parse(cards.next().unwrap()),
                Card::parse(cards.next().unwrap()),
            ],
        }
    }

    fn get_type(&self) -> Type {
        let mut found_pair: Option<Card> = None;
        let mut found_three: Option<Card> = None;
        for card in self.cards {
            match self.cards.iter().filter(|&&c| c == card).count() {
                5 => {
                    return Type::FiveOfAKind;
                }
                4 => {
                    return Type::FourOfAKind;
                }
                3 => {
                    if found_pair.is_some() {
                        return Type::FullHouse;
                    } else {
                        found_three = Some(card);
                    }
                }
                2 => {
                    if found_three.is_some() {
                        return Type::FullHouse;
                    }
                    if let Some(c) = found_pair {
                        if c != card {
                            return Type::TwoPair;
                        }
                    }
                    found_pair = Some(card);
                }
                _ => {}
            }
        }

        if found_three.is_some() {
            return Type::ThreeOfAKind;
        }
        if found_pair.is_some() {
            return Type::OnePair;
        }
        return Type::HighCard;
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let out = self.get_type().cmp(&other.get_type());
        if out != Ordering::Equal {
            return out;
        }
        self.cards.cmp(&other.cards)
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl fmt::Display for Hand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.cards.iter().map(|c| c.symbol.to_string()).collect::<Vec<String>>().join(""))
    }
}

impl Card {
    const SYMBOLS: &str = "AKQJT98765432";

    fn parse(symbol: char) -> Self {
        if let Some(index) = Self::SYMBOLS.find(symbol) {
            Self { symbol, index }
        } else {
            panic!("unknown card '{symbol}'");
        }
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index.cmp(&other.index)
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}
