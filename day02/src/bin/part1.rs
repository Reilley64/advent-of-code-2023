use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let cube_map = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let mut sum = 0;
    for game in input.lines() {
        let (identifier, data) = game.split_once(": ").unwrap();

        let id = &identifier[5..identifier.len()].parse::<i32>().unwrap();
        let sets: Vec<&str> = data.split("; ").collect();

        if !sets.iter().any(|set| {
            let cubes: Vec<&str> = set.split(", ").collect();
            return cubes.iter().any(|cube| {
                let (size, colour) = cube.split_once(" ").unwrap();
                return size.parse::<i32>().unwrap() > *cube_map.get(colour).unwrap();
            });
        }) {
            sum += id;
        }
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
        assert_eq!(output, 8);
    }
}
