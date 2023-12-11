use lib::pos::Pos;
use std::cell::RefCell;

#[derive(Eq, PartialEq, Debug)]
struct Grid(Vec<Vec<char>>);

impl Grid {
    fn parse(input: &str) -> Self {
        Self(input.split('\n').map(|c| c.chars().collect()).collect())
    }

    fn flip_cw(&self) -> Self {
        let mut new = vec![];

        for x in 0..self.0[0].len() {
            let mut row = vec![];
            for y in 0..self.0.len() {
                row.push(self.0[y][x])
            }
            new.push(row);
        }

        Self(new)
    }

    fn find_all_galaxies(&self) -> Vec<Pos> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|(_, &c)| c == '#')
                    .map(move |(x, _)| (x as isize, y as isize).into())
            })
            .collect()
    }

    fn empty_rows(&self) -> Vec<usize> {
        self.0
            .iter()
            .enumerate()
            .filter(|(_, l)| l.iter().all(|&c| c == '.'))
            .map(|(y, _)| y)
            .collect()
    }

    fn empty_cols(&self) -> Vec<usize> {
        self.flip_cw().empty_rows()
    }

    fn solve(&self, multiplier: usize) -> usize {
        let add: usize = multiplier - 1;

        let empty_rows = self.empty_rows();
        let empty_cols = self.empty_cols();

        let galaxies: Vec<_> = self
            .find_all_galaxies()
            .iter()
            .cloned()
            .map(RefCell::new)
            .collect();

        for (i, &row) in empty_rows.iter().enumerate() {
            galaxies
                .iter()
                .filter(|p| p.borrow().y as usize > row + add * i)
                .for_each(|p| {
                    let pc = *p.borrow();
                    *p.borrow_mut() = Pos {
                        x: pc.x,
                        y: pc.y + add as isize,
                    }
                });
        }

        for (i, &col) in empty_cols.iter().enumerate() {
            galaxies
                .iter()
                .filter(|p| p.borrow().x as usize > col + add * i)
                .for_each(|p| {
                    let pc = *p.borrow();
                    *p.borrow_mut() = Pos {
                        x: pc.x + add as isize,
                        y: pc.y,
                    };
                });
        }

        galaxies
            .iter()
            .flat_map(|g1| {
                galaxies
                    .iter()
                    .map(move |g2| g1.borrow().manhattan_distance(*g2.borrow()))
            })
            .sum::<usize>()
            / 2
    }
}

fn main() {
    let input: String = lib::read_input!();

    let grid = Grid::parse(&input);

    let p1 = grid.solve(2);
    println!("Part 1 Solution: {p1}");

    let p2 = grid.solve(1_000_000);
    println!("Part 2 Solution: {p2}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_flip_cw() {
        let g = Grid(vec![vec!['a', 'b', 'c'], vec!['d', 'e', 'f']]);
        let ex = Grid(vec![vec!['a', 'd'], vec!['b', 'e'], vec!['c', 'f']]);
        assert_eq!(g.flip_cw(), ex);
    }
}
