use std::collections::HashMap;

fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let mut parts = input.split("\n\n");
    let instructions = parts.next().unwrap();
    let nodes = parts.next().unwrap();
    let node_map: HashMap<&str, (&str, &str)> = nodes
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split('=').map(|s| s.trim()).collect();
            let key = parts[0];
            let (left, right) = parts[1].trim_matches(|c| c == '(' || c == ')').split_once(", ").unwrap();
            (key, (left, right))
        })
        .collect();

    let mut steps = 0;
    let mut instruction_index = 0;
    let mut pattern = "AAA";
    while pattern != "ZZZ" {
        steps += 1;
        let (left, right) = node_map.get(pattern).unwrap();
        pattern = if instructions.chars().nth(instruction_index).unwrap() == 'L' {
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

    steps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("test1.txt");
        let output = process(input);
        assert_eq!(output, 2);
    }

    #[test]
    fn test2() {
        let input = include_str!("test2.txt");
        let output = process(input);
        assert_eq!(output, 6);
    }
}
