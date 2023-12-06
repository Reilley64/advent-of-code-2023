fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i64 {
    let mut maps  = input.split("\n\n");

    let seeds_string = maps.next().unwrap();
    let mut seeds: Vec<i64> = seeds_string.split_whitespace().skip(1).map(|x| x.parse().unwrap()).collect();

    while let Some(map_string) = maps.next() {
        let map_lines: Vec<Vec<i64>> = map_string
            .lines()
            .skip(1)
            .map(|line| {
                line.split_whitespace()
                    .map(|s| s.parse().unwrap())
                    .collect()
            })
            .collect();

        for seed in seeds.iter_mut() {
            'outer: for map_numbers in map_lines.iter() {
                let destination_range_start = map_numbers[0];
                let source_range_start = map_numbers[1];
                let range_length = map_numbers[2];

                if source_range_start <= *seed && *seed < source_range_start + range_length {
                    *seed = destination_range_start + (*seed - source_range_start);
                    break 'outer;
                }
            }
        }
    }

    seeds.iter().min().unwrap().clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("./test.txt");
        let output = process(input);
        assert_eq!(output, 35);
    }
}
