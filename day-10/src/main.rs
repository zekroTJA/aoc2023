#![feature(iter_intersperse)]

use core::fmt;
use lib::*;
use std::collections::HashSet;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Tile {
    H,
    V,
    NE,
    NW,
    SW,
    SE,
    Ground,
    Start,
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Tile::H => write!(f, "-"),
            Tile::V => write!(f, "|"),
            Tile::NE => write!(f, "L"),
            Tile::NW => write!(f, "J"),
            Tile::SW => write!(f, "7"),
            Tile::SE => write!(f, "F"),
            Tile::Ground => write!(f, "."),
            Tile::Start => write!(f, "S"),
        }
    }
}

impl Tile {
    fn parse(c: char) -> Self {
        match c {
            '|' => Self::V,
            '-' => Self::H,
            'L' => Self::NE,
            'J' => Self::NW,
            '7' => Self::SW,
            'F' => Self::SE,
            '.' => Self::Ground,
            'S' => Self::Start,
            x => panic!("invalid tile: {x}"),
        }
    }

    fn go_dirs(&self) -> Vec<Direction> {
        match self {
            Tile::H => vec![Direction::Left, Direction::Right],
            Tile::V => vec![Direction::Up, Direction::Down],
            Tile::NE => vec![Direction::Up, Direction::Right],
            Tile::NW => vec![Direction::Up, Direction::Left],
            Tile::SW => vec![Direction::Down, Direction::Left],
            Tile::SE => vec![Direction::Down, Direction::Right],
            Tile::Ground => vec![],
            Tile::Start => vec![
                Direction::Up,
                Direction::Down,
                Direction::Left,
                Direction::Right,
            ],
        }
    }

    fn receive_dirs(&self) -> Vec<Direction> {
        self.go_dirs().iter().map(|d| d.reverse()).collect()
    }
}

struct Grid(Vec<Vec<Tile>>);

impl Grid {
    fn parse(input: &str) -> Self {
        Self(
            input
                .split('\n')
                .rev()
                .map(|line| line.chars().map(Tile::parse).collect())
                .collect(),
        )
    }

    fn replace_non_connected(&self, inloop: &HashSet<Pos>) -> Self {
        Self(
            self.0
                .iter()
                .enumerate()
                .map(|(y, line)| {
                    line.iter()
                        .enumerate()
                        .map(|(x, t)| {
                            if inloop.contains(&(x as isize, y as isize).into()) {
                                *t
                            } else {
                                Tile::Ground
                            }
                        })
                        .collect()
                })
                .collect(),
        )
    }

    fn intersperse(&self) -> Self {
        let newline: Vec<_> = (0..self.0[0].len()).map(|_| Tile::Ground).collect();

        let rows: Vec<Vec<_>> = self
            .0
            .iter()
            .cloned()
            .intersperse_with(|| newline.clone())
            .map(|line| {
                line.iter()
                    .cloned()
                    .intersperse_with(|| Tile::Ground)
                    .collect()
            })
            .collect();

        let border: Vec<_> = (0..rows[0].len()).map(|_| Tile::Ground).collect();
        let rows = [vec![border.clone()], rows, vec![border.clone()]].concat();
        let rows: Vec<_> = rows
            .iter()
            .map(|c| [vec![Tile::Ground], c.clone(), vec![Tile::Ground]].concat())
            .collect();

        Self(rows)
    }

    fn find(&self, tile: Tile) -> Pos {
        for (y, line) in self.0.iter().enumerate() {
            for (x, t) in line.iter().enumerate() {
                if t == &tile {
                    return (x as isize, y as isize).into();
                }
            }
        }

        panic!("Tile not found!")
    }

    fn at(&self, pos: Pos) -> Tile {
        self.0[pos.y as usize][pos.x as usize]
    }

    fn in_bounds(&self, pos: Pos) -> bool {
        !pos.is_negative() && pos.y < self.0.len() as isize && pos.x < self.0[0].len() as isize
    }

    fn could_connect(&self, curr_pos: Pos, dir: Direction) -> bool {
        let to_pos = curr_pos + dir.into();
        if !self.in_bounds(to_pos) {
            return false;
        }

        let dest = self.at(to_pos);
        dest.receive_dirs().contains(&dir)
    }

    fn can_go(&self, curr_pos: Pos, dir: Direction) -> bool {
        if !self.could_connect(curr_pos, dir) {
            return false;
        }

        let curr = self.at(curr_pos);
        curr.go_dirs().contains(&dir)
    }

    fn neightbor_positions(&self, pos: Pos) -> Vec<Pos> {
        [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ]
        .iter()
        .map(|&d| pos + d.into())
        .filter(|&p| self.in_bounds(p))
        .collect()
    }

    fn neighbors(&self, pos: Pos) -> Vec<Pos> {
        let curr = self.at(pos);
        curr.go_dirs()
            .iter()
            .filter(|&&d| self.can_go(pos, d))
            .map(|&d| pos + d.into())
            .collect()
    }

    fn part1(&self) -> (usize, HashSet<Pos>) {
        let start_pos = self.find(Tile::Start);

        let mut queue = vec![start_pos];
        let mut visited = HashSet::new();
        let mut i = 0;
        while let Some(next) = queue.pop() {
            i += 1;
            visited.insert(next);

            let neighbors = self.neighbors(next);
            if neighbors.iter().filter(|&p| visited.contains(p)).count() > 1 {
                return (i / 2, visited);
            }

            neighbors
                .iter()
                .filter(|v| !visited.contains(v))
                .for_each(|&v| queue.insert(0, v));
        }

        panic!("no result found!");
    }

    fn part2(&self) -> usize {
        let start_pos = (0isize, 0isize).into();

        let mut queue: Vec<_> = vec![start_pos];
        let mut visited = HashSet::new();
        while let Some(next) = queue.pop() {
            visited.insert(next);

            for p in self.neightbor_positions(next) {
                if visited.contains(&p) {
                    continue;
                }

                let t = self.at(p);
                if t != Tile::Ground {
                    continue;
                }

                if self.could_connect(p, Direction::Left) && self.could_connect(p, Direction::Left)
                    || self.could_connect(p, Direction::Up)
                        && self.could_connect(p, Direction::Down)
                {
                    continue;
                }

                queue.push(p);
            }
        }

        let outer = visited
            .iter()
            .filter(|p| p.x % 2 == 1 && p.y % 2 == 1)
            .count();

        let mut all = 0;
        for (y, line) in self.0.iter().enumerate() {
            if y % 2 == 0 {
                continue;
            }
            for (x, t) in line.iter().enumerate() {
                if x % 2 == 0 || t != &Tile::Ground {
                    continue;
                }
                all += 1;
            }
        }

        all - outer
    }

    #[allow(dead_code)]
    fn print(&self) {
        for line in self.0.iter().rev() {
            for c in line {
                print!("{c}");
            }
            println!();
        }
    }
}

fn main() {
    let input: String = lib::read_input!();

    let grid = Grid::parse(&input);

    let (p1, inloop) = grid.part1();
    p1!(p1);

    // ---------------------------------------------

    let grid = grid.replace_non_connected(&inloop).intersperse();

    let p2 = grid.part2();
    p2!(p2);
}
