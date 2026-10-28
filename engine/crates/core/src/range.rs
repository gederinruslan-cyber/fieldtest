use std::cmp::Ordering;
use std::fmt;

use crate::card::{RANKS, parse_rank};
use crate::{Card, Combo, Error};

/// A weight in [0, 1] for each of the 1,326 combos.
///
/// Notation: `AA`, `22+`, `77-55`, `AKs`, `AKo`, `AK`, `ATs+`, `KTs-K7s`, `AhKh`, each with an
/// optional `:weight`, comma separated; a later token overrides an earlier one. Printing is
/// canonical (pairs, then suited, then offsuit, runs compressed) and parses back to the same
/// range.
#[derive(Clone, PartialEq, Debug)]
pub struct Range {
    pub(crate) weights: [f64; 1326],
}

#[derive(Clone, Copy, PartialEq)]
enum Shape {
    Pair,
    Suited,
    Offsuit,
    /// Suited and offsuit, as in `AK`.
    Both,
}

/// A hand class such as `AKs`.
#[derive(Clone, Copy, PartialEq)]
struct Class {
    high: u8,
    low: u8,
    shape: Shape,
}

impl Class {
    fn parse(s: &str) -> Option<Class> {
        let (high, low, suffix) = match *s.as_bytes() {
            [h, l] => (h, l, None),
            [h, l, suffix] => (h, l, Some(suffix)),
            _ => return None,
        };
        let (high, low) = (parse_rank(high)?, parse_rank(low)?);
        let shape = match (high.cmp(&low), suffix) {
            (Ordering::Equal, None) => Shape::Pair,
            (Ordering::Greater, None) => Shape::Both,
            (Ordering::Greater, Some(b's')) => Shape::Suited,
            (Ordering::Greater, Some(b'o')) => Shape::Offsuit,
            _ => return None,
        };
        Some(Class { high, low, shape })
    }

    /// The class one step along a run: pairs move both ranks, other classes the kicker.
    fn with_low(self, low: u8) -> Class {
        let high = if self.shape == Shape::Pair {
            low
        } else {
            self.high
        };
        Class { high, low, ..self }
    }

    fn combos(self) -> impl Iterator<Item = Combo> {
        (0..4u8)
            .flat_map(|a| (0..4u8).map(move |b| (a, b)))
            .filter(move |&(a, b)| match self.shape {
                Shape::Pair => a > b,
                Shape::Suited => a == b,
                Shape::Offsuit => a != b,
                Shape::Both => true,
            })
            .filter_map(move |(a, b)| Combo::new(Card::new(self.high, a)?, Card::new(self.low, b)?))
    }
}

impl fmt::Display for Class {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (high, low) = (
            RANKS[self.high as usize] as char,
            RANKS[self.low as usize] as char,
        );
        let suffix = match self.shape {
            Shape::Suited => "s",
            Shape::Offsuit => "o",
            Shape::Pair | Shape::Both => "",
        };
        write!(f, "{high}{low}{suffix}")
    }
}

/// The classes a token names: `AKs`, `22+`, `ATs+`, `77-55`, `KTs-K7s`.
fn expand(body: &str) -> Option<Vec<Class>> {
    if let Some(start) = body.strip_suffix('+') {
        let class = Class::parse(start)?;
        let top = if class.shape == Shape::Pair {
            12
        } else {
            class.high - 1
        };
        return Some((class.low..=top).map(|r| class.with_low(r)).collect());
    }
    if let Some((a, b)) = body.split_once('-') {
        let (a, b) = (Class::parse(a)?, Class::parse(b)?);
        if a.shape != b.shape || (a.shape != Shape::Pair && a.high != b.high) {
            return None;
        }
        return Some(
            (a.low.min(b.low)..=a.low.max(b.low))
                .map(|r| a.with_low(r))
                .collect(),
        );
    }
    Class::parse(body).map(|class| vec![class])
}

/// A single combo such as `AhKh`.
fn specific(body: &str) -> Option<Combo> {
    let (a, b) = (body.get(..2)?, body.get(2..)?);
    Combo::new(a.parse().ok()?, b.parse().ok()?)
}

fn weight_suffix(weight: f64) -> String {
    if weight == 1.0 {
        String::new()
    } else {
        format!(":{weight}")
    }
}

impl Range {
    pub fn empty() -> Range {
        Range {
            weights: [0.0; 1326],
        }
    }

    pub fn parse(notation: &str) -> Result<Range, Error> {
        let mut range = Range::empty();
        if notation.trim().is_empty() {
            return Ok(range);
        }
        for token in notation.split(',') {
            let (body, weight) = match token.split_once(':') {
                Some((body, w)) => (body, w.trim().parse().map_err(|_| Error::InvalidWeight)?),
                None => (token, 1.0),
            };
            if !(weight > 0.0 && weight <= 1.0) {
                return Err(Error::InvalidWeight);
            }
            let body = body.trim();
            let combos: Vec<Combo> = match specific(body) {
                Some(combo) => vec![combo],
                None => expand(body)
                    .ok_or(Error::InvalidToken)?
                    .into_iter()
                    .flat_map(Class::combos)
                    .collect(),
            };
            for combo in combos {
                range.weights[combo.index()] = weight;
            }
        }
        Ok(range)
    }

    /// Sets one combo's weight; 0 removes it.
    pub fn set(&mut self, combo: Combo, weight: f64) -> Result<(), Error> {
        if !(0.0..=1.0).contains(&weight) {
            return Err(Error::InvalidWeight);
        }
        self.weights[combo.index()] = weight;
        Ok(())
    }

    pub fn weight(&self, combo: Combo) -> f64 {
        self.weights[combo.index()]
    }

    /// Combos with a positive weight, in `Combo::all` order.
    pub fn combos(&self) -> impl Iterator<Item = (Combo, f64)> + '_ {
        Combo::all()
            .zip(self.weights.iter().copied())
            .filter(|&(_, w)| w > 0.0)
    }

    /// Mean weight of each of the 169 classes, row by row over ranks A to 2: pairs on the
    /// diagonal, suited hands above it (row rank high), offsuit hands below it.
    pub fn grid_weights(&self) -> [f64; 169] {
        std::array::from_fn(|i| {
            let (row, col) = (12 - (i / 13) as u8, 12 - (i % 13) as u8);
            let class = match row.cmp(&col) {
                Ordering::Equal => Class {
                    high: row,
                    low: row,
                    shape: Shape::Pair,
                },
                Ordering::Greater => Class {
                    high: row,
                    low: col,
                    shape: Shape::Suited,
                },
                Ordering::Less => Class {
                    high: col,
                    low: row,
                    shape: Shape::Offsuit,
                },
            };
            let (sum, count) = class
                .combos()
                .fold((0.0, 0), |(s, n), c| (s + self.weight(c), n + 1));
            sum / f64::from(count)
        })
    }

    /// Sets every combo of a class such as `AKs`, `AK` or `QQ`; weight 0 removes it.
    pub fn set_class(&mut self, class: &str, weight: f64) -> Result<(), Error> {
        if !(0.0..=1.0).contains(&weight) {
            return Err(Error::InvalidWeight);
        }
        for combo in Class::parse(class).ok_or(Error::InvalidToken)?.combos() {
            self.weights[combo.index()] = weight;
        }
        Ok(())
    }

    /// The weight every combo of the class shares, if they all share one.
    fn uniform(&self, class: Class) -> Option<f64> {
        let mut weights = class.combos().map(|c| self.weight(c));
        let first = weights.next()?;
        weights.all(|w| w == first).then_some(first)
    }

    /// Writes `classes` (ordered from the top of a possible `+` run) as compressed runs.
    fn write_runs(&self, classes: &[Class], tokens: &mut Vec<String>) {
        let mut i = 0;
        while i < classes.len() {
            let Some(weight) = self.uniform(classes[i]) else {
                let combos = classes[i].combos().map(|c| (c, self.weight(c)));
                tokens.extend(
                    combos
                        .filter(|&(_, w)| w > 0.0)
                        .map(|(c, w)| format!("{c}{}", weight_suffix(w))),
                );
                i += 1;
                continue;
            };
            let end = i + classes[i..]
                .iter()
                .take_while(|&&c| self.uniform(c) == Some(weight))
                .count();
            if weight > 0.0 {
                let (top, bottom) = (classes[i], classes[end - 1]);
                let body = match (i, end - i) {
                    (_, 1) => top.to_string(),
                    (0, _) => format!("{bottom}+"),
                    _ => format!("{top}-{bottom}"),
                };
                tokens.push(format!("{body}{}", weight_suffix(weight)));
            }
            i = end;
        }
    }
}

impl fmt::Display for Range {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut tokens = Vec::new();
        let pairs: Vec<Class> = (0..13)
            .rev()
            .map(|r| Class {
                high: r,
                low: r,
                shape: Shape::Pair,
            })
            .collect();
        self.write_runs(&pairs, &mut tokens);
        for shape in [Shape::Suited, Shape::Offsuit] {
            for high in (1..13).rev() {
                let kickers: Vec<Class> = (0..high)
                    .rev()
                    .map(|low| Class { high, low, shape })
                    .collect();
                self.write_runs(&kickers, &mut tokens);
            }
        }
        f.write_str(&tokens.join(","))
    }
}
