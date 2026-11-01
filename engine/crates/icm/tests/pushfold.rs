#[path = "../../core/tests/common/mod.rs"]
mod common;

use common::fixture;
use fieldtest_core::{Combo, Method, Range, equity, exact};
use fieldtest_icm::{BOARDS, Error, TARGET, class_index, matchups, push_fold};
use proptest::prelude::*;

/// One hand class, the largest an offsuit class can be: 12 of 1,326 combos.
const CLASS: f64 = 12.0 / 1326.0;

/// NH-5: at 2–8 bb the published ranges are not an equilibrium of the stated game.
const HRC: f64 = 0.03;

#[test]
fn matches_hrc_heads_up() {
    for row in fixture("hu-push-fold.csv") {
        let field = |i: usize| row[i].parse::<f64>().unwrap();
        if field(0) < 12.0 {
            continue;
        }
        let stack = 100 * field(0) as u64;
        let ante = if field(1) > 0.0 { 10 } else { 0 };
        let got = push_fold(&[stack, stack], 50, 100, ante, &[1.0]).unwrap();
        assert!(
            (got.push_share - field(2) / 100.0).abs() <= HRC,
            "{row:?}: push {}",
            got.push_share
        );
        assert!(
            (got.call_share - field(3) / 100.0).abs() <= HRC,
            "{row:?}: call {}",
            got.call_share
        );
    }
}

fn class(notation: &str) -> Vec<Combo> {
    Range::parse(notation)
        .unwrap()
        .combos()
        .map(|(c, _)| c)
        .collect()
}

#[test]
fn table_matches_enumeration() {
    let m = matchups();
    for (a, b) in [("AKs", "QQ"), ("T9s", "22")] {
        let (mut wins, mut ties) = (0u64, 0u64);
        for x in class(a) {
            for y in class(b) {
                let single = |c| {
                    let mut r = Range::empty();
                    r.set(c, 1.0).unwrap();
                    r
                };
                let eq = exact(&[single(x), single(y)], &[]).unwrap().unwrap();
                assert_eq!(eq.method, Method::Exact { deals: BOARDS });
                wins += (eq.players[0].win * BOARDS as f64).round() as u64;
                ties += (eq.players[0].tie * BOARDS as f64).round() as u64;
            }
        }
        let i = class_index(class(a)[0]) * 169 + class_index(class(b)[0]);
        assert_eq!(
            (u64::from(m.wins[i]), u64::from(m.ties[i])),
            (wins, ties),
            "{a} v {b}"
        );
    }
}

#[test]
fn table_is_consistent_both_ways() {
    let m = matchups();
    for a in 0..169 {
        for b in 0..169 {
            let (i, j) = (a * 169 + b, b * 169 + a);
            assert_eq!(m.ties[i], m.ties[j]);
            assert_eq!(m.pairs[i], m.pairs[j]);
            let total = u64::from(m.wins[i]) + u64::from(m.ties[i]) + u64::from(m.wins[j]);
            assert_eq!(total, u64::from(m.pairs[i]) * BOARDS, "{a} v {b}");
        }
    }
}

/// Grid label of class `i`, as `Range::grid_weights` orders them.
fn label(i: usize) -> String {
    let ranks = b"AKQJT98765432";
    let (row, col) = (i / 13, i % 13);
    let (high, low) = (ranks[row.min(col)] as char, ranks[row.max(col)] as char);
    match row.cmp(&col) {
        std::cmp::Ordering::Equal => format!("{high}{low}"),
        std::cmp::Ordering::Less => format!("{high}{low}s"),
        std::cmp::Ordering::Greater => format!("{high}{low}o"),
    }
}

/// Independent of the table: the core engine's Monte Carlo equity against the solved push
/// range agrees with every pure call or fold of the big blind at 10 bb, within 4 sigma.
#[test]
fn big_blind_decisions_hold_under_core_equity() {
    let got = push_fold(&[1_000, 1_000], 50, 100, 0, &[1.0]).unwrap();
    let mut pushed = Range::empty();
    for (i, &w) in got.push.iter().enumerate() {
        pushed.set_class(&label(i), w).unwrap();
    }
    for (i, &call) in got.call.iter().enumerate() {
        if call > 0.0 && call < 1.0 {
            continue;
        }
        let hand = class(&label(i))[0];
        let mut held = Range::empty();
        held.set(hand, 1.0).unwrap();
        let eq = equity(&[held, pushed.clone()], &[], 100_000, i as u64)
            .unwrap()
            .players[0];
        // Calling wins a 2,000 pot with `equity`; folding keeps 900.
        let (edge, sigma) = (2_000.0 * eq.equity - 900.0, 2_000.0 * eq.std_error);
        if call == 1.0 {
            assert!(edge > -4.0 * sigma, "{} calls at {edge}", label(i));
        } else {
            assert!(edge < 4.0 * sigma, "{} folds at {edge}", label(i));
        }
    }
}

#[test]
fn icm_pressure_tightens_the_call() {
    let icm = push_fold(&[2_000, 2_000, 300], 50, 100, 0, &[50.0, 30.0, 20.0]).unwrap();
    let chip_ev = push_fold(&[2_000, 2_000], 50, 100, 0, &[1.0]).unwrap();
    assert!(
        icm.call_share < chip_ev.call_share - CLASS,
        "{} v {}",
        icm.call_share,
        chip_ev.call_share
    );
}

#[test]
fn bad_input_is_refused() {
    assert_eq!(
        push_fold(&[1_000, 1_000], 0, 100, 0, &[1.0]),
        Err(Error::InvalidBlinds)
    );
    assert_eq!(
        push_fold(&[1_000, 1_000], 100, 100, 0, &[1.0]),
        Err(Error::InvalidBlinds)
    );
    assert_eq!(
        push_fold(&[1_000, 100], 50, 100, 0, &[1.0]),
        Err(Error::InvalidStacks)
    );
    assert_eq!(
        push_fold(&[60, 1_000], 50, 100, 10, &[1.0]),
        Err(Error::InvalidStacks)
    );
    assert_eq!(
        push_fold(&[1_000, 1_000, 10], 50, 100, 10, &[1.0]),
        Err(Error::InvalidStacks)
    );
    assert_eq!(
        push_fold(&[1_000], 50, 100, 0, &[1.0]),
        Err(Error::InvalidStacks)
    );
    assert_eq!(
        push_fold(&[1_000, 1_000], 50, 100, 0, &[]),
        Err(Error::InvalidPayouts)
    );
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(12))]

    #[test]
    fn nash_within_target(
        stacks in prop::collection::vec(150u64..6_000, 2..=6),
        payouts in prop::collection::vec(0.0f64..100.0, 1..=3),
    ) {
        let mut payouts = payouts;
        payouts.truncate(stacks.len());
        payouts.sort_by(|a, b| b.total_cmp(a));
        prop_assume!(payouts[0] > 0.0);
        let got = push_fold(&stacks, 50, 100, 0, &payouts).unwrap();
        prop_assert!(got.exploitability <= TARGET, "{}", got.exploitability);
        prop_assert!(got.push.iter().chain(&got.call).all(|w| (0.0..=1.0).contains(w)));
    }
}
