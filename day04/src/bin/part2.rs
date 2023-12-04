use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    fn get_matches(winning_numbers: &Vec<&str>, numbers: &Vec<&str>) -> i32 {
        numbers.iter().filter(|&&number| winning_numbers.contains(&number)).count() as i32
    }

    let mut game_cards = HashMap::new();

    for line in input.lines() {
        let mut parts = line.split(":");
        let identifier = parts.next().unwrap();
        let data = parts.next().unwrap();

        let mut identifier_parts = identifier.split_whitespace();
        let _ = identifier_parts.next().unwrap();
        let id = identifier_parts.next().unwrap();

        let mut data_parts = data.split("|");
        let winning_numbers_string = data_parts.next().unwrap();
        let numbers_string = data_parts.next().unwrap();

        let winning_numbers: Vec<&str> = winning_numbers_string.split_whitespace().collect();
        let numbers: Vec<&str> = numbers_string.split_whitespace().collect();

        if !game_cards.contains_key(&id.to_string()) {
            game_cards.insert(id.to_string(), 1);
        }

        let matches = get_matches(&winning_numbers, &numbers);

        for i in 0..matches {
            let id_int: i32 = id.parse().unwrap();
            let game_id_int: i32 = id_int + i + 1;
            let game_id = game_id_int.to_string();

            *game_cards.entry(game_id).or_insert(1) += *game_cards.get(&id.to_string()).unwrap();
        }
    }

    let mut sum = 0;

    for (_id, cards) in game_cards {
        sum += cards;
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
        assert_eq!(output, 30);
    }
}
