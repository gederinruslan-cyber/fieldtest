//! Showdown equity for 2 to 9 players, each holding a range: exact enumeration of every deal
//! and board completion when the work fits the budget, otherwise seeded Monte Carlo.
//!
//! A deal gives each player one live combo, no two sharing a card, and is weighted by the
//! product of the combos' weights; every completion of the board is equally likely. Monte
//! Carlo samples exactly that distribution (rejecting deals that share a card).

use rand_chacha::ChaCha8Rng;
use rand_chacha::rand_core::{Rng, SeedableRng};

use crate::{Card, Error, Range, evaluate};

/// Work an exact call may do, in hand evaluations: board completions × (live combos + deals /
/// 8), since stepping through a deal costs about an eighth of an evaluation. About 0.45 s
/// natively.
const BUDGET: u64 = 15_000_000;

/// Consecutive deals sharing a card after which Monte Carlo gives up with `NoValidDeal`.
const MAX_REJECTIONS: u32 = 10_000;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Method {
    /// Every (deal, board) pair without a shared card was counted.
    Exact {
        deals: u64,
    },
    MonteCarlo {
        samples: u64,
    },
}

/// One player's result, as shares of 1.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Share {
    /// Wins the whole pot.
    pub win: f64,
    /// Splits the pot.
    pub tie: f64,
    /// Expected share of the pot: wins plus the split shares.
    pub equity: f64,
    /// Standard error of `equity`; 0 when exact.
    pub std_error: f64,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Equity {
    pub method: Method,
    pub players: Vec<Share>,
}

/// A combo a player can hold on this board: its cards, as a mask, and its weight.
#[derive(Clone, Copy)]
struct Live {
    cards: [Card; 2],
    mask: u64,
    weight: f64,
}

fn bit(card: Card) -> u64 {
    1 << card.0
}

/// Checks the spot and lists each player's combos that miss the board.
fn spot(ranges: &[Range], board: &[Card]) -> Result<(Vec<Vec<Live>>, u64), Error> {
    if !(2..=9).contains(&ranges.len()) || ranges.iter().any(|r| r.combos().next().is_none()) {
        return Err(Error::InvalidPlayers);
    }
    let board_mask = board.iter().fold(0, |m, &c| m | bit(c));
    if ![0, 3, 4, 5].contains(&board.len()) || board_mask.count_ones() as usize != board.len() {
        return Err(Error::InvalidBoard);
    }
    let mut live: Vec<Vec<Live>> = ranges
        .iter()
        .map(|range| {
            range
                .combos()
                .map(|(combo, weight)| Live {
                    cards: [combo.high(), combo.low()],
                    mask: bit(combo.high()) | bit(combo.low()),
                    weight,
                })
                .filter(|l| l.mask & board_mask == 0)
                .collect()
        })
        .collect();
    // A card in every combo of one player (a single hand) is dead for everyone else.
    let held: Vec<u64> = live.iter().map(|l| held(l)).collect();
    for (player, live) in live.iter_mut().enumerate() {
        let dead = (0..held.len())
            .filter(|&p| p != player)
            .fold(0, |m, p| m | held[p]);
        live.retain(|c| c.mask & dead == 0);
    }
    if live.iter().any(Vec::is_empty) {
        return Err(Error::NoValidDeal);
    }
    Ok((live, board_mask))
}

/// The cards every one of these combos holds.
fn held(live: &[Live]) -> u64 {
    live.iter().fold(!0, |m, c| m & c.mask)
}

/// Running totals per player.
struct Totals {
    win: Vec<f64>,
    tie: Vec<f64>,
    equity: Vec<f64>,
    squares: Vec<f64>,
    weight: f64,
    deals: u64,
}

impl Totals {
    fn new(players: usize) -> Totals {
        let zero = vec![0.0; players];
        Totals {
            win: zero.clone(),
            tie: zero.clone(),
            equity: zero.clone(),
            squares: zero,
            weight: 0.0,
            deals: 0,
        }
    }

    /// One showdown: the best rank wins, equal best ranks split.
    fn add(&mut self, ranks: &[u32], weight: f64) {
        let best = ranks.iter().copied().max().unwrap_or(0);
        let winners = ranks.iter().filter(|&&r| r == best).count();
        let share = 1.0 / winners as f64;
        for (i, _) in ranks.iter().enumerate().filter(|(_, r)| **r == best) {
            if winners == 1 {
                self.win[i] += weight;
            } else {
                self.tie[i] += weight;
            }
            self.equity[i] += weight * share;
            self.squares[i] += weight * share * share;
        }
        self.weight += weight;
        self.deals += 1;
    }

    fn shares(&self, std_error: impl Fn(usize) -> f64) -> Vec<Share> {
        (0..self.win.len())
            .map(|i| Share {
                win: self.win[i] / self.weight,
                tie: self.tie[i] / self.weight,
                equity: self.equity[i] / self.weight,
                std_error: std_error(i),
            })
            .collect()
    }
}

/// Exact equity, or `None` when enumerating every deal and board is over the budget.
pub fn exact(ranges: &[Range], board: &[Card]) -> Result<Option<Equity>, Error> {
    let (live, board_mask) = spot(ranges, board)?;
    // Cards in every live combo of some player (a single hand) can never reach the board.
    let held = live.iter().fold(0, |m, l| m | held(l));
    let rest: Vec<Card> = Card::all()
        .filter(|&c| bit(c) & (board_mask | held) == 0)
        .collect();
    let missing = 5 - board.len();
    let completions = (0..missing as u64).fold(1u64, |n, i| n * (rest.len() as u64 - i) / (i + 1));
    let combos: u64 = live.iter().map(|l| l.len() as u64).sum();
    let deals = live
        .iter()
        .fold(1u64, |n, l| n.saturating_mul(l.len() as u64));
    if completions.saturating_mul(combos.saturating_add(deals / 8)) > BUDGET {
        return Ok(None);
    }

    let mut totals = Totals::new(live.len());
    let mut cards = [Card(0); 7];
    cards[..board.len()].copy_from_slice(board);
    let mut ranks: Vec<Vec<u32>> = live.iter().map(|l| vec![0; l.len()]).collect();
    let mut picked: Vec<usize> = (0..missing).collect();
    loop {
        let mut used = board_mask;
        for (slot, &i) in picked.iter().enumerate() {
            cards[board.len() + slot] = rest[i];
            used |= bit(rest[i]);
        }
        for (ranks, live) in ranks.iter_mut().zip(&live) {
            for (rank, combo) in ranks.iter_mut().zip(live) {
                if combo.mask & used == 0 {
                    cards[5..].copy_from_slice(&combo.cards);
                    *rank = evaluate(&cards).value();
                }
            }
        }
        deal(&live, &ranks, used, 1.0, &mut Vec::new(), &mut totals);
        // The next `missing`-card subset of `rest`, in lexicographic order.
        let Some(at) = (0..missing)
            .rev()
            .find(|&s| picked[s] < rest.len() - missing + s)
        else {
            break;
        };
        picked[at] += 1;
        for s in at + 1..missing {
            picked[s] = picked[s - 1] + 1;
        }
    }
    if totals.deals == 0 {
        return Err(Error::NoValidDeal);
    }
    Ok(Some(Equity {
        method: Method::Exact {
            deals: totals.deals,
        },
        players: totals.shares(|_| 0.0),
    }))
}

/// Every deal on one board: player by player, combos that share no card so far.
fn deal(
    live: &[Vec<Live>],
    ranks: &[Vec<u32>],
    used: u64,
    weight: f64,
    hand: &mut Vec<u32>,
    totals: &mut Totals,
) {
    let player = hand.len();
    if player == live.len() {
        totals.add(hand, weight);
        return;
    }
    for (combo, &rank) in live[player].iter().zip(&ranks[player]) {
        if combo.mask & used == 0 {
            hand.push(rank);
            deal(
                live,
                ranks,
                used | combo.mask,
                weight * combo.weight,
                hand,
                totals,
            );
            hand.pop();
        }
    }
}

/// A uniform float in [0, 1).
fn unit(rng: &mut ChaCha8Rng) -> f64 {
    (rng.next_u64() >> 11) as f64 / (1u64 << 53) as f64
}

/// Monte Carlo equity over `samples` deals (at least 2) drawn with `seed`.
pub fn monte_carlo(
    ranges: &[Range],
    board: &[Card],
    samples: u64,
    seed: u64,
) -> Result<Equity, Error> {
    let (live, board_mask) = spot(ranges, board)?;
    if samples < 2 {
        return Err(Error::InvalidSamples);
    }
    let cumulative: Vec<Vec<f64>> = live
        .iter()
        .map(|l| {
            l.iter()
                .scan(0.0, |sum, c| {
                    *sum += c.weight;
                    Some(*sum)
                })
                .collect()
        })
        .collect();
    let mut rng = ChaCha8Rng::seed_from_u64(seed);
    let mut totals = Totals::new(live.len());
    let mut cards = [Card(0); 7];
    cards[..board.len()].copy_from_slice(board);
    let mut held = Vec::with_capacity(live.len());
    let mut ranks = Vec::with_capacity(live.len());
    for _ in 0..samples {
        let mut rejections = 0;
        let mut used = loop {
            held.clear();
            let mut used = board_mask;
            for (live, cumulative) in live.iter().zip(&cumulative) {
                let pick = unit(&mut rng) * cumulative[cumulative.len() - 1];
                let at = cumulative.partition_point(|&c| c <= pick);
                let combo = live[at.min(live.len() - 1)];
                if combo.mask & used != 0 {
                    break;
                }
                used |= combo.mask;
                held.push(combo.cards);
            }
            if held.len() == live.len() {
                break used;
            }
            rejections += 1;
            if rejections == MAX_REJECTIONS {
                return Err(Error::NoValidDeal);
            }
        };
        for slot in &mut cards[board.len()..5] {
            // Six random bits give 0..64; values off the deck or already dealt are redrawn.
            let card = loop {
                let c = (rng.next_u64() >> 58) as u8;
                if c < 52 && used & 1 << c == 0 {
                    break Card(c);
                }
            };
            used |= bit(card);
            *slot = card;
        }
        ranks.clear();
        for hole in &held {
            cards[5..].copy_from_slice(hole);
            ranks.push(evaluate(&cards).value());
        }
        totals.add(&ranks, 1.0);
    }
    let n = samples as f64;
    let players = totals.shares(|i| {
        let mean = totals.equity[i] / n;
        let variance = (totals.squares[i] - n * mean * mean).max(0.0) / (n - 1.0);
        (variance / n).sqrt()
    });
    Ok(Equity {
        method: Method::MonteCarlo { samples },
        players,
    })
}

/// Exact equity when it fits the budget, otherwise Monte Carlo with `samples` and `seed`.
pub fn equity(ranges: &[Range], board: &[Card], samples: u64, seed: u64) -> Result<Equity, Error> {
    match exact(ranges, board)? {
        Some(equity) => Ok(equity),
        None => monte_carlo(ranges, board, samples, seed),
    }
}
