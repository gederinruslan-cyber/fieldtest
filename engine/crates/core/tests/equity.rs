mod common;

use common::fixture;
use fieldtest_core::{Card, Equity, Error, Method, Range, equity, exact, monte_carlo};
use proptest::prelude::*;

fn ranges(players: &[&str]) -> Vec<Range> {
    players.iter().map(|p| Range::parse(p).unwrap()).collect()
}

fn cards(s: &str) -> Vec<Card> {
    s.split_whitespace().map(|c| c.parse().unwrap()).collect()
}

fn shares(e: &Equity) -> Vec<f64> {
    e.players.iter().map(|p| p.equity).collect()
}

#[test]
fn heads_up_reference_equities() {
    for row in fixture("hu-equities.csv") {
        let e = exact(&ranges(&[&row[0], &row[1]]), &[]).unwrap().unwrap();
        assert_eq!(e.method, Method::Exact { deals: 1_712_304 }, "{row:?}");
        let got = [e.players[0].win, e.players[1].win, e.players[0].tie];
        for (got, want) in got.iter().zip(&row[2..5]) {
            let want: f64 = want.parse().unwrap();
            assert!((got * 100.0 - want).abs() <= 0.005, "{row:?}: {got}");
        }
    }
}

#[test]
fn card_removal_leaves_the_one_ace_pair() {
    let e = exact(&ranges(&["AhAs", "AA"]), &[]).unwrap().unwrap();
    assert_eq!(shares(&e), vec![0.5, 0.5]);
}

#[test]
fn monte_carlo_is_seeded_and_within_its_stated_error() {
    let players = ranges(&["AhAs", "KdKc"]);
    let mc = monte_carlo(&players, &[], 200_000, 7).unwrap();
    assert_eq!(mc, monte_carlo(&players, &[], 200_000, 7).unwrap());
    assert_eq!(mc.method, Method::MonteCarlo { samples: 200_000 });
    let p = &mc.players[0];
    assert!((p.std_error - 0.00087).abs() < 0.0001, "{}", p.std_error);
    assert!(
        (p.equity - 0.8125).abs() <= 4.0 * p.std_error,
        "{}",
        p.equity
    );
}

#[test]
fn multiway_hands_enumerate_and_agree_with_sampling() {
    let nine = ranges(&[
        "AhAs", "KhKs", "QhQs", "JhJs", "ThTs", "9h9s", "8h8s", "7h7s", "6h6s",
    ]);
    let e = equity(&nine, &[], 1, 1).unwrap();
    assert_eq!(e.method, Method::Exact { deals: 278_256 });
    let three = ranges(&["AhAs", "KdKc", "QhQs"]);
    let (e, mc) = (
        exact(&three, &[]).unwrap().unwrap(),
        monte_carlo(&three, &[], 100_000, 3).unwrap(),
    );
    for (e, mc) in e.players.iter().zip(&mc.players) {
        assert!((e.equity - mc.equity).abs() <= 4.0 * mc.std_error);
    }
}

#[test]
fn wide_preflop_ranges_fall_back_to_sampling() {
    let players = ranges(&["AA", "22+,A2+"]);
    assert_eq!(exact(&players, &[]), Ok(None));
    let e = equity(&players, &[], 1_000, 1).unwrap();
    assert_eq!(e.method, Method::MonteCarlo { samples: 1_000 });
}

#[test]
fn bad_spots_are_refused() {
    let none = |players: &[&str], board: &str| equity(&ranges(players), &cards(board), 10, 1);
    assert_eq!(none(&["AA"], ""), Err(Error::InvalidPlayers));
    assert_eq!(none(&["AA"; 10], ""), Err(Error::InvalidPlayers));
    assert_eq!(none(&["AA", ""], ""), Err(Error::InvalidPlayers));
    assert_eq!(none(&["AhAs", "AhKd"], ""), Err(Error::NoValidDeal));
    assert_eq!(none(&["AA", "KK"], "Ah 7c"), Err(Error::InvalidBoard));
    assert_eq!(none(&["AA", "KK"], "Ah Ah 7c"), Err(Error::InvalidBoard));
}

fn any_spot() -> impl Strategy<Value = (Vec<Range>, Vec<Card>)> {
    let deck: Vec<Card> = Card::all().collect();
    let range = prop::collection::vec((0..1326usize, 0.01..=1.0f64), 1..6).prop_map(|combos| {
        let mut range = Range::empty();
        for (i, w) in combos {
            let combo = fieldtest_core::Combo::from_index(i).unwrap();
            range.set(combo, w).unwrap();
        }
        range
    });
    (
        prop::collection::vec(range, 2..=4),
        prop_oneof![Just(3usize), Just(4), Just(5)]
            .prop_flat_map(move |n| prop::sample::subsequence(deck.clone(), n)),
    )
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(64))]

    #[test]
    fn shares_sum_to_one((players, board) in any_spot(), seed in 0..1000u64) {
        for e in [exact(&players, &board), monte_carlo(&players, &board, 500, seed).map(Some)] {
            let Ok(Some(e)) = e else { continue };
            prop_assert!((shares(&e).iter().sum::<f64>() - 1.0).abs() <= 1e-9);
            for p in &e.players {
                prop_assert!(p.win + p.tie <= 1.0 + 1e-9 && p.equity >= p.win);
            }
        }
    }

    #[test]
    fn the_same_range_twice_splits_evenly((players, board) in any_spot()) {
        if let Ok(Some(e)) = exact(&[players[0].clone(), players[0].clone()], &board) {
            prop_assert!((e.players[0].equity - 0.5).abs() <= 1e-9);
        }
    }
}
