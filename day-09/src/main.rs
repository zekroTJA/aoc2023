use lib::*;

fn diff(v: &[isize]) -> Vec<isize> {
    v.windows(2).map(|w| w[1] - w[0]).collect()
}

fn diffs(v: &[isize]) -> Vec<Vec<isize>> {
    let mut curr = v.to_vec();
    let mut res = vec![curr.clone()];

    loop {
        curr = diff(&curr);
        res.push(curr.clone());
        if curr.iter().all(|&v| v == curr[0]) {
            break;
        }
    }

    res
}

fn extrapolate(v: &[Vec<isize>]) -> isize {
    let mut iter = v.iter().rev();
    let e = *iter.next().unwrap().last().unwrap();
    iter.fold(e, |e, line| e + line.last().unwrap())
}

fn extrapolate_backwards(v: &[Vec<isize>]) -> isize {
    let mut iter = v.iter().rev();
    let e = *iter.next().unwrap().first().unwrap();
    iter.fold(e, |e, line| line.first().unwrap() - e)
}

fn main() {
    let input: String = lib::read_input!();

    let histories: Vec<Vec<isize>> = input
        .split('\n')
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect();

    let p1: isize = histories
        .iter()
        .map(|v| diffs(v))
        .map(|v| extrapolate(&v))
        .sum();

    p1!(p1);

    let p2: isize = histories
        .iter()
        .map(|v| diffs(v))
        .map(|v| extrapolate_backwards(&v))
        .sum();

    p2!(p2);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_diff() {
        assert_eq!(diff(&[1, 3, 6, 10]), vec![2, 3, 4]);
    }
}
