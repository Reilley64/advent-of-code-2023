use std::cmp;
use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let mut sum = 0;
    for game in input.lines() {
        let mut cube_map: HashMap<&str, i32> = HashMap::new();

        let (_identifier, data) = game.split_once(": ").unwrap();

        let sets: Vec<&str> = data.split("; ").collect();

        for set in sets {
            let cubes: Vec<&str> = set.split(", ").collect();

            for cube in cubes {
                let (size, colour) = cube.split_once(" ").unwrap();
                *cube_map
                    .entry(colour)
                    .and_modify(|e| *e = cmp::max(*e, size.parse::<i32>().unwrap()))
                    .or_insert_with(|| size.parse::<i32>().unwrap());
            }
        }

        sum += cube_map.get("red").unwrap() * cube_map.get("green").unwrap() * cube_map.get("blue").unwrap();
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
        assert_eq!(output, 2286);
    }
}
