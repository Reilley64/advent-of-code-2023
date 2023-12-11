use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut sum = 0;
    for (r, row) in grid.iter().enumerate() {
        for (c, &char) in row.iter().enumerate() {
            if char != '*' {
                continue;
            }

            let mut cs = HashSet::new();
            for cr in vec![r - 1, r, r + 1] {
                for mut cc in vec![c - 1, c, c + 1] {
                    if cr < 0 || cr >= grid.len() || cc < 0 || cc >= grid[cr].len() || !grid[cr][cc].is_numeric() {
                        continue;
                    }

                    while cc > 0 && grid[cr][cc - 1].is_numeric() {
                        cc -= 1;
                    }

                    cs.insert((cr, cc));
                }
            }

            if cs.len() != 2 {
                continue;
            }

            let mut ns = Vec::new();
            for (cr, mut cc) in cs {
                let mut s = String::new();
                while cc < grid[cr].len() && grid[cr][cc].is_numeric() {
                    s.push(grid[cr][cc]);
                    cc += 1;
                }
                ns.push(s.parse::<i32>().unwrap());
            }

            sum += ns[0] * ns[1];
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("test.txt");
        let output = process(input);
        assert_eq!(output, 467835);
    }
}
