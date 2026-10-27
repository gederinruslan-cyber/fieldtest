use fieldtest_core::{Card, Category, evaluate};
use proptest::prelude::*;

fn cards(s: &str) -> Vec<Card> {
    s.split_whitespace().map(|c| c.parse().unwrap()).collect()
}

/// Wikipedia "Poker probability", 7-card hands (NH-2 §1). Exact. Moves into
/// `engine/fixtures/` with F0-04 once the human approves the candidates.
const SEVEN_CARD_COUNTS: [(Category, u64); 9] = [
    (Category::HighCard, 23_294_460),
    (Category::Pair, 58_627_800),
    (Category::TwoPair, 31_433_400),
    (Category::Trips, 6_461_620),
    (Category::Straight, 6_180_020),
    (Category::Flush, 4_047_644),
    (Category::FullHouse, 3_473_184),
    (Category::Quads, 224_848),
    (Category::StraightFlush, 41_584),
];
const ROYAL_FLUSHES: u64 = 4_324;

#[test]
fn seven_card_category_counts_are_exact() {
    let deck: Vec<Card> = Card::all().collect();
    let royal = evaluate(&cards("As Ks Qs Js Ts"));
    let mut counts = [0u64; 9];
    let mut royals = 0;
    let mut idx = [0, 1, 2, 3, 4, 5, 6];
    loop {
        let rank = evaluate(&idx.map(|i| deck[i]));
        counts[rank.category() as usize] += 1;
        royals += u64::from(rank == royal);
        // Next 7-combination of 0..52 in lexicographic order.
        let Some(i) = (0..7).rev().find(|&i| idx[i] < 45 + i) else {
            break;
        };
        idx[i] += 1;
        for j in i + 1..7 {
            idx[j] = idx[j - 1] + 1;
        }
    }
    assert_eq!(counts.iter().sum::<u64>(), 133_784_560);
    for (category, expected) in SEVEN_CARD_COUNTS {
        assert_eq!(counts[category as usize], expected, "{category:?}");
    }
    assert_eq!(royals, ROYAL_FLUSHES);
}

#[test]
fn reference_hands_order_correctly() {
    let e = |s| evaluate(&cards(s));
    assert!(e("As Ks Qs Js Ts") > e("9s Ks Qs Js Ts"));
    assert_eq!(e("9s Ks Qs Js Ts").category(), Category::StraightFlush);
    assert_eq!(e("Ac 2d 3h 4s 5c").category(), Category::Straight);
    assert!(e("Ac 2d 3h 4s 5c") < e("2c 3d 4h 5s 6c"));
    assert!(e("As Ah Kc Kd Qs") > e("Ac Ad Kh Ks Jc"));
    assert_eq!(e("As Ah Kc Kd Qs"), e("Ac Ad Kh Ks Qc"));
}

fn seven_cards() -> impl Strategy<Value = Vec<Card>> {
    prop::sample::subsequence(Card::all().collect::<Vec<_>>(), 7).prop_shuffle()
}

proptest! {
    #[test]
    fn seven_cards_rank_as_their_best_five(hand in seven_cards()) {
        let best = (0..7)
            .flat_map(|a| (a + 1..7).map(move |b| (a, b)))
            .map(|(a, b)| {
                let five: Vec<Card> = (0..7).filter(|&i| i != a && i != b).map(|i| hand[i]).collect();
                evaluate(&five)
            })
            .max();
        prop_assert_eq!(Some(evaluate(&hand)), best);
    }

    #[test]
    fn rank_ignores_card_order_and_suit_names(
        hand in seven_cards(),
        suits in Just(vec![0u8, 1, 2, 3]).prop_shuffle(),
    ) {
        let mut reversed = hand.clone();
        reversed.reverse();
        let renamed: Vec<Card> =
            hand.iter().map(|c| Card::new(c.rank(), suits[c.suit() as usize]).unwrap()).collect();
        prop_assert_eq!(evaluate(&hand), evaluate(&reversed));
        prop_assert_eq!(evaluate(&hand), evaluate(&renamed));
    }
}
