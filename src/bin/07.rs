use std::cmp::Ordering;

#[warn(dead_code)]
const L: usize = {
    let mut i = 0;
    let mut lines = 1;
    while i < INPUT.as_bytes().len() {
        if INPUT.as_bytes()[i] == b'\n' {
            lines += 1
        }
        i += 1;
    }
    lines
};
#[aoc::main(07)]
fn main() -> () {}

const CARDS: usize = 13;

const CARD_STRENGTH: [u8; CARDS] = [
    b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'J', b'Q', b'K', b'A',
];

const CARD_STRENGTH_JOKER: [u8; CARDS] = [
    b'J', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b'T', b'Q', b'K', b'A',
];

fn card_array_by_joker(joker: bool) -> [u8; CARDS] {
    if joker {
        CARD_STRENGTH_JOKER
    } else {
        CARD_STRENGTH
    }
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Default)]
enum HandType {
    #[default]
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
struct Hand {
    bid: u32,
    typ: HandType,
    cards: [u8; 5],
}

impl Hand {
    fn init(line: &str, joker_enabled: bool) -> Self {
        let (card_str, bid_str) = (&line[..5], &line[6..]);
        let bid = bid_str.parse::<u32>().unwrap();
        let mut cards: [u8; 5] = [0; 5];
        card_str
            .as_bytes()
            .iter()
            .enumerate()
            .for_each(|(index, &val)| {
                cards[index] = card_array_by_joker(joker_enabled)
                    .iter()
                    .position(|&a| a == val)
                    .unwrap() as u8
            });
        Self {
            bid,
            typ: Self::determine(cards, joker_enabled),
            cards,
        }
    }

    fn determine(cards: [u8; 5], joker: bool) -> HandType {
        let mut ocurrences: [u8; CARDS] = [0; CARDS];
        for card in cards.iter() {
            ocurrences[*card as usize] += 1;
        }
        let mut max: u8 = 0;
        let mut second: u8 = 0;
        let mut iterator = ocurrences.into_iter();
        if joker {
            iterator.next();
        }
        for ocurrence in iterator {
            if ocurrence > max {
                second = max;
                max = ocurrence;
            } else if ocurrence > second {
                second = ocurrence;
            }
        }
        if joker {
            max += ocurrences[0];
        }

        match (max, second) {
            (5, _) => HandType::FiveKind,
            (4, _) => HandType::FourKind,
            (3, 2) => HandType::FullHouse,
            (3, _) => HandType::ThreeKind,
            (2, 2) => HandType::TwoPair,
            (2, 1) => HandType::OnePair,
            (1, _) => HandType::HighCard,
            (_, _) => panic!("Unexpected card arrengement {:?}", cards),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.typ != other.typ {
            return self.typ.cmp(&other.typ);
        }
        for (card1, card2) in self.cards.into_iter().zip(other.cards) {
            match card1.cmp(&card2) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => {}
            }
        }
        Ordering::Equal
    }
}

fn part1(input: &str) -> u32 {
    let mut hands: [Hand; L] = [Hand::default(); L];
    for (index, line) in input.lines().enumerate() {
        hands[index] = Hand::init(line, false);
    }
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (index, val)| acc + val.bid * (index as u32 + 1))
}

fn part2(input: &str) -> u32 {
    let mut hands: [Hand; L] = [Hand::default(); L];
    for (index, line) in input.lines().enumerate() {
        hands[index] = Hand::init(line, true);
    }
    hands.sort();
    hands
        .into_iter()
        .enumerate()
        .fold(0, |acc, (index, val)| acc + val.bid * (index as u32 + 1))
}
