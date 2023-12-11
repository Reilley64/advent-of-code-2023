use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    fn find_numbers(s: &str) -> (String, String) {
        let number_map = HashMap::from([
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
        ]);

        let pattern = Regex::new(&format!("({})", number_map.keys().map(|&s| s).collect::<Vec<_>>().join("|"))).unwrap();

        let mut first: Option<String> = None;
        let mut last: Option<String> = None;

        let mut set_match = |matched: String| {
            if first.is_none() {
                first = Some(matched.clone());
                last = Some(matched.clone());
            } else {
                last = Some(matched.clone());
            }
        };

        let chars: Vec<char> = s.chars().collect();

        let mut word = String::new();
        for c in chars.iter() {
            if c.is_numeric() {
                word.clear();
                set_match(c.to_string());
            } else {
                word.push(*c);

                for mat in pattern.find_iter(&word) {
                    if let Some(matched) = number_map.get(&mat.as_str()) {
                        set_match(matched.to_string());
                    }
                }
            }
        }

        (first.unwrap(), last.unwrap())
    }

    let mut sum = 0;

    for line in input.lines() {
        let (first, last) = find_numbers(line);
        sum += format!("{}{}", first, last).parse::<i32>().unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test2() {
        let input = include_str!("test2.txt");
        let output = process(input);
        assert_eq!(output, 281);
    }
}
