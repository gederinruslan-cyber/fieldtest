//! Push/fold Nash: the small blind moves all in or folds, the big blind calls or folds.

use fieldtest_core::Combo;

/// The class of a combo in grid order (as `Range::grid_weights`): rows and columns run from
/// ace to deuce, pairs on the diagonal, suited above it, offsuit below.
pub fn class_index(combo: Combo) -> usize {
    let (high, low) = (combo.high(), combo.low());
    let (h, l) = (12 - high.rank() as usize, 12 - low.rank() as usize);
    if high.suit() == low.suit() || h == l {
        h * 13 + l
    } else {
        l * 13 + h
    }
}

use std::sync::OnceLock;

use crate::{Error, icm};

/// Win then tie counts per ordered class pair; see `examples/hu_table.rs` and ADR 0005.
static TABLE: &[u8] = include_bytes!("../data/hu-preflop.bin");
/// Boards per pair of combos: C(48, 5).
pub const BOARDS: u64 = 1_712_304;
/// Fictitious play stops once neither player can gain more than this share of the prize pool.
pub const TARGET: f64 = 1e-4;
const MAX_ROUNDS: usize = 20_000;

/// A solved spot. Weights are per class in grid order; shares are of the 1,326 combos.
#[derive(Clone, PartialEq, Debug)]
pub struct PushFold {
    pub push: [f64; 169],
    pub call: [f64; 169],
    pub push_share: f64,
    pub call_share: f64,
    /// The most either player gains by changing any hand's action, as a share of the prize
    /// pool.
    pub exploitability: f64,
}

/// Heads-up all-in data per ordered class pair (small blind's class first).
pub struct Matchups {
    /// Combos in each class (6, 4 or 12).
    pub combos: [u32; 169],
    /// Pairs of combos without a shared card.
    pub pairs: Vec<u32>,
    pub wins: Vec<u32>,
    pub ties: Vec<u32>,
}

pub fn matchups() -> &'static Matchups {
    static DATA: OnceLock<Matchups> = OnceLock::new();
    DATA.get_or_init(|| {
        let word =
            |i: usize| u32::from_le_bytes(TABLE[4 * i..4 * i + 4].try_into().unwrap_or([0; 4]));
        let all: Vec<Combo> = Combo::all().collect();
        let mut combos = [0u32; 169];
        let mut pairs = vec![0u32; 169 * 169];
        for &x in &all {
            combos[class_index(x)] += 1;
            for &y in &all {
                let cards = [x.high(), x.low()];
                if !cards.contains(&y.high()) && !cards.contains(&y.low()) {
                    pairs[class_index(x) * 169 + class_index(y)] += 1;
                }
            }
        }
        Matchups {
            combos,
            pairs,
            wins: (0..169 * 169).map(|i| word(2 * i)).collect(),
            ties: (0..169 * 169).map(|i| word(2 * i + 1)).collect(),
        }
    })
}

/// The ways the hand can end, valued for the small blind and the big blind.
#[derive(Clone, Copy)]
enum Outcome {
    Fold,
    Steal,
    SmallWins,
    Tie,
    BigWins,
}

/// Prize equity of every stack, with a busted player taking the last place still unpaid.
fn values(stacks: &[u64], payouts: &[f64]) -> Result<Vec<f64>, Error> {
    let alive: Vec<u64> = stacks.iter().copied().filter(|&s| s > 0).collect();
    let mut equity = match alive.len() {
        1 => vec![payouts[0]],
        n => icm(&alive, &payouts[..payouts.len().min(n)])?,
    }
    .into_iter();
    let last = payouts.get(alive.len()).copied().unwrap_or(0.0);
    Ok(stacks
        .iter()
        .map(|&s| {
            if s > 0 {
                equity.next().unwrap_or(0.0)
            } else {
                last
            }
        })
        .collect())
}

/// Solves small blind (`stacks[0]`) against big blind (`stacks[1]`); every other player has
/// folded after posting the ante. Payouts run from first place down.
pub fn push_fold(
    stacks: &[u64],
    small_blind: u64,
    big_blind: u64,
    ante: u64,
    payouts: &[f64],
) -> Result<PushFold, Error> {
    if small_blind == 0 || small_blind >= big_blind {
        return Err(Error::InvalidBlinds);
    }
    let posted = |i: usize| ante + [small_blind, big_blind].get(i).copied().unwrap_or(0);
    if stacks.len() < 2 || stacks.iter().enumerate().any(|(i, &s)| s <= posted(i)) {
        return Err(Error::InvalidStacks);
    }
    icm(stacks, payouts)?; // validates the rest of the stacks and the payouts
    let n = stacks.len() as u64;
    let dead = ante * n;
    let (sb, bb) = (stacks[0] - ante, stacks[1] - ante);
    let all_in = sb.min(bb);
    let pot = dead + 2 * all_in;
    let after = |outcome: Outcome| {
        let mut s: Vec<u64> = stacks.iter().map(|&s| s - ante).collect();
        let (small, big) = match outcome {
            Outcome::Fold => (sb - small_blind, bb + small_blind + dead),
            Outcome::Steal => (sb + big_blind + dead, bb - big_blind),
            Outcome::SmallWins => (sb - all_in + pot, bb - all_in),
            Outcome::BigWins => (sb - all_in, bb - all_in + pot),
            // The odd chip goes to the small blind, first to the left of the button.
            Outcome::Tie => (sb - all_in + pot.div_ceil(2), bb - all_in + pot / 2),
        };
        (s[0], s[1]) = (small, big);
        values(&s, payouts).map(|v| (v[0], v[1]))
    };
    let [fold, steal, small_wins, tie, big_wins] = [
        Outcome::Fold,
        Outcome::Steal,
        Outcome::SmallWins,
        Outcome::Tie,
        Outcome::BigWins,
    ]
    .map(after);
    let (fold, steal, small_wins, tie, big_wins) = (fold?, steal?, small_wins?, tie?, big_wins?);

    let m = matchups();
    // gain_small[a][b]: the small blind's gain from a call over a fold by the big blind, weighted
    // by the chance the big blind holds b; gain_big[b][a]: the big blind's gain from calling
    // over folding against a, weighted by the combo pairs.
    let mut gain_small = vec![0.0; 169 * 169];
    let mut gain_big = vec![0.0; 169 * 169];
    for a in 0..169 {
        let row: f64 = (0..169).map(|b| f64::from(m.pairs[a * 169 + b])).sum();
        for b in 0..169 {
            let i = a * 169 + b;
            let pairs = f64::from(m.pairs[i]);
            let deals = pairs * BOARDS as f64;
            let (win, tie_p) = (f64::from(m.wins[i]) / deals, f64::from(m.ties[i]) / deals);
            let lose = 1.0 - win - tie_p;
            let small = win * small_wins.0 + tie_p * tie.0 + lose * big_wins.0;
            let big = win * small_wins.1 + tie_p * tie.1 + lose * big_wins.1;
            gain_small[i] = pairs / row * (small - steal.0);
            gain_big[b * 169 + a] = pairs * (big - steal.1);
        }
    }
    let push_edge = |call: &[f64; 169], a: usize| {
        steal.0 - fold.0
            + (0..169)
                .map(|b| gain_small[a * 169 + b] * call[b])
                .sum::<f64>()
    };
    let call_edge = |push: &[f64; 169], b: usize| {
        (0..169)
            .map(|a| gain_big[b * 169 + a] * push[a])
            .sum::<f64>()
    };

    let pool: f64 = payouts.iter().sum();
    let (mut push, mut call) = ([1.0; 169], [0.0; 169]);
    for round in 1..=MAX_ROUNDS {
        let step = 1.0 / (round as f64 + 1.0);
        // Each side moves a step towards its best response to the other's average.
        let best: [f64; 169] =
            std::array::from_fn(|b| f64::from(u8::from(call_edge(&push, b) > 0.0)));
        for (c, best) in call.iter_mut().zip(best) {
            *c += (best - *c) * step;
        }
        let best: [f64; 169] =
            std::array::from_fn(|a| f64::from(u8::from(push_edge(&call, a) > 0.0)));
        for (p, best) in push.iter_mut().zip(best) {
            *p += (best - *p) * step;
        }
        if round % 25 == 0 && regret(m, &push, &call, push_edge, call_edge) <= TARGET * pool {
            break;
        }
    }
    // Averaging leaves every hand that ever switched slightly mixed. A hand that gains more
    // than the target from one action takes it always, unless that misses the target; ε is
    // measured on the strategies returned.
    let clear = TARGET * pool;
    let push_gain: [f64; 169] = std::array::from_fn(|a| push_edge(&call, a));
    let call_gain: [f64; 169] = std::array::from_fn(|b| {
        let facing: f64 = (0..169)
            .map(|a| f64::from(m.pairs[a * 169 + b]) * push[a])
            .sum();
        if facing > 0.0 {
            call_edge(&push, b) / facing
        } else {
            0.0
        }
    });
    let pure = |w: [f64; 169], gain: [f64; 169]| -> [f64; 169] {
        std::array::from_fn(|i| {
            if gain[i].abs() > clear {
                f64::from(u8::from(gain[i] > 0.0))
            } else {
                w[i]
            }
        })
    };
    let (pure_push, pure_call) = (pure(push, push_gain), pure(call, call_gain));
    let averaged = regret(m, &push, &call, push_edge, call_edge) / pool;
    let purified = regret(m, &pure_push, &pure_call, push_edge, call_edge) / pool;
    let exploitability = if purified <= TARGET.max(averaged) {
        (push, call) = (pure_push, pure_call);
        purified
    } else {
        averaged
    };
    let share =
        |w: &[f64; 169]| (0..169).map(|c| f64::from(m.combos[c]) * w[c]).sum::<f64>() / 1326.0;
    Ok(PushFold {
        push_share: share(&push),
        call_share: share(&call),
        push,
        call,
        exploitability,
    })
}

/// The most either player gains on average by best-responding to the other.
fn regret(
    m: &Matchups,
    push: &[f64; 169],
    call: &[f64; 169],
    push_edge: impl Fn(&[f64; 169], usize) -> f64,
    call_edge: impl Fn(&[f64; 169], usize) -> f64,
) -> f64 {
    let small: f64 = (0..169)
        .map(|a| {
            let edge = push_edge(call, a);
            f64::from(m.combos[a]) / 1326.0 * (edge.max(0.0) - push[a] * edge)
        })
        .sum();
    // call_edge sums combo pairs; dividing by all pairs of b gives the per-hand expectation.
    let big: f64 = (0..169)
        .map(|b| {
            let edge = call_edge(push, b);
            let pairs: f64 = (0..169).map(|a| f64::from(m.pairs[a * 169 + b])).sum();
            f64::from(m.combos[b]) / 1326.0 * (edge.max(0.0) - call[b] * edge) / pairs
        })
        .sum();
    small.max(big)
}
