use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let pattern = Regex::new(r#"(one|two|three|four|five|six|seven|eight|nine)"#).unwrap();

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

    let find_number = |s: &str, reverse: bool| -> Option<String> {
        let mut result: Option<String> = None;
        let mut word = String::new();

        let chars: Vec<char> = if reverse {
            s.chars().rev().collect()
        } else {
            s.chars().collect()
        };

        for c in chars.iter() {
            if c.is_numeric() {
                result = Some(c.to_string());
            } else {
                word = if reverse {
                    format!("{}{}", c, word)
                } else {
                    format!("{}{}", word, c)
                };

                for mat in pattern.find_iter(&word) {
                    if let Some(matched) = number_map.get(&mat.as_str()) {
                        result = Some(matched.to_string());
                    }
                }
            }

            if result.is_some() {
                break;
            }
        }

        result
    };

    let mut sum = 0;

    for line in input.lines() {
        let first = find_number(line, false);
        let last = find_number(line, true);

        if first.is_some() && last.is_some() {
            let first = first.unwrap();
            let last = last.unwrap();
            println!("{} {} {}", line, first, last);
            sum += format!("{}{}", first, last).parse::<i32>().unwrap();
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
        assert_eq!(output, 281);
    }
}
