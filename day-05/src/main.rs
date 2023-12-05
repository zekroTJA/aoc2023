use std::ops::Range;

struct Map {
    src: Range<isize>,
    offset: isize,
}

impl Map {
    fn parse(raw: &str) -> Self {
        let mut split = raw.split_ascii_whitespace();
        let dest: isize = split.next().unwrap().parse().unwrap();
        let start: isize = split.next().unwrap().parse().unwrap();
        let len: isize = split.next().unwrap().parse().unwrap();
        Self {
            src: start..start + len,
            offset: dest - start,
        }
    }
}

struct Maps(Vec<Map>);

impl Maps {
    fn parse(raw: &str) -> Self {
        Self(raw.split('\n').skip(1).map(Map::parse).collect())
    }

    fn map(&self, v: isize) -> isize {
        self.0
            .iter()
            .find(|m| m.src.contains(&v))
            .map(|m| v + m.offset)
            .unwrap_or(v)
    }

    fn map_ranges(&self, ranges: &[Range<isize>]) -> Vec<Range<isize>> {
        let mut ranges = ranges.to_vec();

        let mut mapped = vec![];

        while let Some(range) = ranges.pop() {
            let mut intersection_found = false;

            for m in &self.0 {
                if let Some(intersect) = range_intersection(&m.src, &range) {
                    mapped.push(intersect.start + m.offset..intersect.end + m.offset);

                    let before = range.start..intersect.start;
                    if !before.is_empty() {
                        ranges.push(before);
                    }

                    let after = intersect.end..range.end;
                    if !after.is_empty() {
                        ranges.push(after);
                    }

                    intersection_found = true;
                    break;
                }
            }

            if !intersection_found {
                mapped.push(range);
            }
        }

        mapped
    }
}

fn range_intersection(a: &Range<isize>, b: &Range<isize>) -> Option<Range<isize>> {
    let start = a.start.max(b.start);
    let end = a.end.min(b.end);

    if start >= end {
        None
    } else {
        Some(start..end)
    }
}

fn main() {
    let input: String = lib::read_input!();

    let parts: Vec<_> = input.split("\n\n").collect();

    let seeds: Vec<isize> = parts[0][7..]
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let maps: Vec<Maps> = parts[1..].iter().map(|v| Maps::parse(v)).collect();

    let p1: isize = seeds
        .iter()
        .map(|s| maps.iter().fold(*s, |c, m| m.map(c)))
        .min()
        .unwrap();

    println!("Part 1 Solution: {p1}");

    // ----------------------------------------------------------------------

    let numbers: Vec<isize> = parts[0][7..]
        .split_ascii_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let seeds: Vec<_> = numbers.chunks(2).map(|c| c[0]..c[0] + c[1]).collect();

    let p2: isize = maps
        .iter()
        .fold(seeds, |s, m| m.map_ranges(&s))
        .iter()
        .map(|v| v.clone().min().unwrap())
        .min()
        .unwrap();

    println!("Part 2 Solution: {p2}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_range_intersection() {
        assert_eq!(Some(2..4), range_intersection(&(1..4), &(2..6)));
        assert_eq!(Some(2..4), range_intersection(&(1..4), &(2..4)));
        assert_eq!(Some(2..3), range_intersection(&(1..4), &(2..3)));
        assert_eq!(Some(1..2), range_intersection(&(1..4), &(0..2)));
        assert_eq!(None, range_intersection(&(1..2), &(2..4)));
    }
}
