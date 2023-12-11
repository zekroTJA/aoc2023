use lib::*;
use num::Integer;
use std::collections::HashMap;

fn parse_mapping(line: &str) -> (&str, (&str, &str)) {
    let (from, to) = line.split_once(" = ").unwrap();
    let to = to[1..to.len() - 1].split_once(", ").unwrap();
    (from, to)
}

fn main() {
    let input: String = lib::read_input!();

    let (dirs, mappings) = input.split_once("\n\n").unwrap();

    let map: HashMap<&str, (&str, &str)> = mappings.split('\n').map(parse_mapping).collect();

    let mut current = "AAA";
    let mut count = 0;

    'outer: loop {
        for d in dirs.chars() {
            if current == "ZZZ" {
                break 'outer;
            }

            current = match d {
                'L' => map[current].0,
                'R' => map[current].1,
                _ => panic!("invalid direction!"),
            };

            count += 1;
        }
    }

    println!("Part 1 Solution: {count}");

    // -----------------------------------------------------------

    let starting_points: Vec<_> = map
        .iter()
        .filter(|(from, _)| from.chars().nth(2) == Some('A'))
        .map(|(from, _)| *from)
        .collect();

    let mut all_counts = vec![];

    for sp in starting_points {
        let mut count = 0usize;
        let mut current = sp;
        let mut first = None;
        let mut counts = vec![];

        let mut dirs = dirs.chars().cycle();
        let mut d: char = dirs.next().unwrap();

        'outer: loop {
            while count == 0 || current.chars().nth(2) != Some('Z') {
                count += 1;
                current = match d {
                    'L' => map[current].0,
                    'R' => map[current].1,
                    _ => panic!("invalid direction!"),
                };
                d = dirs.next().unwrap();
            }

            counts.push(count);

            match first {
                None => {
                    first = Some(current);
                    count = 0;
                }
                Some(first) if current == first => break 'outer,
                _ => {}
            }
        }

        all_counts.push(counts[0]);
    }

    let mut iter = all_counts.iter();
    let lcm = iter.next().unwrap();
    let p2 = iter.fold(*lcm, |g, c| g.lcm(c));

    p2!(p2);
}
