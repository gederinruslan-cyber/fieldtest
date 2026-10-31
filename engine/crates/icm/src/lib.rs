//! Tournament equity. All poker math lives in the engine (ADR 0002).
//!
//! Malmuth-Harville ICM: each remaining player takes the next place with probability equal to
//! their share of the chips still in play. Exact up to `f64` rounding.

/// Error codes for input the ICM functions refuse.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Error {
    /// Fewer than 2 or more than `MAX_PLAYERS` players, or a stack of 0 chips.
    InvalidStacks,
    /// No payouts, more payouts than players, or a negative or non-finite payout.
    InvalidPayouts,
}

/// Exact ICM keeps one value per set of players, so 2^20 of them at most.
pub const MAX_PLAYERS: usize = 20;

/// Each player's expected prize; `payouts[0]` is first place. The equities sum to the prize
/// pool.
pub fn icm(stacks: &[u64], payouts: &[f64]) -> Result<Vec<f64>, Error> {
    let n = stacks.len();
    if !(2..=MAX_PLAYERS).contains(&n) || stacks.contains(&0) {
        return Err(Error::InvalidStacks);
    }
    if payouts.is_empty() || payouts.len() > n || payouts.iter().any(|p| !p.is_finite() || *p < 0.0)
    {
        return Err(Error::InvalidPayouts);
    }
    let total = stacks
        .iter()
        .try_fold(0u64, |sum, &s| sum.checked_add(s))
        .ok_or(Error::InvalidStacks)?;

    // `reach[placed]`: the probability that the players in `placed` took the first
    // `placed.count_ones()` places, in some order; `chips[placed]`: their chips.
    let mut reach = vec![0.0; 1 << n];
    let mut chips = vec![0u64; 1 << n];
    let mut equity = vec![0.0; n];
    reach[0] = 1.0;
    for placed in 0..1usize << n {
        if placed != 0 {
            let lowest = placed.trailing_zeros() as usize;
            chips[placed] = chips[placed & (placed - 1)] + stacks[lowest];
        }
        let place = placed.count_ones() as usize;
        if place >= payouts.len() {
            continue;
        }
        let left = (total - chips[placed]) as f64;
        for (i, &stack) in stacks.iter().enumerate() {
            if placed & 1 << i == 0 {
                let p = reach[placed] * stack as f64 / left;
                equity[i] += p * payouts[place];
                reach[placed | 1 << i] += p;
            }
        }
    }
    Ok(equity)
}
