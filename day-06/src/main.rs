fn main() {
    let input: String = lib::read_input!();

    let values: Vec<Vec<usize>> = input
        .split('\n')
        .map(|line| {
            line.split_ascii_whitespace()
                .skip(1)
                .map(|v| v.parse().unwrap())
                .collect()
        })
        .collect();

    let time_distances: Vec<_> = values[0].iter().zip(values[1].iter()).collect();

    let p1: usize = time_distances
        .iter()
        .map(|(&t, &d)| possibilities(t, d))
        .product();
    println!("Part 1 Solution: {p1}");

    // ----------------------------------------------------------------------

    let mut time_distances = input
        .split('\n')
        .map(|line| line[9..].replace(' ', "").parse().unwrap());

    let time: usize = time_distances.next().unwrap();
    let distance: usize = time_distances.next().unwrap();

    let p2 = possibilities(time, distance);
    println!("Part 2 Solution: {p2}");
}

fn possibilities(time: usize, distance: usize) -> usize {
    (1..time)
        .filter(|charge| charge * (time - charge) > distance)
        .count()

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
