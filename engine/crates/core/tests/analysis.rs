use fieldtest_core::{Card, Draw, Error, HandClass, Range, analyze, filter, parse_board};
use proptest::prelude::*;

fn board(s: &str) -> Vec<Card> {
    parse_board(s).unwrap()
}

/// Live weight, then classes and draws with a positive weight, in `ALL` order.
type Spot = (f64, Vec<(HandClass, f64)>, Vec<(Draw, f64)>);

fn breakdown(range: &str, on: &str) -> Spot {
    let b = analyze(&Range::parse(range).unwrap(), &board(on));
    let classes = HandClass::ALL
        .into_iter()
        .map(|c| (c, b.class_weight(c)))
        .filter(|p| p.1 > 0.0);
    let draws = Draw::ALL
        .into_iter()
        .map(|d| (d, b.draw_weight(d)))
        .filter(|p| p.1 > 0.0);
    (b.live(), classes.collect(), draws.collect())
}

#[test]
fn reference_spots() {
    use HandClass::*;
    assert_eq!(breakdown("AA", "Ah 7c 2d"), (3.0, vec![(Set, 3.0)], vec![]));
    assert_eq!(
        breakdown("AK", "Kh 7c 2d"),
        (12.0, vec![(TopPair, 12.0)], vec![])
    );
    assert_eq!(
        breakdown("AhQh", "Kh 7h 2c"),
        (1.0, vec![(AceHigh, 1.0)], vec![(Draw::FlushDraw, 1.0)])
    );
    assert_eq!(
        breakdown("98s", "Tc 7d 2s"),
        (4.0, vec![(Nothing, 4.0)], vec![(Draw::OpenEnded, 4.0)])
    );
    assert_eq!(
        breakdown("J9o", "Qc 8d 2s"),
        (12.0, vec![(Nothing, 12.0)], vec![(Draw::Gutshot, 12.0)])
    );
    for (combo, class) in [
        ("Ks2s", TopPair),
        ("9s2h", MiddlePair),
        ("4s2h", WeakPair),
        ("QQ", Underpair),
        ("AA", Overpair),
        ("AsQs", AceHigh),
        ("K9", TwoPair),
        ("99", Set),
    ] {
        let (_, classes, _) = breakdown(combo, "Kh 9c 4d");
        assert_eq!(
            classes.iter().map(|p| p.0).collect::<Vec<_>>(),
            vec![class],
            "{combo}"
        );
    }
    assert_eq!(breakdown("9s8s", "9c 9d 4h").1, vec![(Trips, 1.0)]);
}

#[test]
fn draws_need_a_hole_card_and_a_card_to_come() {
    assert_eq!(
        breakdown("AhAd", "9c 8d 7s 6h"),
        (1.0, vec![(HandClass::Overpair, 1.0)], vec![])
    );
    assert!(breakdown("AhQh", "Kh 7h 2c 3s 9d").2.is_empty());
}

#[test]
fn boards_are_three_to_five_distinct_cards() {
    for bad in ["Ah 7c", "Ah 7c 2d 3d 4d 5d", "Ah Ah 2d", "Ah 7c 1d"] {
        assert!(
            matches!(
                parse_board(bad),
                Err(Error::InvalidBoard) | Err(Error::InvalidCard)
            ),
            "{bad}"
        );
    }
}

#[test]
fn grid_weights_and_class_setting() {
    let grid = Range::parse("AKs,QQ:0.5").unwrap().grid_weights();
    let (aks, qq) = (1, 2 * 13 + 2); // row A col K; row Q col Q
    assert_eq!((grid[aks], grid[qq]), (1.0, 0.5));
    assert_eq!(grid.iter().filter(|&&w| w != 0.0).count(), 2);
    let mut range = Range::empty();
    range.set_class("AKo", 1.0).unwrap();
    assert_eq!(range.to_string(), "AKo");
    assert_eq!(range.set_class("AKx", 1.0), Err(Error::InvalidToken));
}

fn any_spot() -> impl Strategy<Value = (Range, Vec<Card>)> {
    let deck: Vec<Card> = Card::all().collect();
    (
        prop::collection::vec(
            (0..1326usize, prop_oneof![Just(1.0), 0.01..=1.0f64]),
            1..300,
        ),
        (3..=5usize).prop_flat_map(move |n| prop::sample::subsequence(deck.clone(), n)),
    )
        .prop_map(|(combos, board)| {
            let mut range = Range::empty();
            for (i, w) in combos {
                range
                    .set(fieldtest_core::Combo::from_index(i).unwrap(), w)
                    .unwrap();
            }
            (range, board)
        })
}

proptest! {
    #[test]
    fn classes_partition_the_live_combos((range, board) in any_spot()) {
        let b = analyze(&range, &board);
        let total: f64 = HandClass::ALL.iter().map(|&c| b.class_weight(c)).sum();
        let shares: f64 = HandClass::ALL.iter().map(|&c| b.class_share(c)).sum();
        prop_assert!((total - b.live()).abs() <= 1e-9 * b.live().max(1.0));
        prop_assert!(b.live() == 0.0 || (shares - 1.0).abs() <= 1e-12);
        let mut live = range.clone();
        for (combo, _) in range.combos() {
            if board.contains(&combo.high()) || board.contains(&combo.low()) {
                live.set(combo, 0.0).unwrap();
            }
        }
        prop_assert_eq!(filter(&range, &board, &HandClass::ALL, &[]), live);
    }

    #[test]
    fn a_filtered_range_holds_only_kept_classes(
        (range, board) in any_spot(),
        kept in prop::sample::subsequence(HandClass::ALL.to_vec(), 0..=15),
    ) {
        let b = analyze(&filter(&range, &board, &kept, &[]), &board);
        for class in HandClass::ALL {
            prop_assert!(kept.contains(&class) || b.class_weight(class) == 0.0);
        }
    }
}
