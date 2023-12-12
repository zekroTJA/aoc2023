#![feature(iter_intersperse)]

use lib::*;
use std::{cell::RefCell, collections::HashMap, hash::Hash};

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
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
    blocks: Vec<usize>,
}

impl MapLine {
    fn parse(line: &str) -> Self {
        let (map_raw, blocks_raw) = line.split_once(' ').unwrap();

        let blocks = blocks_raw.split(',').map(|v| v.parse().unwrap()).collect();
        let map = map_raw.chars().map(Type::parse).collect();

        Self { map, blocks }
    }

    fn count(&self) -> usize {
        type Cache<'a> = HashMap<(&'a [Type], &'a [usize]), usize>;
        let cache = RefCell::new(Cache::new());

        fn count<'a>(
            line: &'a [Type],
            blocks: &'a [usize],
            cache: &'a RefCell<Cache<'a>>,
        ) -> usize {
            if let Some(res) = cache.borrow().get(&(line, blocks)) {
                return *res;
            }

            if line.is_empty() {
                return if blocks.is_empty() { 1 } else { 0 };
            }

            if blocks.is_empty() {
                return if line.contains(&Type::Damaged) { 0 } else { 1 };
            }

            let spring = line[0];
            let block_size = blocks[0];
            let mut c = 0;

            if spring == Type::Operational || spring == Type::Unknown {
                c += count(&line[1..], blocks, cache);
            }

            if (spring == Type::Damaged || spring == Type::Unknown)
                && line.len() >= block_size
                && !line[..block_size].contains(&Type::Operational)
                && (line.len() == block_size || line[block_size] != Type::Damaged)
            {
                if line.len() <= block_size {
                    c += count(&[], &blocks[1..], cache)
                } else {
                    c += count(&line[block_size + 1..], &blocks[1..], cache);
                }
            }

            cache.borrow_mut().insert((line, blocks), c);
            c
        }

        count(&self.map, &self.blocks, &cache)
    }

    fn unfold(&self, folds: usize) -> Self {
        let map = (0..folds)
            .map(|_| self.map.iter())
            .intersperse([Type::Unknown].iter())
            .flatten()
            .cloned()
            .collect();

        let blocks = self.blocks.repeat(folds);

        Self { map, blocks }
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
