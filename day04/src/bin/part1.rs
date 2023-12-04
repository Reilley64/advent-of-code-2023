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

        let winning_numbers: Vec<&str> = winning_numbers_string.split_whitespace().collect();
        let numbers: Vec<&str> = numbers_string.split_whitespace().collect();

        let mut points = 0;

        for number in numbers {
            if winning_numbers.contains(&number) {
                if points == 0 {
                    points = 1;
                } else {
                    points *= 2;
                }
            }
        }

        sum += points;
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
