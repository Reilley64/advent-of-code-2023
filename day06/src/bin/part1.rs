fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i64 {
    let mut parts = input.split("\n");
    let time_str = parts.next().unwrap();
    let time_values: Vec<i32> = time_str.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
    let distance_str = parts.next().unwrap();
    let distance_values: Vec<i32> = distance_str.split_whitespace().skip(1).map(|s| s.parse().unwrap()).collect();
    let time_distance: Vec<(i32, i32)> = time_values.iter().zip(distance_values.iter()).map(|(&t, &d)| (t, d)).collect();

    let mut sum = 1;

    for (time, distance) in time_distance {
        let mut beat = 0;

        for i in 1..time {
            if i > time / 2 {
                if time % 2 == 0 {
                    beat = beat * 2 - 1;
                } else {
                    beat *= 2;
                }
                break;
            }

            if (distance / i) + i < time {
                beat += 1;
            }
        }

        println!("{}", beat);

        sum *= beat;
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
        assert_eq!(output, 288);
    }
}
