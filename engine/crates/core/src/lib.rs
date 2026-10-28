//! Cards, combos, ranges and hand evaluation. All poker math lives in the engine (ADR 0002).

mod analysis;
mod card;
mod combo;
mod eval;
mod range;

pub use analysis::{Breakdown, Draw, HandClass, analyze, classify, draws, filter, parse_board};
pub use card::Card;
pub use combo::Combo;
pub use eval::{Category, HandRank, evaluate};
pub use range::Range;

/// Error codes for input the engine refuses.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
    InvalidCard,
    InvalidToken,
    InvalidWeight,
    InvalidBoard,
}
