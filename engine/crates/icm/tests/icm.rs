#[path = "../../core/tests/common/mod.rs"]
mod common;

use common::fixture;
use fieldtest_icm::{Error, icm};
use proptest::prelude::*;

fn parse<T: std::str::FromStr>(field: &str) -> Vec<T> {
    field.split('/').map(|f| f.parse().ok().unwrap()).collect()
}

#[test]
fn reference_values() {
    for row in fixture("icm.csv") {
        let (stacks, payouts) = (parse::<u64>(&row[1]), parse::<f64>(&row[2]));
        let pool: f64 = payouts.iter().sum();
        for (got, want) in icm(&stacks, &payouts)
            .unwrap()
            .iter()
            .zip(parse::<f64>(&row[4]))
        {
            assert!((got - want).abs() <= 1e-9 * pool, "{row:?}: {got}");
        }
    }
}

#[test]
fn winner_takes_all_is_chip_ev() {
    let got = icm(&[3_200, 5_000, 5_000], &[100.0]).unwrap();
    for (got, stack) in got.iter().zip([3_200.0, 5_000.0, 5_000.0]) {
        assert!((got - 100.0 * stack / 13_200.0).abs() <= 1e-12);
    }
}

#[test]
fn bad_input_is_refused() {
    assert_eq!(icm(&[100], &[1.0]), Err(Error::InvalidStacks));
    assert_eq!(icm(&[100; 21], &[1.0]), Err(Error::InvalidStacks));
    assert_eq!(icm(&[100, 0], &[1.0]), Err(Error::InvalidStacks));
    assert_eq!(icm(&[100, 50], &[]), Err(Error::InvalidPayouts));
    assert_eq!(
        icm(&[1, 2, 3], &[4.0, 3.0, 2.0, 1.0]),
        Err(Error::InvalidPayouts)
    );
    assert_eq!(icm(&[1, 2, 3], &[4.0, -1.0]), Err(Error::InvalidPayouts));
    assert_eq!(icm(&[1, 2, 3], &[f64::NAN]), Err(Error::InvalidPayouts));
}

fn any_spot() -> impl Strategy<Value = (Vec<u64>, Vec<f64>)> {
    prop::collection::vec(1..100_000u64, 2..=12).prop_flat_map(|stacks| {
        let n = stacks.len();
        (Just(stacks), prop::collection::vec(0.0..1_000.0f64, 1..=n))
    })
}

proptest! {
    #![proptest_config(ProptestConfig::with_cases(128))]

    #[test]
    fn equities_sum_to_the_prize_pool((stacks, payouts) in any_spot()) {
        let pool: f64 = payouts.iter().sum();
        let total: f64 = icm(&stacks, &payouts).unwrap().iter().sum();
        prop_assert!((total - pool).abs() <= 1e-9 * pool.max(1.0));
    }

    #[test]
    fn input_order_does_not_matter((stacks, payouts) in any_spot()) {
        let forward = icm(&stacks, &payouts).unwrap();
        let reversed: Vec<u64> = stacks.iter().rev().copied().collect();
        let mut backward = icm(&reversed, &payouts).unwrap();
        backward.reverse();
        for (a, b) in forward.iter().zip(&backward) {
            prop_assert!((a - b).abs() <= 1e-9 * payouts.iter().sum::<f64>().max(1.0));
        }
    }

    #[test]
    fn more_chips_never_mean_less_equity((stacks, mut payouts) in any_spot()) {
        payouts.sort_by(|a, b| b.total_cmp(a));
        let equity = icm(&stacks, &payouts).unwrap();
        let tolerance = 1e-9 * payouts.iter().sum::<f64>().max(1.0);
        for i in 0..stacks.len() {
            for j in 0..stacks.len() {
                if stacks[i] == stacks[j] {
                    prop_assert!((equity[i] - equity[j]).abs() <= tolerance);
                } else if stacks[i] > stacks[j] {
                    prop_assert!(equity[i] >= equity[j] - tolerance);
                }
            }
        }
    }
}
