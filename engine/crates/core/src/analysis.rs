use crate::eval::{bit, straight_high};
use crate::{Card, Category, Combo, Error, Range, evaluate};

/// What a combo has made on the board, strongest first. Classes are exclusive: a combo takes
/// the first that applies.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum HandClass {
    StraightFlush,
    Quads,
    FullHouse,
    Flush,
    Straight,
    /// A pocket pair matching a board card.
    Set,
    /// A hole card matching a board pair.
    Trips,
    /// Both hole ranks on the board.
    TwoPair,
    Overpair,
    TopPair,
    /// A pocket pair below the top board card.
    Underpair,
    MiddlePair,
    /// Paired to the third board rank or lower.
    WeakPair,
    AceHigh,
    Nothing,
}

impl HandClass {
    pub const ALL: [HandClass; 15] = [
        HandClass::StraightFlush,
        HandClass::Quads,
        HandClass::FullHouse,
        HandClass::Flush,
        HandClass::Straight,
        HandClass::Set,
        HandClass::Trips,
        HandClass::TwoPair,
        HandClass::Overpair,
        HandClass::TopPair,
        HandClass::Underpair,
        HandClass::MiddlePair,
        HandClass::WeakPair,
        HandClass::AceHigh,
        HandClass::Nothing,
    ];
}

/// Draws on the flop and turn; a combo can hold several.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Draw {
    /// Four to a flush including a hole card, no flush yet.
    FlushDraw,
    /// Two or more ranks complete a straight the board alone would not.
    OpenEnded,
    /// Exactly one such rank.
    Gutshot,
}

impl Draw {
    pub const ALL: [Draw; 3] = [Draw::FlushDraw, Draw::OpenEnded, Draw::Gutshot];
}

/// A board of 3 to 5 distinct cards written `Ah 7c 2d`.
pub fn parse_board(cards: &str) -> Result<Vec<Card>, Error> {
    let board = cards
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<Card>, _>>()?;
    let mut distinct = board.clone();
    distinct.sort();
    distinct.dedup();
    if !(3..=5).contains(&board.len()) || distinct.len() != board.len() {
        return Err(Error::InvalidBoard);
    }
    Ok(board)
}

fn with_board(combo: Combo, board: &[Card]) -> Vec<Card> {
    let mut cards = board.to_vec();
    cards.extend([combo.high(), combo.low()]);
    cards
}

pub fn classify(combo: Combo, board: &[Card]) -> HandClass {
    let category = evaluate(&with_board(combo, board)).category();
    match category {
        Category::StraightFlush => return HandClass::StraightFlush,
        Category::Quads => return HandClass::Quads,
        Category::FullHouse => return HandClass::FullHouse,
        Category::Flush => return HandClass::Flush,
        Category::Straight => return HandClass::Straight,
        _ => {}
    }
    let on_board = |rank: u8| board.iter().filter(|c| c.rank() == rank).count();
    let mut board_ranks: Vec<u8> = board.iter().map(|c| c.rank()).collect();
    board_ranks.sort_by(|a, b| b.cmp(a));
    board_ranks.dedup();
    let (high, low) = (combo.high().rank(), combo.low().rank());
    if high == low {
        return match () {
            _ if on_board(high) > 0 => HandClass::Set,
            _ if high > board_ranks[0] => HandClass::Overpair,
            _ => HandClass::Underpair,
        };
    }
    if on_board(high) >= 2 || on_board(low) >= 2 {
        return HandClass::Trips;
    }
    if on_board(high) > 0 && on_board(low) > 0 {
        return HandClass::TwoPair;
    }
    let paired = [high, low].into_iter().find(|&r| on_board(r) > 0);
    match paired.and_then(|r| board_ranks.iter().position(|&b| b == r)) {
        Some(0) => HandClass::TopPair,
        Some(1) => HandClass::MiddlePair,
        Some(_) => HandClass::WeakPair,
        None if high == 12 => HandClass::AceHigh,
        None => HandClass::Nothing,
    }
}

pub fn draws(combo: Combo, board: &[Card]) -> Vec<Draw> {
    let mut found = Vec::new();
    if board.len() >= 5 {
        return found;
    }
    let cards = with_board(combo, board);
    let category = evaluate(&cards).category();
    let suited = |suit: u8| cards.iter().filter(|c| c.suit() == suit).count();
    let hole = [combo.high(), combo.low()];
    if category < Category::Flush && hole.iter().any(|c| suited(c.suit()) == 4) {
        found.push(Draw::FlushDraw);
    }
    if category < Category::Straight {
        let mask = |cards: &[Card]| cards.iter().fold(0u16, |m, c| m | bit(c.rank()));
        let (all, board_only) = (mask(&cards), mask(board));
        let completing = (0..13)
            .filter(|&r| {
                straight_high(all | bit(r)).is_some()
                    && straight_high(board_only | bit(r)).is_none()
            })
            .count();
        match completing {
            0 => {}
            1 => found.push(Draw::Gutshot),
            _ => found.push(Draw::OpenEnded),
        }
    }
    found
}

/// Weighted combos of a range on a board, after card removal, by class and draw. Exact: every
/// live combo is enumerated.
#[derive(Clone, PartialEq, Debug)]
pub struct Breakdown {
    live: f64,
    classes: [f64; 15],
    draws: [f64; 3],
}

impl Breakdown {
    /// Total weight of the combos that share no card with the board.
    pub fn live(&self) -> f64 {
        self.live
    }

    pub fn class_weight(&self, class: HandClass) -> f64 {
        self.classes[class as usize]
    }

    pub fn draw_weight(&self, draw: Draw) -> f64 {
        self.draws[draw as usize]
    }

    /// Share of the live weight; 0 for an empty range.
    pub fn class_share(&self, class: HandClass) -> f64 {
        self.share(self.class_weight(class))
    }

    pub fn draw_share(&self, draw: Draw) -> f64 {
        self.share(self.draw_weight(draw))
    }

    fn share(&self, weight: f64) -> f64 {
        if self.live > 0.0 {
            weight / self.live
        } else {
            0.0
        }
    }
}

fn live_combos<'a>(range: &'a Range, board: &'a [Card]) -> impl Iterator<Item = (Combo, f64)> + 'a {
    range
        .combos()
        .filter(|(c, _)| !board.contains(&c.high()) && !board.contains(&c.low()))
}

pub fn analyze(range: &Range, board: &[Card]) -> Breakdown {
    let mut breakdown = Breakdown {
        live: 0.0,
        classes: [0.0; 15],
        draws: [0.0; 3],
    };
    for (combo, weight) in live_combos(range, board) {
        breakdown.live += weight;
        breakdown.classes[classify(combo, board) as usize] += weight;
        for draw in draws(combo, board) {
            breakdown.draws[draw as usize] += weight;
        }
    }
    breakdown
}

/// The live combos whose class is in `classes` or that hold any of `draws`, at their weights.
pub fn filter(range: &Range, board: &[Card], classes: &[HandClass], draws_kept: &[Draw]) -> Range {
    let mut kept = Range::empty();
    for (combo, weight) in live_combos(range, board) {
        if classes.contains(&classify(combo, board))
            || draws(combo, board).iter().any(|d| draws_kept.contains(d))
        {
            kept.weights[combo.index()] = weight;
        }
    }
    kept
}
