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

    let mut grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let start = find_start(&grid).expect("No start found");

    let mut seen = HashSet::from([start]);
    let mut queue = VecDeque::from([start]);
    let mut possible_s = HashSet::from(["|", "-", "J", "L", "7", "F"]);
    while let Some((r, c)) = queue.pop_front() {
        let char = grid[r][c];

        if r > 0 && "S|JL".contains(char) && "|7F".contains(grid[r - 1][c]) && !seen.contains(&(r - 1, c)) {
            seen.insert((r - 1, c));
            queue.push_back((r - 1, c));
            if char == 'S' {
                let definetly_s = HashSet::from(["|", "J", "L"]);
                possible_s = possible_s.intersection(&definetly_s).cloned().collect();
            }
        }

        if r < grid.len() - 1 && "S|7F".contains(char) && "|JL".contains(grid[r + 1][c]) && !seen.contains(&(r + 1, c)) {
            seen.insert((r + 1, c));
            queue.push_back((r + 1, c));
            if char == 'S' {
                let definetly_s = HashSet::from(["|", "7", "F"]);
                possible_s = possible_s.intersection(&definetly_s).cloned().collect();
            }
        }

        if c > 0 && "S-J7".contains(char) && "-LF".contains(grid[r][c - 1]) && !seen.contains(&(r, c - 1)) {
            seen.insert((r, c - 1));
            queue.push_back((r, c - 1));
            if char == 'S' {
                let definetly_s = HashSet::from(["-", "J", "7"]);
                possible_s = possible_s.intersection(&definetly_s).cloned().collect();
            }
        }

        if c < grid[r].len() - 1 && "S-LF".contains(char) && "-J7".contains(grid[r][c + 1]) && !seen.contains(&(r, c + 1)) {
            seen.insert((r, c + 1));
            queue.push_back((r, c + 1));
            if char == 'S' {
                let definetly_s = HashSet::from(["-", "L", "F"]);
                possible_s = possible_s.intersection(&definetly_s).cloned().collect();
            }
        }
    }

    let s = possible_s.iter().next().unwrap();
    grid[start.0][start.1] = (*s).parse().unwrap();
    grid = grid.into_iter()
        .enumerate()
        .map(|(r, row)| row.iter().enumerate().map(|(c, char)| if seen.contains(&(r, c)) { *char } else { '.' }).collect())
        .collect();

    let mut outside = HashSet::new();
    for (r, row) in grid.iter().enumerate() {
        let mut within = false;
        let mut up: Option<bool> = None;

        for (c, &char) in row.iter().enumerate() {
            match char {
                '|' => {
                    within = !within;
                }
                'L' | 'F' => {
                    up = Some(char == 'L');
                }
                '7' | 'J' => {
                    if (char == 'J' && !up.unwrap()) || (char == '7' && up.unwrap()) {
                        within = !within;
                    }
                    up = None;
                }
                _ => {},
            }

            if !within {
                outside.insert((r, c));
            }
        }
    }

    (grid.len() * grid[0].len() - outside.union(&seen).count()) as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test3() {
        let input = include_str!("test3.txt");
        let output = process(input);
        assert_eq!(output, 4);
    }

    #[test]
    fn test4() {
        let input = include_str!("test4.txt");
        let output = process(input);
        assert_eq!(output, 8);
    }

    #[test]
    fn test5() {
        let input = include_str!("test5.txt");
        let output = process(input);
        assert_eq!(output, 10);
    }
}
