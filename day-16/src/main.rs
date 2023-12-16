use lib::*;
use std::{cell::RefCell, collections::HashSet};

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn parse(input: &str) -> Self {
        Self(input.split('\n').map(|l| l.chars().collect()).collect())
    }

    fn in_bounds(&self, p: Pos) -> bool {
        !p.is_negative() && p.y < self.0.len() as isize && p.x < self.0[0].len() as isize
    }

    fn mv(&self, p: Pos, dir: Direction, covered: &RefCell<HashSet<(Pos, Direction)>>) {
        if !self.in_bounds(p) || covered.borrow().contains(&(p, dir)) {
            return;
        }

        covered.borrow_mut().insert((p, dir));

        match self.0[p.y as usize][p.x as usize] {
            '.' => self.mv(p.mv(dir), dir, covered),
            '-' => match dir {
                Direction::Left | Direction::Right => self.mv(p.mv(dir), dir, covered),
                Direction::Down | Direction::Up => {
                    self.mv(p.mv(Direction::Left), Direction::Left, covered);
                    self.mv(p.mv(Direction::Right), Direction::Right, covered);
                }
            },
            '|' => match dir {
                Direction::Down | Direction::Up => self.mv(p.mv(dir), dir, covered),
                Direction::Left | Direction::Right => {
                    self.mv(p.mv(Direction::Down), Direction::Down, covered);
                    self.mv(p.mv(Direction::Up), Direction::Up, covered);
                }
            },
            '/' => match dir {
                Direction::Up => self.mv(p.mv(Direction::Left), Direction::Left, covered),
                Direction::Down => self.mv(p.mv(Direction::Right), Direction::Right, covered),
                Direction::Left => self.mv(p.mv(Direction::Up), Direction::Up, covered),
                Direction::Right => self.mv(p.mv(Direction::Down), Direction::Down, covered),
            },
            '\\' => match dir {
                Direction::Up => self.mv(p.mv(Direction::Right), Direction::Right, covered),
                Direction::Down => self.mv(p.mv(Direction::Left), Direction::Left, covered),
                Direction::Left => self.mv(p.mv(Direction::Down), Direction::Down, covered),
                Direction::Right => self.mv(p.mv(Direction::Up), Direction::Up, covered),
            },
            c => panic!("invalid character: {c}"),
        }
    }

    fn find_covered(&self, pos: Pos, dir: Direction) -> usize {
        let covered = RefCell::new(HashSet::new());
        self.mv(pos, dir, &covered);
        let covered = covered.borrow();
        let s: HashSet<_> = covered.iter().map(|(p, _)| p).collect();
        s.len()
    }
}

fn main() {
    let input: String = lib::read_input!();

    let grid = Grid::parse(&input);

    let p1 = grid.find_covered((0isize, 0isize).into(), Direction::Right);
    p1!(p1);

    let p2: usize = (0..grid.0[0].len())
        .map(|x| {
            (
                Pos {
                    x: x as isize,
                    y: 0,
                },
                Direction::Up,
            )
        })
        .chain((0..grid.0[0].len()).map(|x| {
            (
                Pos {
                    x: x as isize,
                    y: grid.0.len() as isize - 1,
                },
                Direction::Down,
            )
        }))
        .chain((0..grid.0.len()).map(|y| {
            (
                Pos {
                    x: 0,
                    y: y as isize,
                },
                Direction::Right,
            )
        }))
        .chain((0..grid.0.len()).map(|y| {
            (
                Pos {
                    x: grid.0[0].len() as isize - 1,
                    y: y as isize,
                },
                Direction::Left,
            )
        }))
        .map(|(p, d)| grid.find_covered(p, d))
        .max()
        .unwrap();
    p2!(p2);
}
