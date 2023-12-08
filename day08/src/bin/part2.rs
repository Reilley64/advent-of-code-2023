use num_integer::Integer;
use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i64 {
    let mut parts = input.split("\n\n");
    let instructions = parts.next().unwrap();
    let nodes = parts.next().unwrap();
    let node_map: HashMap<&str, (&str, &str)> = nodes
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
            let pattern = parts[0];
            let (left, right) = parts[1].trim_matches(|c| c == '(' || c == ')').split_once(", ").unwrap();
            (pattern, (left, right))
        })
        .collect();

    let mut patterns: Vec<&str> = node_map.keys().filter(|&key| key.ends_with("A")).cloned().collect();
    let mut cycles = Vec::new();
    for pattern in patterns.iter_mut() {
        let mut instruction_index = 0;
        let mut cycle_count = 0;

        while !pattern.ends_with("Z") {
            cycle_count += 1;
            let (left, right) = node_map.get(pattern).unwrap();
            *pattern = if instructions.chars().nth(instruction_index).unwrap() == 'L' {
                left
            } else {
                right
            };
            instruction_index = if instruction_index + 1 == instructions.len() {
                0
            } else {
                instruction_index + 1
            };
        }

        cycles.push(cycle_count);
    }

    let lcm = cycles
        .iter()
        .fold(None, |acc: Option<i64>, x| match acc {
            Some(acc) => Some(acc.lcm(&x)),
            None => Some(*x),
        })
        .unwrap();

    lcm
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("test3.txt");
        let output = process(input);
        assert_eq!(output, 6);
    }
}
