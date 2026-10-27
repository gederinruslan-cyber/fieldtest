//! Browser bindings for the engine: thin wrappers, no math. Every function returns a `Result`
//! whose error is a code (`InvalidToken`, `InvalidWeight`, `InvalidCard`, `InvalidHand`);
//! none panics across the boundary.

use fieldtest_core::{Card, Range, evaluate};
use wasm_bindgen::prelude::*;

fn code(error: fieldtest_core::Error) -> String {
    format!("{error:?}")
}

/// 5 to 7 distinct cards written `As Ks Qs Js Ts`.
fn hand(cards: &str) -> Result<Vec<Card>, String> {
    let hand = cards
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<Card>, _>>()
        .map_err(code)?;
    let mut distinct = hand.clone();
    distinct.sort();
    distinct.dedup();
    if !(5..=7).contains(&hand.len()) || distinct.len() != hand.len() {
        return Err("InvalidHand".into());
    }
    Ok(hand)
}

/// The range in canonical notation.
#[wasm_bindgen(js_name = normalizeRange)]
pub fn normalize_range(notation: &str) -> Result<String, String> {
    Range::parse(notation)
        .map(|range| range.to_string())
        .map_err(code)
}

/// Combos in the range, each counted at its weight.
#[wasm_bindgen(js_name = rangeSize)]
pub fn range_size(notation: &str) -> Result<f64, String> {
    // Summed from +0.0: `Iterator::sum` over no floats gives -0.0.
    let range = Range::parse(notation).map_err(code)?;
    Ok(range
        .combos()
        .fold(0.0, |total, (_, weight)| total + weight))
}

/// Strength of the best five-card hand; a bigger number wins, equal numbers tie.
#[wasm_bindgen(js_name = evaluateHand)]
pub fn evaluate_hand(cards: &str) -> Result<u32, String> {
    Ok(evaluate(&hand(cards)?).value())
}

/// The category of the best five-card hand, e.g. `StraightFlush`.
#[wasm_bindgen(js_name = handCategory)]
pub fn hand_category(cards: &str) -> Result<String, String> {
    Ok(format!("{:?}", evaluate(&hand(cards)?).category()))
}
