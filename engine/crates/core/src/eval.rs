use std::iter::once;

use crate::Card;

/// Hand categories, weakest first. A royal flush is the highest straight flush.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Category {
    HighCard,
    Pair,
    TwoPair,
    Trips,
    Straight,
    Flush,
    FullHouse,
    Quads,
    StraightFlush,
}

const CATEGORIES: [Category; 9] = [
    Category::HighCard,
    Category::Pair,
    Category::TwoPair,
    Category::Trips,
    Category::Straight,
    Category::Flush,
    Category::FullHouse,
    Category::Quads,
    Category::StraightFlush,
];

/// Strength of the best five-card hand: a bigger rank wins, equal ranks tie. The category
/// sits in bits 20..24, then up to five ranks of 4 bits each, most significant first.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct HandRank(u32);

impl HandRank {
    pub fn category(self) -> Category {
        CATEGORIES[(self.0 >> 20) as usize]
    }

    /// The rank as a number that orders hands the same way.
    pub fn value(self) -> u32 {
        self.0
    }
}

fn hand(category: Category, ranks: impl Iterator<Item = u8>) -> HandRank {
    let (value, count) = ranks
        .take(5)
        .fold((category as u32, 0), |(v, n), r| (v << 4 | r as u32, n + 1));
    HandRank(value << (4 * (5 - count)))
}

pub(crate) fn bit(rank: u8) -> u16 {
    1 << rank
}

/// Ranks present in a 13-bit mask, highest first.
fn ranks(mask: u16) -> impl Iterator<Item = u8> {
    (0..13).rev().filter(move |&r| mask & bit(r) != 0)
}

/// The top rank of the highest straight in a 13-bit mask; the ace also plays low (wheel).
pub(crate) fn straight_high(mask: u16) -> Option<u8> {
    let with_low_ace = mask << 1 | mask >> 12;
    (4..14u8)
        .rev()
        .find(|&top| with_low_ace >> (top - 4) & 0b11111 == 0b11111)
        .map(|top| top - 1)
}

/// Ranks the best five-card hand among `cards` (5 to 7 distinct cards; a repeated card counts
/// once). Never panics.
pub fn evaluate(cards: &[Card]) -> HandRank {
    let mut suits = [0u16; 4];
    for card in cards {
        suits[card.suit() as usize] |= bit(card.rank());
    }
    // With at most 7 cards a flush excludes quads and full houses.
    if let Some(&flush) = suits.iter().find(|s| s.count_ones() >= 5) {
        return match straight_high(flush) {
            Some(high) => hand(Category::StraightFlush, once(high)),
            None => hand(Category::Flush, ranks(flush)),
        };
    }
    let [c, d, h, s] = suits;
    let any = c | d | h | s;
    let two = c & d | c & h | c & s | d & h | d & s | h & s;
    let three = c & d & h | c & d & s | c & h & s | d & h & s;
    let four = c & d & h & s;
    let (trips, pairs) = (three & !four, two & !three);

    if let Some(quad) = ranks(four).next() {
        return hand(
            Category::Quads,
            once(quad).chain(ranks(any & !bit(quad)).take(1)),
        );
    }
    if let Some(trip) = ranks(trips).next()
        && let Some(pair) = ranks(trips & !bit(trip) | pairs).next()
    {
        return hand(Category::FullHouse, [trip, pair].into_iter());
    }
    if let Some(high) = straight_high(any) {
        return hand(Category::Straight, once(high));
    }
    if let Some(trip) = ranks(trips).next() {
        return hand(
            Category::Trips,
            once(trip).chain(ranks(any & !bit(trip)).take(2)),
        );
    }
    let mut paired = ranks(pairs);
    match (paired.next(), paired.next()) {
        (Some(p1), Some(p2)) => {
            let kicker = ranks(any & !bit(p1) & !bit(p2)).take(1);
            hand(Category::TwoPair, [p1, p2].into_iter().chain(kicker))
        }
        (Some(p), None) => hand(Category::Pair, once(p).chain(ranks(any & !bit(p)).take(3))),
        _ => hand(Category::HighCard, ranks(any)),
    }
}
