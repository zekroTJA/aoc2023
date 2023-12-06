fn main() {
    let input: String = lib::read_input!();

    let values: Vec<Vec<isize>> = input
        .split('\n')
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect();

    let time_distances: Vec<_> = values[0].iter().zip(values[1].iter()).collect();

    let p1: isize = time_distances
        .iter()
        .map(|(&t, &d)| possibilities(t, d))
        .product();
    println!("Part 1 Solution: {p1}");

    // ----------------------------------------------------------------------

    let mut time_distances = input
        .split('\n')
        .map(|line| line[9..].replace(' ', "").parse().unwrap());

    let time: isize = time_distances.next().unwrap();
    let distance: isize = time_distances.next().unwrap();

    let p2 = possibilities_faster(time as f64, distance as f64);
    println!("Part 2 Solution: {p2}");
}

// This was my original approach, but i wanted to explore how to solve this
// by using a quadratic equation.
fn possibilities(time: isize, distance: isize) -> isize {
    (1..time)
        .filter(|charge| charge * (time - charge) > distance)
        .count() as isize

    // // This is the imperative version of the code above which
    // // - for some reason - runs signifficantly faster (~3x) than
    // // the declarative version above. Probably, this is because
    // // zero cost abstractions might not be applied in debug mode.
    // // See benchmarks.txt for more details.
    //
    // let mut poss = 0;
    // for charge in 1..time {
    //     if charge * (time - charge) > distance {
    //         poss += 1;
    //     }
    // }
    // poss
}

fn possibilities_faster(t: f64, d: f64) -> isize {
    let h1 = 0.5 * (t - (t.powi(2) - 4.0 * d).sqrt());
    let h2 = 0.5 * (t + (t.powi(2) - 4.0 * d).sqrt());

    (h2.floor() - h1.floor()).abs() as isize
}
