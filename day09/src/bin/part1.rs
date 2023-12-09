fn main() {
    let input = include_str!("input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let mut sum = 0;

    for line in input.lines() {
        let mut sequences: Vec<Vec<i32>> = vec![line.split_whitespace().map(|x| x.parse().unwrap()).collect()];

        while sequences.last().unwrap().iter().any(|&x| x != 0) {
            let mut sequence: Vec<i32> = Vec::new();

            for (index, number) in sequences.last().unwrap().iter().enumerate().skip(1) {
                sequence.push(number - sequences.last().unwrap()[index - 1]);
            }

            sequences.push(sequence);
        }

        let mut last_number = 0;
        for sequence in sequences.iter_mut().rev() {
            sequence.push(sequence.last().unwrap() + last_number);
            last_number = *sequence.last().unwrap();
        }

        sum += sequences.first().unwrap().last().unwrap();
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("test.txt");
        let output = process(input);
        assert_eq!(output, 114);
    }
}
