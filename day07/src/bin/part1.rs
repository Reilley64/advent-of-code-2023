use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = process(input);
    dbg!(output);
}

fn process(input: &str) -> i32 {
    let hands: Vec<(Vec<char>, i32)> = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let hand = parts.next().unwrap().chars().collect();
            let bid = parts.next().unwrap().parse().unwrap();
            (hand, bid)
        })
        .collect();

    let mut hand_scores: Vec<(usize, i32)> = Vec::new();
    for (index, (hand, _bid)) in hands.iter().enumerate() {
        let mut sorted_hand = hand.clone();
        sorted_hand.sort_by(|&a: &char, &b: &char| {
            let order = "23456789TJQKA";
            order.find(a).cmp(&order.find(b))
        });

        let unique_chars: HashSet<char> = sorted_hand.clone().into_iter().collect();
        let mut unique_score = 0;
        'main: for unique_char in unique_chars {
            let unique_char_index = sorted_hand.iter().position(|&x| x == unique_char).unwrap();

            if let Some(end_char) = sorted_hand.get(unique_char_index + 4) {
                if *end_char == unique_char {
                    unique_score = 6;
                    break;
                }
            }

            if let Some(end_char) = sorted_hand.get(unique_char_index + 3) {
                if *end_char == unique_char {
                    unique_score = 5;
                    break;
                }
            }

            if let Some(end_char) = sorted_hand.get(unique_char_index + 2) {
                if *end_char == unique_char {
                    let mut remaining = sorted_hand.clone();
                    remaining.drain(unique_char_index..unique_char_index+3);

                    if remaining.iter().all(|&x| x == remaining[0]) {
                        unique_score = 4;
                        break;
                    }

                    unique_score = 3;
                    break;
                }
            }

            if let Some(end_char) = sorted_hand.get(unique_char_index + 1) {
                if *end_char == unique_char {
                    let mut remaining = sorted_hand.clone();
                    remaining.drain(unique_char_index..unique_char_index+2);

                    for other_char in &remaining {
                        let other_char_index = remaining.iter().position(|&x| x == *other_char).unwrap();

                        if let Some(other_end_char) = remaining.get(other_char_index + 1) {
                            if *other_end_char == *other_char {
                                unique_score = unique_score.max(2);
                                continue 'main;
                            }
                        }
                    }

                    unique_score = unique_score.max(1);
                    continue 'main;
                }
            }
        }

        hand_scores.push((index, unique_score));
    }

    hand_scores.sort_by(|a, b| {
        if a.1 != b.1 {
            a.1.cmp(&b.1)
        } else {
            let a_hand = hands[a.0].0.clone();
            let b_hand = hands[b.0].0.clone();

            for (a_char, b_char) in a_hand.iter().zip(b_hand.iter()) {
                let order = "23456789TJQKA";

                if a_char != b_char {
                    let a_val = order.find(*a_char).unwrap_or_else(|| order.len());
                    let b_val = order.find(*b_char).unwrap_or_else(|| order.len());

                    return a_val.cmp(&b_val);
                }
            }

            a.1.cmp(&b.1)
        }
    });

    let mut sum = 0;

    for (index, (bid_index, _unique_score)) in hand_scores.iter().enumerate() {
        sum += hands[*bid_index].1 * (index + 1) as i32;
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
        assert_eq!(output, 5905);
    }
}
