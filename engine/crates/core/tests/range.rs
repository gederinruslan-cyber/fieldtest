use fieldtest_core::{Card, Combo, Error, Range};
use proptest::prelude::*;

fn weights(s: &str) -> Vec<f64> {
    Range::parse(s).unwrap().combos().map(|(_, w)| w).collect()
}

#[test]
fn reference_ranges_count_their_combos() {
    for (notation, combos) in [
        ("22+", 78),
        ("ATs+", 16),
        ("AKo", 12),
        ("AK", 16),
        ("KTs-K7s", 16),
        ("77-55", 18),
        ("AhKh", 1),
        ("22+,A2+,K2+,Q2+,J2+,T2+,92+,82+,72+,62+,52+,42+,32", 1326),
    ] {
        assert_eq!(weights(notation).len(), combos, "{notation}");
    }
    let mixed = weights("QQ+,AKs:0.5");
    assert_eq!(mixed.iter().filter(|&&w| w == 1.0).count(), 18);
    assert_eq!(mixed.iter().filter(|&&w| w == 0.5).count(), 4);
    assert!(Range::parse("").unwrap().combos().next().is_none());
}

#[test]
fn invalid_notation_is_an_error_code() {
    for (notation, error) in [
        ("AA:0", Error::InvalidWeight),
        ("AA:1.5", Error::InvalidWeight),
        ("AA:x", Error::InvalidWeight),
        ("AKx", Error::InvalidToken),
        ("AA-KQ", Error::InvalidToken),
        ("AKs-QJs", Error::InvalidToken),
        ("AhAh", Error::InvalidToken),
        ("AA,,KK", Error::InvalidToken),
        ("1h2h", Error::InvalidToken),
    ] {
        assert_eq!(Range::parse(notation), Err(error), "{notation}");
    }
}

#[test]
fn all_cards_and_combos_round_trip() {
    let cards: Vec<Card> = Card::all().collect();
    assert_eq!(cards.len(), 52);
    for c in &cards {
        assert_eq!(c.to_string().parse::<Card>(), Ok(*c));
    }
    let combos: Vec<Combo> = Combo::all().collect();
    assert_eq!(combos.len(), 1326);
    for (i, c) in combos.iter().enumerate() {
        assert_eq!(c.index(), i);
        assert_eq!(Combo::new(c.low(), c.high()), Some(*c));
    }
}

fn weight() -> impl Strategy<Value = f64> {
    prop_oneof![
        Just(1.0),
        Just(0.5),
        (0.0..=1.0f64).prop_filter("positive", |w| *w > 0.0)
    ]
}

fn class_token() -> impl Strategy<Value = String> {
    const RANKS: &[u8] = b"23456789TJQKA";
    (
        0..13usize,
        0..13usize,
        prop::sample::select(vec!["s", "o", ""]),
        weight(),
    )
        .prop_map(|(a, b, suffix, w)| {
            let (hi, lo) = (RANKS[a.max(b)] as char, RANKS[a.min(b)] as char);
            let suffix = if a == b { "" } else { suffix };
            format!("{hi}{lo}{suffix}:{w}")
        })
}

fn any_range() -> impl Strategy<Value = Range> {
    (
        prop::collection::vec(class_token(), 0..12),
        prop::collection::vec((0..1326usize, prop_oneof![Just(0.0), weight()]), 0..24),
    )
        .prop_map(|(classes, combos)| {
            let mut range = Range::parse(&classes.join(",")).unwrap();
            for (i, w) in combos {
                range.set(Combo::from_index(i).unwrap(), w).unwrap();
            }
            range
        })
}

proptest! {
    #[test]
    fn printed_ranges_parse_back_identically(range in any_range()) {
        let printed = range.to_string();
        let reparsed = Range::parse(&printed).unwrap();
        prop_assert_eq!(&reparsed, &range);
        prop_assert_eq!(reparsed.to_string(), printed);
    }

    #[test]
    fn parsing_never_panics(s in "[2-9TJQKAshdco+:,.0-9 x-]{0,24}|\\PC{0,24}") {
        let _ = Range::parse(&s);
    }
}
