use std::fmt;
use std::str::FromStr;

use crate::Error;

pub(crate) const RANKS: &[u8; 13] = b"23456789TJQKA";
const SUITS: &[u8; 4] = b"cdhs";

/// A playing card, stored as `rank * 4 + suit`: ranks 0 (deuce) to 12 (ace), suits clubs,
/// diamonds, hearts, spades. Written `Ah`, `Tc`.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Card(pub(crate) u8);

impl Card {
    pub fn new(rank: u8, suit: u8) -> Option<Card> {
        (rank < 13 && suit < 4).then_some(Card(rank * 4 + suit))
    }

    pub fn rank(self) -> u8 {
        self.0 / 4
    }

    pub fn suit(self) -> u8 {
        self.0 % 4
    }

    /// The 52 cards, deuce of clubs first.
    pub fn all() -> impl Iterator<Item = Card> {
        (0..52).map(Card)
    }
}

pub(crate) fn parse_rank(c: u8) -> Option<u8> {
    RANKS.iter().position(|&r| r == c).map(|r| r as u8)
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (rank, suit) = (RANKS[self.rank() as usize], SUITS[self.suit() as usize]);
        write!(f, "{}{}", rank as char, suit as char)
    }
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Card, Error> {
        let &[rank, suit] = s.as_bytes() else {
            return Err(Error::InvalidCard);
        };
        let suit = SUITS
            .iter()
            .position(|&c| c == suit)
            .ok_or(Error::InvalidCard)?;
        Card::new(parse_rank(rank).ok_or(Error::InvalidCard)?, suit as u8).ok_or(Error::InvalidCard)
    }
}
