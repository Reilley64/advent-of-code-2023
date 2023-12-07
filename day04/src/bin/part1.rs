use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut parts = line.split(":");
        let _identifier = parts.next().unwrap();
        let data = parts.next().unwrap();

        parts = data.split("|");
        let winning_numbers_string = parts.next().unwrap();
        let numbers_string = parts.next().unwrap();

        let winning_numbers: HashSet<i32> = winning_numbers_string
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();
        let numbers: HashSet<i32> = numbers_string
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect();

        let winning: Vec<i32> = winning_numbers
            .intersection(&numbers)
            .copied()
            .collect();

        if winning.len() > 0 {
            sum += u32::pow(2, (winning.len() as u32) - 1) as i32;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("./test.txt");
        let output = process(input);
        assert_eq!(output, 13);
    }
}
