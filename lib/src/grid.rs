pub fn flip_grid<T: Copy>(grid: &[Vec<T>]) -> Vec<Vec<T>> {
    let mut new = vec![];

    for x in 0..grid[0].len() {
        let mut row = vec![];
        for line in grid {
            row.push(line[x])
        }
        new.push(row);
    }

    new
}
