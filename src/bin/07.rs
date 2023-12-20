use std::cmp::Ordering;

advent_of_code::solution!(7);
#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Clone, Copy, PartialEq, PartialOrd)]
enum CardType {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
    Joker = 1,
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut hands: Vec<(HandType, [CardType; 5], usize)> = Vec::new();
    for line in input.lines() {
        if let Some((hand, bid)) = line.split_once(" ") {
            let mut hand_type = HandType::HighCard;

            let mut found_three_match = false;
            let mut found_two_match = false;
            let mut two_match = ' ';
            for card in hand.chars() {
                let match_count = hand.matches(card);
                match match_count.count() {
                    5 => {
                        hand_type = HandType::FiveOfAKind;
                        break;
                    }
                    4 => {
                        hand_type = HandType::FourOfAKind;
                        break;
                    }
                    3 => {
                        if found_three_match {
                            continue;
                        } else if found_two_match {
                            hand_type = HandType::FullHouse;
                            break;
                        } else {
                            found_three_match = true;
                        }
                    }
                    2 => {
                        if found_three_match {
                            hand_type = HandType::FullHouse;
                        } else if found_two_match {
                            if two_match != card {
                                hand_type = HandType::TwoPair;
                                break;
                            } else {
                                continue;
                            }
                        } else {
                            found_two_match = true;
                            two_match = card;
                        }
                    }
                    _ => {}
                }
            }

            if hand_type == HandType::HighCard {
                if found_three_match {
                    hand_type = HandType::ThreeOfAKind;
                } else if found_two_match {
                    hand_type = HandType::OnePair;
                }
            }

            if let Ok(amount) = bid.trim().parse::<usize>() {
                let mut card_types = [CardType::Two; 5];
                for (i, card) in hand.chars().enumerate() {
                    match card {
                        'A' => card_types[i] = CardType::Ace,
                        'K' => card_types[i] = CardType::King,
                        'Q' => card_types[i] = CardType::Queen,
                        'J' => card_types[i] = CardType::Jack,
                        'T' => card_types[i] = CardType::Ten,
                        '9' => card_types[i] = CardType::Nine,
                        '8' => card_types[i] = CardType::Eight,
                        '7' => card_types[i] = CardType::Seven,
                        '6' => card_types[i] = CardType::Six,
                        '5' => card_types[i] = CardType::Five,
                        '4' => card_types[i] = CardType::Four,
                        '3' => card_types[i] = CardType::Three,
                        '2' => card_types[i] = CardType::Two,
                        _ => {}
                    }
                }
                hands.push((hand_type, card_types, amount));
            }
        }
    }

    hands.sort_by(|(type_a, hand_a, _), (type_b, hand_b, _)| {
        let mut order = Ordering::Equal;
        if type_a > type_b {
            order = Ordering::Greater;
        } else if type_a < type_b {
            order = Ordering::Less;
        } else {
            for i in 0..hand_a.len() {
                if hand_a[i] > hand_b[i] {
                    order = Ordering::Greater;
                    break;
                } else if hand_a[i] < hand_b[i] {
                    order = Ordering::Less;
                    break;
                }
            }
        }
        return order;
    });
    let mut sum = 0;
    for (i, (_, _, bid)) in hands.iter().enumerate() {
        sum += bid * (i + 1);
    }

    Some(sum as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut hands: Vec<(HandType, [CardType; 5], usize)> = Vec::new();
    for line in input.lines() {
        if let Some((hand, bid)) = line.split_once(" ") {
            let mut hand_type = HandType::HighCard;

            let mut found_three_match = false;
            let mut found_two_match = false;
            let mut last_match = ' ';
            let jokers = hand.matches('J').count();
            for card in hand.chars() {
                if card == 'J' {
                    continue;
                }
                let match_count = hand.matches(card);
                match match_count.count() {
                    5 => {
                        hand_type = HandType::FiveOfAKind;
                        break;
                    }
                    4 => {
                        if jokers == 0 {
                            hand_type = HandType::FourOfAKind;
                        } else {
                            hand_type = HandType::FiveOfAKind;
                        }
                        break;
                    }
                    3 => {
                        if last_match == card {
                            continue;
                        } else if found_two_match {
                            if jokers == 0 {
                                hand_type = HandType::FullHouse;
                            } else {
                                hand_type = HandType::FiveOfAKind;
                            }
                            break;
                        } else if jokers == 2 {
                            hand_type = HandType::FiveOfAKind;
                        } else if jokers == 1 {
                            hand_type = HandType::FourOfAKind;
                        } else {
                            found_three_match = true;
                        }
                    }
                    2 => {
                        if last_match == card {
                            continue;
                        }
                        if found_three_match {
                            hand_type = HandType::FullHouse;
                        } else if found_two_match {
                            if last_match != card {
                                if jokers == 0 {
                                    hand_type = HandType::TwoPair;
                                } else {
                                    hand_type = HandType::FullHouse;
                                }
                                break;
                            }
                        } else if jokers == 0 || jokers == 1 {
                            found_two_match = true;
                            last_match = card;
                        } else if jokers == 3 {
                            hand_type = HandType::FiveOfAKind;
                            break;
                        } else if jokers == 2 {
                            hand_type = HandType::FourOfAKind;
                            break;
                        }
                    }
                    _ => {}
                }
            }

            if hand_type == HandType::HighCard {
                if found_three_match {
                    hand_type = HandType::ThreeOfAKind;
                } else if found_two_match {
                    if jokers == 0 {
                        hand_type = HandType::OnePair;
                    } else {
                        hand_type = HandType::ThreeOfAKind;
                    }
                } else if jokers == 5 || jokers == 4 {
                    hand_type = HandType::FiveOfAKind;
                } else if jokers == 3 {
                    hand_type = HandType::FourOfAKind;
                } else if jokers == 2 {
                    hand_type = HandType::ThreeOfAKind;
                } else if jokers == 1 {
                    hand_type = HandType::OnePair;
                }
            }

            if let Ok(amount) = bid.trim().parse::<usize>() {
                let mut card_types = [CardType::Two; 5];
                for (i, card) in hand.chars().enumerate() {
                    match card {
                        'A' => card_types[i] = CardType::Ace,
                        'K' => card_types[i] = CardType::King,
                        'Q' => card_types[i] = CardType::Queen,
                        'J' => card_types[i] = CardType::Joker,
                        'T' => card_types[i] = CardType::Ten,
                        '9' => card_types[i] = CardType::Nine,
                        '8' => card_types[i] = CardType::Eight,
                        '7' => card_types[i] = CardType::Seven,
                        '6' => card_types[i] = CardType::Six,
                        '5' => card_types[i] = CardType::Five,
                        '4' => card_types[i] = CardType::Four,
                        '3' => card_types[i] = CardType::Three,
                        '2' => card_types[i] = CardType::Two,
                        _ => {}
                    }
                }
                hands.push((hand_type, card_types, amount));
            }
        }
    }

    hands.sort_by(|(type_a, hand_a, _), (type_b, hand_b, _)| {
        let mut order = Ordering::Equal;
        if type_a > type_b {
            order = Ordering::Greater;
        } else if type_a < type_b {
            order = Ordering::Less;
        } else {
            for i in 0..hand_a.len() {
                if hand_a[i] > hand_b[i] {
                    order = Ordering::Greater;
                    break;
                } else if hand_a[i] < hand_b[i] {
                    order = Ordering::Less;
                    break;
                }
            }
        }
        return order;
    });
    let mut sum = 0;
    for (i, (_, _, bid)) in hands.iter().enumerate() {
        sum += bid * (i + 1);
    }

    Some(sum as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
