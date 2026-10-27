use std::fmt;

use crate::Card;

/// Two distinct cards, the higher card first. There are 1,326.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct Combo {
    high: Card,
    low: Card,
}

impl Combo {
    pub fn new(a: Card, b: Card) -> Option<Combo> {
        match a.cmp(&b) {
            std::cmp::Ordering::Greater => Some(Combo { high: a, low: b }),
            std::cmp::Ordering::Less => Some(Combo { high: b, low: a }),
            std::cmp::Ordering::Equal => None,
        }
    }

    pub fn high(self) -> Card {
        self.high
    }

    pub fn low(self) -> Card {
        self.low
    }

    /// Position in 0..1326, ordered by high card and then low card (the order of `all`).
    pub fn index(self) -> usize {
        let high = self.high.0 as usize;
        high * (high - 1) / 2 + self.low.0 as usize
    }

    pub fn from_index(index: usize) -> Option<Combo> {
        let high = (1..52u8).find(|&h| index < h as usize * (h as usize + 1) / 2)?;
        let low = index - high as usize * (high as usize - 1) / 2;
        Some(Combo {
            high: Card(high),
            low: Card(low as u8),
        })
    }

    pub fn all() -> impl Iterator<Item = Combo> {
        (1..52u8).flat_map(|h| {
            (0..h).map(move |l| Combo {
                high: Card(h),
                low: Card(l),
            })
        })
    }
}

impl fmt::Display for Combo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.high, self.low)
    }
}
