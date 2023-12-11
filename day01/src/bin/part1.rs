fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    fn find_numbers(s: &str) -> (Option<String>, Option<String>) {
        let mut first: Option<String> = None;
        let mut last: Option<String> = None;

        let chars: Vec<char> = s.chars().collect();

        for c in chars.iter() {
            if c.is_numeric() {
                if first.is_none() {
                    first = Some(c.to_string().clone());
                    last = Some(c.to_string().clone());
                } else {
                    last = Some(c.to_string().clone());
                }
            }
        }

        (first, last)
    }

    let mut sum = 0;

    for line in input.lines() {
        let (first, last) = find_numbers(line);

        if first.is_some() {
            sum += format!("{}{}", first.unwrap(), last.unwrap()).parse::<i32>().unwrap();
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let input = include_str!("test1.txt");
        let output = process(input);
        assert_eq!(output, 142);
    }
}
