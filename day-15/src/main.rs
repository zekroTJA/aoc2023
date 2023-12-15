use std::{cell::RefCell, collections::HashMap};

use lib::*;

enum Op<'a> {
    Remove { label: &'a str },
    Add { label: &'a str, fl: usize },
}

impl<'a> Op<'a> {
    fn parse(v: &'a str) -> Self {
        if let Some(label) = v.strip_suffix('-') {
            Self::Remove { label }
        } else {
            let (label, fl) = v.split_once('=').unwrap();
            let fl = fl.parse().unwrap();
            Self::Add { label, fl }
        }
    }
}

fn hash(v: &str) -> usize {
    let mut curr = 0;

    for c in v.chars() {
        curr += c as usize;
        curr *= 17;
        curr %= 256;
    }

    curr
}

fn main() {
    let input: String = lib::read_input!();

    let p1: usize = input.split(',').map(hash).sum();
    p1!(p1);

    // ------------------------------------------------

    let mut map = HashMap::new();

    for op in input.split(',').map(Op::parse) {
        match op {
            Op::Add { label, fl } => {
                let entry = map.entry(hash(label)).or_insert(RefCell::new(vec![]));
                let mut lenses = entry.borrow_mut();

                if let Some(i) = lenses.iter().position(|(lbl, _)| lbl == &label) {
                    lenses.remove(i);
                    lenses.insert(i, (label, fl));
                } else {
                    lenses.push((label, fl));
                }
            }
            Op::Remove { label } => {
                if let Some(mut lenses) = map.get(&hash(label)).map(|v| v.borrow_mut()) {
                    if let Some(i) = lenses.iter().position(|(lbl, _)| lbl == &label) {
                        lenses.remove(i);
                    };
                };
            }
        }
    }

    let p2: usize = map
        .iter()
        .flat_map(|(&k, v)| {
            v.borrow()
                .iter()
                .enumerate()
                .map(|(idx, &lens)| (k, idx, lens))
                .collect::<Vec<_>>()
        })
        .map(|(k, idx, (_, fl))| (k + 1) * (idx + 1) * fl)
        .sum();
    p2!(p2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
        assert_eq!(hash("rn"), 0);
    }
}
