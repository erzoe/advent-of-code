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
        let joker = Card::parse('J');
        let contains_joker = self.cards.contains(&joker);
        for card in self.cards {
            match self.cards.iter().filter(|&&c| c == card).count() {
                5 => {
                    return Type::FiveOfAKind;
                }
                4 => {
                    if contains_joker {
                        // it does not matter whether the 4 equal cards are jokers and change into the remaining card
                        // or if the remaining card is a joker and changes into the four other cards
                        return Type::FiveOfAKind;
                    } else {
                        return Type::FourOfAKind;
                    }
                }
                3 => {
                    if found_pair.is_some() {
                        if contains_joker {
                            return Type::FiveOfAKind;
                        } else {
                            return Type::FullHouse;
                        }
                    } else {
                        found_three = Some(card);
                    }
                }
                2 => {
                    if found_three.is_some() {
                        if contains_joker {
                            return Type::FiveOfAKind;
                        } else {
                            return Type::FullHouse;
                        }
                    }
                    if let Some(c) = found_pair {
                        if card == joker || c == joker {
                            return Type::FourOfAKind;
                        } else if contains_joker {
                            return Type::FullHouse;
                        } else if c != card {
                            return Type::TwoPair;
                        }
                    }
                    found_pair = Some(card);
                }
                _ => {}
            }
        }

        if found_three.is_some() {
            if contains_joker {
                return Type::FourOfAKind;
            } else {
                return Type::ThreeOfAKind;
            }
        }
        if found_pair.is_some() {
            if contains_joker {
                return Type::ThreeOfAKind;
            } else {
                return Type::OnePair;
            }
        }
        if contains_joker {
            Type::OnePair
        } else {
            Type::HighCard
        }
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
    const SYMBOLS: &str = "J23456789TQKA";

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



mod tests {
    use crate::{Hand, Type};

    // ------- type tests for example -------

    #[test]
    fn test_type_32T3K () {
        assert_eq!(Hand::parse("32T3K").get_type(), Type::OnePair);
    }

    #[test]
    fn test_type_KK677 () {
        assert_eq!(Hand::parse("KK677").get_type(), Type::TwoPair);
    }

    #[test]
    fn test_type_KTJJT () {
        assert_eq!(Hand::parse("KTJJT").get_type(), Type::FourOfAKind);
    }

    #[test]
    fn test_type_T55J5 () {
        assert_eq!(Hand::parse("T55J5").get_type(), Type::FourOfAKind);
    }

    #[test]
    fn test_type_QQQJA () {
        assert_eq!(Hand::parse("QQQJA").get_type(), Type::FourOfAKind);
    }


    // ------- type tests for other types -------

    #[test]
    fn test_type_JQQQJ () {
        assert_eq!(Hand::parse("JQQQJ").get_type(), Type::FiveOfAKind);
    }

    #[test]
    fn test_type_JQQQK () {
        assert_eq!(Hand::parse("JQQQK").get_type(), Type::FourOfAKind);
    }

    #[test]
    fn test_type_QQQQQ () {
        assert_eq!(Hand::parse("QQQQQ").get_type(), Type::FiveOfAKind);
    }

    #[test]
    fn test_type_23456 () {
        assert_eq!(Hand::parse("23456").get_type(), Type::HighCard);
    }

    #[test]
    fn test_type_2345J () {
        assert_eq!(Hand::parse("2345J").get_type(), Type::OnePair);
    }

    #[test]
    fn test_type_2245J () {
        assert_eq!(Hand::parse("2245J").get_type(), Type::ThreeOfAKind);
    }

    #[test]
    fn test_type_2244J () {
        assert_eq!(Hand::parse("2244J").get_type(), Type::FullHouse);
    }

    #[test]
    fn test_type_2444J () {
        assert_eq!(Hand::parse("2444J").get_type(), Type::FourOfAKind);
    }


    // ------- cmp Hand tests for example -------

    #[test]
    fn test_hand_cmp_T55J5_QQQJA () {
        assert!(Hand::parse("T55J5") < Hand::parse("QQQJA"));
    }

    #[test]
    fn test_hand_cmp_KTJJT_KK677 () {
        assert!(Hand::parse("KTJJT") > Hand::parse("KK677"));
    }
}
