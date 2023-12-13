use lib::*;

struct Grid(Vec<Vec<char>>);

impl Grid {
    fn parse(block: &str) -> Self {
        Self(block.split('\n').map(|l| l.chars().collect()).collect())
    }

    fn flip_cw(&self) -> Self {
        Self(flip_grid(&self.0))
    }

    fn diff_mirror_lines(&self, idx: usize) -> usize {
        (0..=idx)
            .rev()
            .zip(idx + 1..self.0.len())
            .fold(0usize, |diff_sum, (i, j)| {
                diff_sum + diff_lines(&self.0[i], &self.0[j])
            })
    }

    fn find_mirror_horizontal(&self, diff: usize) -> Option<usize> {
        let pivot = self.0.len() / 2 - 1;

        for i in (0..=pivot).rev() {
            if self.diff_mirror_lines(i) == diff {
                return Some(i);
            }
            if self.diff_mirror_lines(pivot + i + 1) == diff {
                return Some(pivot + i + 1);
            }
        }

        None
    }

    fn find_mirror_vertical(&self, diff: usize) -> Option<usize> {
        self.flip_cw().find_mirror_horizontal(diff)
    }

    fn mirror_sum(&self, diff: usize) -> usize {
        let h = self
            .find_mirror_horizontal(diff)
            .map(|v| (v + 1) * 100)
            .unwrap_or(0);
        let v = self.find_mirror_vertical(diff).map(|v| v + 1).unwrap_or(0);

        h + v
    }
}

fn diff_lines(a: &[char], b: &[char]) -> usize {
    a.iter().zip(b).filter(|(va, vb)| va != vb).count()
}

fn main() {
    let input: String = lib::read_input!();

    let grids: Vec<_> = input.split("\n\n").map(Grid::parse).collect();

    let p1: usize = grids.iter().map(|g| g.mirror_sum(0)).sum();
    p1!(p1);

    let p2: usize = grids.iter().map(|g| g.mirror_sum(1)).sum();
    p1!(p2);
}
