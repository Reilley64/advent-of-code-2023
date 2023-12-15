fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i64 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let empty_rows = grid
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|&char| char == '.'))
        .map(|(r, _)| r)
        .collect::<Vec<_>>();
    let empty_cols = (0..grid[0].len())
        .filter(|&c| grid.iter()
            .all(|row| row[c] == '.'))
        .collect::<Vec<_>>();
    let galaxies: Vec<_> = grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| row.iter()
            .enumerate()
            .filter(|&(_, &char)| char == '#').map(move |(c, _)| (r, c)))
        .collect();

    let mut total = 0;
    for (i, (r1, c1)) in galaxies.iter().enumerate() {
        for (r2, c2) in galaxies.iter().skip(i) {
            for r in (*r1).min(*r2)..(*r1).max(*r2) {
                total += if empty_rows.contains(&r) { 1000000 } else { 1 };
            }
            for c in (*c1).min(*c2)..(*c1).max(*c2) {
                total += if empty_cols.contains(&c) { 1000000 } else { 1 };
            }
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("test.txt");
        let output = process(input);
        assert_eq!(output, 82000210);
    }
}
