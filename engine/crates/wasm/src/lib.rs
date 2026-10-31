//! Browser bindings for the engine: thin wrappers, no math. Every function returns a `Result`
//! whose error is a code (`InvalidToken`, `InvalidWeight`, `InvalidCard`, `InvalidHand`,
//! `InvalidBoard`, `InvalidPlayers`, `NoValidDeal`, `InvalidSamples`, `InvalidStacks`,
//! `InvalidPayouts`);
//! none panics across the boundary.

use fieldtest_core::{
    Card, Draw, HandClass, Method, Range, analyze, equity as showdown, evaluate, filter,
    parse_board,
};
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

/// The names of `analyzeRange`'s entries: the 15 hand classes, then the 3 draws.
#[wasm_bindgen(js_name = analysisLabels)]
pub fn analysis_labels() -> Vec<String> {
    let classes = HandClass::ALL.iter().map(|c| format!("{c:?}"));
    classes
        .chain(Draw::ALL.iter().map(|d| format!("{d:?}")))
        .collect()
}

/// The range on a board after card removal: the live weight, then per hand class (15) and
/// per draw (3) its weight followed by its share of the live weight. Exact.
#[wasm_bindgen(js_name = analyzeRange)]
pub fn analyze_range(range: &str, board: &str) -> Result<Vec<f64>, String> {
    let breakdown = analyze(
        &Range::parse(range).map_err(code)?,
        &parse_board(board).map_err(code)?,
    );
    let classes = HandClass::ALL
        .iter()
        .flat_map(|&c| [breakdown.class_weight(c), breakdown.class_share(c)]);
    let draws = Draw::ALL
        .iter()
        .flat_map(|&d| [breakdown.draw_weight(d), breakdown.draw_share(d)]);
    Ok(std::iter::once(breakdown.live())
        .chain(classes)
        .chain(draws)
        .collect())
}

/// The live combos whose class or draw is named in `keep` (comma separated, as in
/// `analysisLabels`), in canonical notation.
#[wasm_bindgen(js_name = filterRange)]
pub fn filter_range(range: &str, board: &str, keep: &str) -> Result<String, String> {
    let (range, board) = (
        Range::parse(range).map_err(code)?,
        parse_board(board).map_err(code)?,
    );
    let names: Vec<&str> = keep
        .split(',')
        .map(str::trim)
        .filter(|n| !n.is_empty())
        .collect();
    let named = |name: String| names.contains(&name.as_str());
    let classes: Vec<HandClass> = HandClass::ALL
        .into_iter()
        .filter(|c| named(format!("{c:?}")))
        .collect();
    let draws: Vec<Draw> = Draw::ALL
        .into_iter()
        .filter(|d| named(format!("{d:?}")))
        .collect();
    if classes.len() + draws.len() != names.len() {
        return Err("InvalidToken".into());
    }
    Ok(filter(&range, &board, &classes, &draws).to_string())
}

/// Mean weight of each of the 169 grid cells, row by row over ranks A to 2 (pairs on the
/// diagonal, suited above it, offsuit below).
#[wasm_bindgen(js_name = gridWeights)]
pub fn grid_weights(range: &str) -> Result<Vec<f64>, String> {
    Ok(Range::parse(range).map_err(code)?.grid_weights().to_vec())
}

/// The range with every combo of `class` (e.g. `AKs`) set to `weight`, canonical.
#[wasm_bindgen(js_name = setClass)]
pub fn set_class(range: &str, class: &str, weight: f64) -> Result<String, String> {
    let mut range = Range::parse(range).map_err(code)?;
    range.set_class(class, weight).map_err(code)?;
    Ok(range.to_string())
}

/// Showdown equity for 2 to 9 players, their ranges separated by `;`, on a board of 0, 3, 4 or
/// 5 cards: exact when it fits the budget, otherwise Monte Carlo over `samples` deals drawn
/// with `seed`. Gives 1 (exact) or 0 (Monte Carlo), the deals enumerated or samples drawn,
/// then per player win, tie, equity and its standard error (0 when exact).
#[wasm_bindgen]
pub fn equity(players: &str, board: &str, samples: u32, seed: u32) -> Result<Vec<f64>, String> {
    let ranges = players
        .split(';')
        .map(Range::parse)
        .collect::<Result<Vec<_>, _>>()
        .map_err(code)?;
    let board = board
        .split_whitespace()
        .map(str::parse)
        .collect::<Result<Vec<Card>, _>>()
        .map_err(code)?;
    let result = showdown(&ranges, &board, samples.into(), seed.into()).map_err(code)?;
    let head = match result.method {
        Method::Exact { deals } => [1.0, deals as f64],
        Method::MonteCarlo { samples } => [0.0, samples as f64],
    };
    let shares = result
        .players
        .iter()
        .flat_map(|p| [p.win, p.tie, p.equity, p.std_error]);
    Ok(head.into_iter().chain(shares).collect())
}

/// Numbers separated by spaces, commas or slashes.
fn numbers<T: std::str::FromStr>(text: &str) -> Option<Vec<T>> {
    text.split([' ', ',', '/'])
        .filter(|t| !t.is_empty())
        .map(|t| t.parse().ok())
        .collect()
}

/// Malmuth-Harville ICM. Stacks are whole chips, payouts from first place down; gives per
/// player the expected prize, its share of the prize pool and the player's share of the chips.
#[wasm_bindgen]
pub fn icm(stacks: &str, payouts: &str) -> Result<Vec<f64>, String> {
    let stacks: Vec<u64> = numbers(stacks).ok_or("InvalidStacks")?;
    let payouts: Vec<f64> = numbers(payouts).ok_or("InvalidPayouts")?;
    let equity = fieldtest_icm::icm(&stacks, &payouts).map_err(|e| format!("{e:?}"))?;
    let pool: f64 = payouts.iter().sum();
    let chips = stacks.iter().sum::<u64>() as f64;
    Ok(equity
        .iter()
        .zip(&stacks)
        .flat_map(|(&e, &s)| [e, if pool > 0.0 { e / pool } else { 0.0 }, s as f64 / chips])
        .collect())
}
