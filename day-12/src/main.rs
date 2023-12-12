#![feature(iter_intersperse)]

use lib::*;
use std::{cell::RefCell, collections::HashMap, hash::Hash};

#[derive(Eq, PartialEq, Clone, Hash, Debug)]
enum Type {
    Operational,
    Damaged,
    Unknown,
}

impl Type {
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::Operational,
            '#' => Self::Damaged,
            '?' => Self::Unknown,
            x => panic!("invalid character: {x}"),
        }
    }
}

#[derive(Debug)]
struct MapLine {
    map: Vec<Type>,
    damaged: Vec<usize>,
}

impl MapLine {
    fn parse(line: &str) -> Self {
        let (map_raw, damaged_raw) = line.split_once(' ').unwrap();

        let damaged = damaged_raw.split(',').map(|v| v.parse().unwrap()).collect();
        let map = map_raw.chars().map(Type::parse).collect();

        Self { map, damaged }
    }

    fn count(&self) -> usize {
        type Cache<'a> = HashMap<(&'a [Type], &'a [usize]), usize>;
        let cache = RefCell::new(Cache::new());

        fn count<'a>(
            line: &'a [Type],
            damaged: &'a [usize],
            cache: &'a RefCell<Cache<'a>>,
        ) -> usize {
            if let Some(res) = cache.borrow().get(&(line, damaged)) {
                return *res;
            }

            if line.is_empty() {
                return if damaged.is_empty() { 1 } else { 0 };
            }

            if damaged.is_empty() {
                return if line.contains(&Type::Damaged) { 0 } else { 1 };
            }

            let mut c = 0;

            if line[0] == Type::Operational || line[0] == Type::Unknown {
                c += count(&line[1..], damaged, cache);
            }

            if (line[0] == Type::Damaged || line[0] == Type::Unknown)
                && line.len() >= damaged[0]
                && !line[..damaged[0]].contains(&Type::Operational)
                && (line.len() == damaged[0] || line[damaged[0]] != Type::Damaged)
            {
                if line.len() <= damaged[0] {
                    c += count(&[], &damaged[1..], cache)
                } else {
                    c += count(&line[damaged[0] + 1..], &damaged[1..], cache);
                }
            }

            cache.borrow_mut().insert((line, damaged), c);
            c
        }

        count(&self.map, &self.damaged, &cache)
    }

    fn unfold(&self, folds: usize) -> Self {
        let map = (0..folds)
            .map(|_| self.map.iter())
            .intersperse([Type::Unknown].iter())
            .flatten()
            .cloned()
            .collect();

        let damaged = self.damaged.repeat(folds);

        Self { map, damaged }
    }
}

fn main() {
    let input: String = lib::read_input!();

    let maps: Vec<_> = input.split('\n').map(MapLine::parse).collect();

    let p1: usize = maps.iter().map(|m| m.count()).sum();
    p1!(p1);

    let maps: Vec<_> = maps.iter().map(|m| m.unfold(5)).collect();
    let p2: usize = maps.iter().map(|m| m.count()).sum();
    p1!(p2);
}
