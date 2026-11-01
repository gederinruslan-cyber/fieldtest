//! Writes `data/hu-preflop.bin`, the exact heads-up all-in table push/fold Nash reads: for each
//! ordered pair of the 169 hand classes, the (deal, board) pairs the first class wins and ties,
//! summed over every pair of combos without a shared card. Little-endian `u32` win then tie,
//! row-major in grid order. Each matchup is enumerated once per suit relabelling.
//!
//! `cargo run --release -p fieldtest-icm --example hu_table`: 93,769 matchups, about 35 minutes
//! on 10 cores. Counting a matchup and its mirror once would halve that.

use std::collections::BTreeMap;
use std::sync::atomic::{AtomicUsize, Ordering};

use fieldtest_core::{Card, Combo, Range, exact};
use fieldtest_icm::class_index;

fn relabel(combo: Combo, perm: &[u8; 4]) -> Combo {
    let card = |c: Card| Card::new(c.rank(), perm[c.suit() as usize]).expect("valid card");
    Combo::new(card(combo.high()), card(combo.low())).expect("distinct cards")
}

fn perms() -> Vec<[u8; 4]> {
    (0..256u32)
        .map(|n| std::array::from_fn(|i| (n >> (2 * i) & 3) as u8))
        .filter(|p: &[u8; 4]| (0..4).all(|s| p.contains(&s)))
        .collect()
}

fn main() {
    let perms = perms();
    let combos: Vec<Combo> = Combo::all().collect();
    let shares = |x: Combo, y: Combo| {
        x.high() != y.high() && x.high() != y.low() && x.low() != y.high() && x.low() != y.low()
    };
    // Canonical matchup → the ordered combo pairs it stands for.
    let mut groups: BTreeMap<(usize, usize), Vec<(Combo, Combo)>> = BTreeMap::new();
    for &x in &combos {
        for &y in &combos {
            if shares(x, y) {
                let key = perms
                    .iter()
                    .map(|p| (relabel(x, p).index(), relabel(y, p).index()))
                    .min()
                    .expect("24 relabellings");
                groups.entry(key).or_default().push((x, y));
            }
        }
    }
    let keys: Vec<(usize, usize)> = groups.keys().copied().collect();
    eprintln!("{} canonical matchups", keys.len());
    let next = AtomicUsize::new(0);
    let threads = std::thread::available_parallelism().map_or(4, |n| n.get());
    let results: Vec<Vec<(usize, u64, u64)>> = std::thread::scope(|s| {
        let workers: Vec<_> = (0..threads)
            .map(|_| {
                s.spawn(|| {
                    let mut out = Vec::new();
                    loop {
                        let i = next.fetch_add(1, Ordering::Relaxed);
                        let Some(&(x, y)) = keys.get(i) else { break };
                        let range = |index| {
                            let mut r = Range::empty();
                            r.set(Combo::from_index(index).expect("index"), 1.0)
                                .expect("weight");
                            r
                        };
                        let eq = exact(&[range(x), range(y)], &[])
                            .expect("valid")
                            .expect("fits");
                        let fieldtest_core::Method::Exact { deals } = eq.method else {
                            unreachable!()
                        };
                        let count = |share: f64| (share * deals as f64).round() as u64;
                        out.push((i, count(eq.players[0].win), count(eq.players[0].tie)));
                        if i.is_multiple_of(2000) {
                            eprintln!("{i}/{}", keys.len());
                        }
                    }
                    out
                })
            })
            .collect();
        workers
            .into_iter()
            .map(|w| w.join().expect("worker"))
            .collect()
    });
    let mut table = vec![(0u64, 0u64); 169 * 169];
    for (i, win, tie) in results.into_iter().flatten() {
        for &(x, y) in &groups[&keys[i]] {
            let cell = &mut table[class_index(x) * 169 + class_index(y)];
            cell.0 += win;
            cell.1 += tie;
        }
    }
    let bytes: Vec<u8> = table
        .iter()
        .flat_map(|&(w, t)| [w, t])
        .flat_map(|v| u32::try_from(v).expect("fits u32").to_le_bytes())
        .collect();
    std::fs::write(
        concat!(env!("CARGO_MANIFEST_DIR"), "/data/hu-preflop.bin"),
        bytes,
    )
    .expect("write table");
}
