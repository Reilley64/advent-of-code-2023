fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i64 {
    let mut maps  = input.split("\n\n");

    let seeds_string = maps.next().unwrap();
    let seed_numbers: Vec<i64> = seeds_string.split_whitespace().skip(1).map(|x| x.parse().unwrap()).collect();
    let mut seed_ranges: Vec<(i64, i64)> = seed_numbers.chunks(2).map(|chunk| (chunk[0], chunk[0] + chunk[1])).collect();

    for map in maps {
        let map_ranges: Vec<(i64, i64, i64)> = map
            .lines()
            .skip(1)
            .map(|line| {
                let mut iter = line.split_whitespace().map(|x| x.parse().unwrap());
                (
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                    iter.next().unwrap(),
                )
            })
            .collect();

        let mut new: Vec<(i64, i64)> = Vec::new();
        while let Some((seed_range_start, seed_range_end)) = seed_ranges.pop() {
            let mut seed_added = false;
            for (destination_range_start, source_range_start, range) in &map_ranges {
                let overlap_start = seed_range_start.max(*source_range_start);
                let overlap_end = seed_range_end.min(*source_range_start + *range);

                if overlap_start < overlap_end {
                    new.push((overlap_start - *source_range_start + *destination_range_start, overlap_end - *source_range_start + *destination_range_start));
                    if overlap_start > seed_range_start {
                        seed_ranges.push((seed_range_start, overlap_start));
                    }
                    if seed_range_end > overlap_end {
                        seed_ranges.push((overlap_end, seed_range_end));
                    }
                    seed_added = true;
                    break;
                }
            }
            if !seed_added {
                new.push((seed_range_start, seed_range_end));
            }
        }

        seed_ranges = new;
    }

    seed_ranges.iter().map(|&(x, _)| x).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = include_str!("./test.txt");
        let output = process(input);
        assert_eq!(output, 46);
    }
}
