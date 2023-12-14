use lib::*;
use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Grid {
    blocks: Vec<Pos>,
    balls: Vec<Pos>,
    size: usize,
}

impl Grid {
    fn parse(input: &str) -> Self {
        let lines = input.split('\n');

        let size = lines.clone().count();

        let blocks = lines
            .clone()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .map(move |(x, _)| (x as isize, y as isize).into())
            })
            .collect();

        let balls = lines
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == 'O')
                    .map(move |(x, _)| (x as isize, y as isize).into())
            })
            .collect();

        Self {
            blocks,
            balls,
            size,
        }
    }

    fn new_stone_pos(&self, pos: Pos, dir: Direction) -> Pos {
        match dir {
            Direction::Left => {
                let end = self
                    .blocks
                    .iter()
                    .rfind(|b| b.y == pos.y && b.x < pos.x)
                    .map(|b| b.x + 1)
                    .unwrap_or(0);

                let in_way = self
                    .balls
                    .iter()
                    .filter(|b| b.y == pos.y && b.x < pos.x && b.x >= end)
                    .count();

                Pos {
                    y: pos.y,
                    x: (end + in_way as isize),
                }
            }
            Direction::Right => {
                let end = self
                    .blocks
                    .iter()
                    .find(|b| b.y == pos.y && b.x > pos.x)
                    .map(|b| b.x - 1)
                    .unwrap_or(self.size as isize - 1);

                let in_way = self
                    .balls
                    .iter()
                    .filter(|b| b.y == pos.y && b.x > pos.x && b.x <= end)
                    .count();

                Pos {
                    y: pos.y,
                    x: (end - in_way as isize),
                }
            }
            Direction::Up => {
                let end = self
                    .blocks
                    .iter()
                    .rfind(|b| b.x == pos.x && b.y < pos.y)
                    .map(|b| b.y + 1)
                    .unwrap_or(0);

                let in_way = self
                    .balls
                    .iter()
                    .filter(|b| b.x == pos.x && b.y < pos.y && b.y >= end)
                    .count();

                Pos {
                    y: (end + in_way as isize),
                    x: pos.x,
                }
            }
            Direction::Down => {
                let end = self
                    .blocks
                    .iter()
                    .find(|b| b.x == pos.x && b.y > pos.y)
                    .map(|b| b.y - 1)
                    .unwrap_or(self.size as isize - 1);

                let in_way = self
                    .balls
                    .iter()
                    .filter(|b| b.x == pos.x && b.y > pos.y && b.y <= end)
                    .count();

                Pos {
                    y: (end - in_way as isize),
                    x: pos.x,
                }
            }
        }
    }

    fn move_stones(&mut self, dir: Direction) {
        self.balls = self
            .balls
            .iter()
            .map(|&b| self.new_stone_pos(b, dir))
            .collect();
        self.balls.sort();
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!();

        for y in 0..self.size {
            for x in 0..self.size {
                let p = (x as isize, y as isize).into();
                if self.balls.contains(&p) {
                    print!("O");
                } else if self.blocks.contains(&p) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!()
        }
    }
}

fn main() {
    let input: String = lib::read_input!();

    let mut grid = Grid::parse(&input);

    grid.move_stones(Direction::Up);

    let p1: usize = grid.balls.iter().map(|&p| grid.size - p.y as usize).sum();
    p1!(p1);

    // -----------------------------------------------------------------------------------

    let mut grid = Grid::parse(&input);

    let mut seen = HashSet::new();
    let mut grids = vec![grid.clone()];
    let mut loop_start = 0;

    loop {
        loop_start += 1;
        for d in [
            Direction::Up,
            Direction::Left,
            Direction::Down,
            Direction::Right,
        ] {
            grid.move_stones(d);
        }
        if !seen.insert(grid.clone()) {
            break;
        }
        grids.push(grid.clone());
    }

    let first = grids.iter().position(|g| g == &grid).unwrap();

    let grid = &grids[((1000000000 - first) % (loop_start - first)) + first];

    let p2: usize = grid.balls.iter().map(|&p| grid.size - p.y as usize).sum();
    p1!(p2);
}
