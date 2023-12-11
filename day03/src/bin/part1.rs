use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut cs = HashSet::new();
    for (r, row) in grid.iter().enumerate() {
        for (c, &char) in row.iter().enumerate() {
            if char.is_numeric() || char == '.' {
                continue;
            }

            for dr in r - 1..r + 2 {
                for mut dc in c - 1..c + 2 {
                    if dr < 0 || dr > grid.len() || dc < 0 || dc >= grid[dr].len() || !grid[dr][dc].is_numeric() {
                        continue;
                    }

                    while dc > 0 && grid[dr][dc - 1].is_numeric() {
                        dc -= 1;
                    }

                    cs.insert((dr, dc));
                }
            }
        }
    }

    let mut ns = Vec::new();
    for (r, mut c) in cs {
        let mut s = String::new();
        while c < grid[r].len() && grid[r][c].is_numeric() {
            s.push(grid[r][c]);
            c += 1;
        }
        ns.push(s.parse::<i32>().unwrap());
    }

    ns.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("test.txt");
        let output = process(input);
        assert_eq!(output, 4361);
    }
}
