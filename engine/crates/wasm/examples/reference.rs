//! Runs the WASM-exported functions natively for the parity test: reads `fn<TAB>arg` lines
//! from stdin and prints `ok<TAB>value` or `err<TAB>code` per line.

use std::io::{BufRead, stdin};

use fieldtest_wasm::{evaluate_hand, hand_category, normalize_range, range_size};

fn main() {
    for line in stdin().lock().lines().map_while(Result::ok) {
        let (function, arg) = line.split_once('\t').unwrap_or((&line, ""));
        let result = match function {
            "normalizeRange" => normalize_range(arg),
            "rangeSize" => range_size(arg).map(|v| v.to_string()),
            "evaluateHand" => evaluate_hand(arg).map(|v| v.to_string()),
            "handCategory" => hand_category(arg),
            _ => Err(format!("UnknownFunction {function}")),
        };
        match result {
            Ok(value) => println!("ok\t{value}"),
            Err(code) => println!("err\t{code}"),
        }
    }
}
