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

struct HandAndBid {
    hand: Hand,
    bid: u32,
}


fn main() {
    let file = File::open("../../input").expect("input file does not exist");
    let reader = BufReader::new(file);
    let mut hands_and_bids = Vec::new();
    for ln in reader.lines().map(|ln| ln.unwrap()) {
        hands_and_bids.push(HandAndBid::parse(&ln));
    }
    hands_and_bids.sort_by(|a,b| a.hand.cmp(&b.hand));
    let mut result: u32 = 0;
    for (index, item) in hands_and_bids.into_iter().enumerate() {
        let rank = index + 1;
        let bid = item.bid;
        let win = rank as u32 * bid;
        println!("{}: {rank} x {bid} = {win}", item.hand);
        result += win;
    }
    println!("result = {result}");
}


impl HandAndBid {
    fn parse(ln: &str) -> Self {
        let (hand, bid) = ln.split_once(' ').expect("Invalid input line, should contain a space to separate hand and bid");
        Self { hand: Hand::parse(hand), bid: bid.parse().expect("Failed to parse bid") }
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
                        if c != card {
                            if card == joker || c == joker {
                                return Type::FourOfAKind;
                            } else if contains_joker {
                                return Type::FullHouse;
                            } else {
                                return Type::TwoPair;
                            }
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

    macro_rules! assert_type {
        ($input: expr, $expected_type: expr) => {
            let actual_type = Hand::parse($input).get_type();
            assert_eq!(actual_type, $expected_type, "\n input: \"{}\"\n", $input);
        }
    }

    macro_rules! assert_cmp {
        ($hand1: expr, $op: tt, $hand2: expr) => {
            let hand1 = Hand::parse($hand1);
            let hand2 = Hand::parse($hand2);
            assert!(hand1 $op hand2, "FAILED: {hand1} {} {hand2}", stringify!($op));
        }
    }


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

    #[test]
    fn test_type_with_joker () {
        assert_type!("JJJJJ", Type::FiveOfAKind);
        assert_type!("J2222", Type::FiveOfAKind);
        assert_type!("2222J", Type::FiveOfAKind);
        assert_type!("222J2", Type::FiveOfAKind);
        assert_type!("2J2J2", Type::FiveOfAKind);
        assert_type!("J222J", Type::FiveOfAKind);

        assert_type!("J3222", Type::FourOfAKind);
        assert_type!("3J222", Type::FourOfAKind);
        assert_type!("23J22", Type::FourOfAKind);
        assert_type!("2J322", Type::FourOfAKind);
        assert_type!("2J232", Type::FourOfAKind);
        assert_type!("22J23", Type::FourOfAKind);
        assert_type!("222J3", Type::FourOfAKind);
        assert_type!("2223J", Type::FourOfAKind);
        assert_type!("J323J", Type::FourOfAKind);

        assert_type!("J2233", Type::FullHouse);
        assert_type!("2233J", Type::FullHouse);
        assert_type!("22J33", Type::FullHouse);
        assert_type!("2J323", Type::FullHouse);
        assert_type!("J2323", Type::FullHouse);
        assert_type!("2323J", Type::FullHouse);

        assert_type!("2234J", Type::ThreeOfAKind);
        assert_type!("223J4", Type::ThreeOfAKind);
        assert_type!("J2234", Type::ThreeOfAKind);
        assert_type!("J3224", Type::ThreeOfAKind);
        assert_type!("3J224", Type::ThreeOfAKind);
        assert_type!("J234J", Type::ThreeOfAKind);
        assert_type!("2J34J", Type::ThreeOfAKind);
        assert_type!("2J3J4", Type::ThreeOfAKind);

        // there is no TwoPair with a joker

        assert_type!("2345J", Type::OnePair);
        assert_type!("J2345", Type::OnePair);
        assert_type!("23J45", Type::OnePair);

        // there is no HighCard with a joker
    }

    #[test]
    fn test_type_without_joker () {
        assert_type!("99999", Type::FiveOfAKind);

        assert_type!("A9999", Type::FourOfAKind);
        assert_type!("9999A", Type::FourOfAKind);
        assert_type!("99A99", Type::FourOfAKind);

        assert_type!("AA999", Type::FullHouse);
        assert_type!("999AA", Type::FullHouse);
        assert_type!("9AA99", Type::FullHouse);
        assert_type!("9A9A9", Type::FullHouse);

        assert_type!("AK999", Type::ThreeOfAKind);
        assert_type!("999AK", Type::ThreeOfAKind);
        assert_type!("A999K", Type::ThreeOfAKind);
        assert_type!("99A9K", Type::ThreeOfAKind);
        assert_type!("9A9K9", Type::ThreeOfAKind);
        assert_type!("9AK99", Type::ThreeOfAKind);

        assert_type!("9988K", Type::TwoPair);
        assert_type!("9898K", Type::TwoPair);
        assert_type!("9889K", Type::TwoPair);
        assert_type!("K8899", Type::TwoPair);
        assert_type!("K9898", Type::TwoPair);
        assert_type!("K8998", Type::TwoPair);
        assert_type!("88K99", Type::TwoPair);
        assert_type!("98K98", Type::TwoPair);
        assert_type!("89K98", Type::TwoPair);

        assert_type!("234AA", Type::OnePair);
        assert_type!("AA234", Type::OnePair);
        assert_type!("A234A", Type::OnePair);
        assert_type!("2A3A4", Type::OnePair);
        assert_type!("23AA4", Type::OnePair);

        assert_type!("23456", Type::HighCard);
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


    // ------- other cmp Hand tests -------

    #[test]
    fn test_hand_cmp_many () {
        assert_cmp!("22222", >, "J2222");
        assert_cmp!("22222", >, "2J222");
        assert_cmp!("22222", >, "22J22");
        assert_cmp!("22222", >, "222J2");
        assert_cmp!("22222", >, "2222J");

        assert_cmp!("22222", >, "22223");
        assert_cmp!("32222", >, "22223");

        assert_cmp!("JK222", >, "AK222");
        assert_cmp!("K2222", >, "JK222");
    }
}
