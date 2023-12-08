advent_of_code::solution!(4);
struct Card {
    id: u32,
    winning: Vec<u32>,
    nums: Vec<u32>,
    points: u32,
}

impl Card {
    fn new(id: u32) -> Self {
        Self {
            id,
            winning: Vec::new(),
            nums: Vec::new(),
            points: 0,
        }
    }
    fn add_winning(&mut self, num: u32) {
        self.winning.push(num);
    }
    fn add_num_sum(&mut self, num: u32) {
        if self.check_win(num) {
            self.points += 1;
        }
        self.nums.push(num);
    }
    fn add_num(&mut self, num: u32) {
        if self.check_win(num) {
            if 0 == self.points {
                self.points = 1;
            } else {
                self.points *= 2;
            }
        }
        self.nums.push(num);
    }
    fn check_win(&self, num: u32) -> bool {
        for &winning_num in &self.winning {
            if winning_num == num {
                return true;
            }
        }
        return false;
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut cards: Vec<Card> = Vec::new();
    let (mut id_str, mut value_str): (&str, &str);
    let (mut winning_str, mut nums_str): (&str, &str);
    let mut sum = 0;
    for line in input.lines() {
        (id_str, value_str) = line.split_once(": ").unwrap();
        (_, id_str) = id_str.split_once(" ").unwrap();
        let mut card = Card::new(id_str.trim().parse::<u32>().unwrap());
        (winning_str, nums_str) = value_str.split_once(" | ").unwrap();
        for num_str in winning_str.split_whitespace() {
            card.add_winning(num_str.parse::<u32>().unwrap())
        }
        for num_str in nums_str.split_whitespace() {
            card.add_num(num_str.parse::<u32>().unwrap())
        }
        sum += card.points;
        cards.push(card);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut cards: Vec<Card> = Vec::new();
    let (mut id_str, mut value_str): (&str, &str);
    let (mut winning_str, mut nums_str): (&str, &str);
    let mut sum = 0;
    for line in input.lines() {
        (id_str, value_str) = line.split_once(": ").unwrap();
        (_, id_str) = id_str.split_once(" ").unwrap();
        let mut card = Card::new(id_str.trim().parse::<u32>().unwrap());
        (winning_str, nums_str) = value_str.split_once(" | ").unwrap();
        for num_str in winning_str.split_whitespace() {
            card.add_winning(num_str.parse::<u32>().unwrap())
        }
        for num_str in nums_str.split_whitespace() {
            card.add_num_sum(num_str.parse::<u32>().unwrap())
        }
        cards.push(card);
    }
    let mut multiplier: Vec<u32> = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let multi = multiplier[i];
        for j in 0..(card.points as usize) {
            if let Some(value) = multiplier.get(i + j + 1) {
                multiplier[i + j + 1] = *value + multi;
            }
        }
        sum += multi;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
