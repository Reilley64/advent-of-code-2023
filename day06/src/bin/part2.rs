fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i64 {
    let mut parts = input.split("\n");
    let time_str = parts.next().unwrap();
    let time: i64 = time_str.split_whitespace().skip(1).fold(String::new(), |acc, s| acc + s).parse().unwrap();
    let distance_str = parts.next().unwrap();
    let distance: i64 = distance_str.split_whitespace().skip(1).fold(String::new(), |acc, s| acc + s).parse().unwrap();

    let mut sum = 0;

    for i in 1..time {
        if i > time / 2 {
            if time % 2 == 0 {
                sum = sum * 2 - 1;
            } else {
                sum *= 2;
            }
            break;
        }

        if (distance /i) + i < time {
            sum += 1;
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
        assert_eq!(output, 71503);
    }
}
