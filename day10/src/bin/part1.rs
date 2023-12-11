use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    fn find_start(grid: &Vec<Vec<char>>) -> Result<(usize, usize), &'static str> {
        for (r, row) in grid.iter().enumerate() {
            for (c, &char) in row.iter().enumerate() {
                if char == 'S' {
                    return Ok((r, c));
                }
            }
        }

        Err("No start found")
    }

    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = find_start(&grid).expect("No start found");

    let mut seen = HashSet::from([start]);
    let mut queue = VecDeque::from([start]);
    while let Some((r, c)) = queue.pop_front() {
        let char = grid[r][c];

        if r > 0 && "S|JL".contains(char) && "|7F".contains(grid[r - 1][c]) && !seen.contains(&(r - 1, c)) {
            seen.insert((r - 1, c));
            queue.push_back((r - 1, c));
        }

        if r < grid.len() - 1 && "S|7F".contains(char) && "|JL".contains(grid[r + 1][c]) && !seen.contains(&(r + 1, c)) {
            seen.insert((r + 1, c));
            queue.push_back((r + 1, c));
        }

        if c > 0 && "S-J7".contains(char) && "-LF".contains(grid[r][c - 1]) && !seen.contains(&(r, c - 1)) {
            seen.insert((r, c - 1));
            queue.push_back((r, c - 1));
        }

        if c < grid[r].len() - 1 && "S-LF".contains(char) && "-J7".contains(grid[r][c + 1]) && !seen.contains(&(r, c + 1)) {
            seen.insert((r, c + 1));
            queue.push_back((r, c + 1));
        }
    }

    (seen.len() / 2) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("test1.txt");
        let output = process(input);
        assert_eq!(output, 4);
    }

    #[test]
    fn test2() {
        let input = include_str!("test2.txt");
        let output = process(input);
        assert_eq!(output, 8);
    }
}
