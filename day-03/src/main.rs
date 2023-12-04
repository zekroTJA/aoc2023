use core::fmt;
use std::collections::HashMap;

use lib::pos::Pos;

#[derive(Debug)]
struct Number {
    number: u32,
    start_point: Pos,
    end_point: Pos,
}

impl fmt::Display for Number {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} [{}, {}]",
            self.number, self.start_point, self.end_point
        )
    }
}

fn is_serial_number(grid: &[Vec<char>], number: &Number) -> Option<(char, Pos)> {
    for x in number.start_point.x - 1..=number.end_point.x + 1 {
        let y = number.start_point.y - 1;
        if y < 0 || y >= grid.len() as isize || x < 0 || x >= grid[0].len() as isize {
            continue;
        }
        let c = grid[y as usize][x as usize];
        if !c.is_ascii_digit() && c != '.' {
            return Some((c, (x, y).into()));
        }
    }

    for x in number.start_point.x - 1..=number.end_point.x + 1 {
        let y = number.start_point.y + 1;
        if y < 0 || y >= grid.len() as isize || x < 0 || x >= grid[0].len() as isize {
            continue;
        }
        let c = grid[y as usize][x as usize];
        if !c.is_ascii_digit() && c != '.' {
            return Some((c, (x, y).into()));
        }
    }

    for y in number.start_point.y - 1..=number.start_point.y + 1 {
        let x = number.start_point.x - 1;
        if y < 0 || y >= grid.len() as isize || x < 0 || x >= grid[0].len() as isize {
            continue;
        }
        let c = grid[y as usize][x as usize];
        if !c.is_ascii_digit() && c != '.' {
            return Some((c, (x, y).into()));
        }
    }

    for y in number.end_point.y - 1..=number.end_point.y + 1 {
        let x = number.end_point.x + 1;
        if y < 0 || y >= grid.len() as isize || x < 0 || x >= grid[0].len() as isize {
            continue;
        }
        let c = grid[y as usize][x as usize];
        if !c.is_ascii_digit() && c != '.' {
            return Some((c, (x, y).into()));
        }
    }

    None
}

fn main() {
    let input: String = lib::read_input!();

    let grid: Vec<Vec<_>> = input
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    let mut numbers = vec![];
    let mut buff = String::new();
    let mut start_pos = None;
    let mut last_pos = Pos::default();
    for (y, line) in grid.iter().enumerate() {
        for (x, &char) in line.iter().enumerate() {
            if char.is_ascii_digit() {
                buff.push(char);
                if start_pos.is_none() {
                    start_pos = Some((x as isize, y as isize).into())
                }
            } else if start_pos.is_some() {
                numbers.push(Number {
                    number: buff.parse().unwrap(),
                    start_point: start_pos.unwrap(),
                    end_point: last_pos,
                });
                buff.clear();
                start_pos = None;
            }
            last_pos = (x as isize, y as isize).into();
        }
    }

    let res: Vec<_> = numbers
        .iter()
        .filter(|n| is_serial_number(&grid, n).is_some())
        .collect();

    let p1: u32 = res.iter().map(|r| r.number).sum();
    println!("Part 1 Solution: {p1}");

    let gears: Vec<_> = numbers
        .iter()
        .filter_map(|n| is_serial_number(&grid, n).map(|ident| (n, ident)))
        .filter(|(_, (c, _))| *c == '*')
        .collect();

    let mut map: HashMap<Pos, Vec<&Number>> = HashMap::new();
    for (number, (_, pos)) in gears {
        let v = map.entry(pos).or_default();
        v.push(number);
    }

    let p2: u32 = map
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().map(|n| n.number).product::<u32>())
        .sum();
    println!("Part 2 Solution: {p2}");
}
