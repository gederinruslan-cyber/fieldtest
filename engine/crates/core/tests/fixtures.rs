//! The golden fixtures are internally consistent: guards against transcription errors. The
//! values themselves are human-approved (NH-2) and never edited to make a test pass.

mod common;

use common::fixture;
use fieldtest_core::{Card, Combo};

fn number(field: &str) -> f64 {
    field
        .parse()
        .unwrap_or_else(|_| panic!("not a number: {field}"))
}

fn numbers(field: &str) -> Vec<f64> {
    field.split('/').map(number).collect()
}

#[test]
fn seven_card_counts_cover_every_hand() {
    let total: u64 = fixture("seven-card-categories.csv")
        .iter()
        .map(|r| r[1].parse::<u64>().unwrap())
        .sum();
    assert_eq!(total, 133_784_560);
}

#[test]
fn heads_up_equities_sum_to_one_hundred() {
    let rows = fixture("hu-equities.csv");
    assert_eq!(rows.len(), 26);
    for row in rows {
        for hand in &row[..2] {
            let (a, b) = hand.split_at(2);
            assert!(
                Combo::new(a.parse::<Card>().unwrap(), b.parse().unwrap()).is_some(),
                "{hand}"
            );
        }
        let sum: f64 = row[2..5].iter().map(|f| number(f)).sum();
        assert!((sum - 100.0).abs() <= 0.02, "{row:?} sums to {sum}");
    }
}

#[test]
fn icm_rows_sum_to_the_prize_pool() {
    for row in fixture("icm.csv") {
        let (payouts, exact) = (numbers(&row[2]), numbers(&row[4]));
        let pool: f64 = payouts.iter().sum();
        assert!((exact.iter().sum::<f64>() - pool).abs() <= 1e-6, "{row:?}");
        if row[3] != "-" {
            for (published, exact) in numbers(&row[3]).iter().zip(&exact) {
                // The source prints whole-number approximations (22.57 shows as 22).
                assert!((published - exact).abs() < 1.0, "{row:?}");
            }
        }
    }
}

#[test]
fn push_fold_rows_cover_two_to_twenty_big_blinds() {
    let rows = fixture("hu-push-fold.csv");
    let mut spots: Vec<(String, String)> =
        rows.iter().map(|r| (r[0].clone(), r[1].clone())).collect();
    spots.sort();
    spots.dedup();
    assert_eq!(spots.len(), 16);
    for row in rows {
        for pct in &row[2..4] {
            assert!((0.0..=100.0).contains(&number(pct)), "{row:?}");
        }
    }
}
