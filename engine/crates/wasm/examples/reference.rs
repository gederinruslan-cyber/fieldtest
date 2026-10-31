//! Runs the WASM-exported functions natively for the parity test: reads lines of
//! tab-separated `function` and arguments from stdin and prints `ok<TAB>value` or
//! `err<TAB>code` per line; a list of numbers prints comma separated.

use std::io::{BufRead, stdin};

use fieldtest_wasm::*;

fn list(values: Vec<f64>) -> String {
    values
        .iter()
        .map(f64::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn main() {
    for line in stdin().lock().lines().map_while(Result::ok) {
        let args: Vec<&str> = line.split('\t').collect();
        let arg = |i: usize| args.get(i).copied().unwrap_or("");
        let result = match arg(0) {
            "analysisLabels" => Ok(analysis_labels().join(",")),
            "normalizeRange" => normalize_range(arg(1)),
            "rangeSize" => range_size(arg(1)).map(|v| v.to_string()),
            "evaluateHand" => evaluate_hand(arg(1)).map(|v| v.to_string()),
            "handCategory" => hand_category(arg(1)),
            "analyzeRange" => analyze_range(arg(1), arg(2)).map(list),
            "filterRange" => filter_range(arg(1), arg(2), arg(3)),
            "gridWeights" => grid_weights(arg(1)).map(list),
            "equity" => match (arg(3).parse(), arg(4).parse()) {
                (Ok(samples), Ok(seed)) => equity(arg(1), arg(2), samples, seed).map(list),
                _ => Err("InvalidSamples".into()),
            },
            "setClass" => match arg(3).parse() {
                Ok(weight) => set_class(arg(1), arg(2), weight),
                Err(_) => Err("InvalidWeight".into()),
            },
            other => Err(format!("UnknownFunction {other}")),
        };
        match result {
            Ok(value) => println!("ok\t{value}"),
            Err(code) => println!("err\t{code}"),
        }
    }
}
