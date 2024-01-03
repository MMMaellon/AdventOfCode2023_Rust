
advent_of_code::solution!(3);
pub fn part_one(input: &str) -> Option<u32> {
    let mut prev_nums: Vec<(usize, u32, usize)> = Vec::new();
    let mut prev_symbols: Vec<usize> = Vec::new();
    let mut symbols: Vec<usize> = Vec::new();
    let mut nums: Vec<(usize, u32, usize)> = Vec::new();
    let mut word: String = String::new();
    let (mut index, mut word_num, mut length): (usize, u32, usize);
    let mut should_add: bool;
    let mut sum: u32 = 0;
    fn complete_word(
        word_num: &u32,
        should_add: &mut bool,
        prev_symbols: &Vec<usize>,
        prev_nums: &Vec<(usize, u32, usize)>,
        symbols: &mut Vec<usize>,
        nums: &mut Vec<(usize, u32, usize)>,
        index: &usize,
        length: &usize,
        sum: &mut u32,
    ) {
        let _ = symbols;
        let _ = prev_nums;
        if *length == 0 {
            return;
        }

        if !*should_add {
            for &prev_symbol in prev_symbols {
                if prev_symbol + 1 >= *index && prev_symbol <= *index + length {
                    *should_add = true;
                    break;
                }
            }
        }
        if *should_add {
            *sum += word_num;
        } else {
            nums.push((*index, *word_num, *length));
        }
        *should_add = false;
    }
    for line in input.lines() {
        word.clear();
        index = 0;
        should_add = false;
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if word.len() == 0 {
                    index = i;
                }
                word.push(c);
            } else {
                if word.len() > 0 {
                    word_num = word.parse().unwrap();
                    length = word.len();
                } else {
                    word_num = 0;
                    length = 0;
                }
                match c {
                    '.' => {
                        complete_word(
                            &word_num,
                            &mut should_add,
                            &prev_symbols,
                            &prev_nums,
                            &mut symbols,
                            &mut nums,
                            &index,
                            &length,
                            &mut sum,
                        );
                        should_add = false;
                    }
                    _ => {
                        complete_word(
                            &word_num,
                            &mut true,
                            &prev_symbols,
                            &prev_nums,
                            &mut symbols,
                            &mut nums,
                            &index,
                            &length,
                            &mut sum,
                        );
                        symbols.push(i);
                        should_add = true;

                        for &mut (prev_index, prev_value, prev_length) in &mut prev_nums {
                            if i + 1 >= prev_index && i <= prev_index + prev_length {
                                sum += prev_value;
                                // prev_value = 0;
                            }
                        }
                    }
                }
                word.clear();
            }
        }
        if word.len() > 0 {
            complete_word(
                &word.parse().unwrap(),
                &mut should_add,
                &prev_symbols,
                &prev_nums,
                &mut symbols,
                &mut nums,
                &index,
                &word.len(),
                &mut sum,
            );
            word.clear();
        }
        prev_symbols = symbols;
        prev_nums = nums;
        symbols = Vec::new();
        nums = Vec::new();
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut prev_nums: Vec<(usize, u32, usize)> = Vec::new();
    let mut prev_gears: Vec<(usize, usize, u32)> = Vec::new();
    let mut gears: Vec<(usize, usize, u32)> = Vec::new();
    let mut nums: Vec<(usize, u32, usize)> = Vec::new();
    let mut word: String = String::new();
    let (mut index, mut word_num, mut length): (usize, u32, usize);
    let mut sum: u32 = 0;
    fn check_overlap(gears: &mut Vec<(usize, usize, u32)>, nums: &Vec<(usize, u32, usize)>) {
        for &mut (index, ref mut _num_count, ref mut _ratio) in gears.iter_mut() {
            for &(num_index, num_value, num_len) in nums {
                if index + 1 >= num_index && index <= num_index + num_len {
                    println!("added number to gear {}", num_value);
                    *_num_count += 1;
                    *_ratio *= num_value;
                }
            }
        }
    }
    for line in input.lines() {
        word.clear();
        index = 0;
        for (i, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if word.len() == 0 {
                    index = i;
                }
                word.push(c);
            } else {
                if word.len() > 0 {
                    word_num = word.parse().unwrap();
                    length = word.len();
                    nums.push((index, word_num, length));
                }
                match c {
                    '*' => {
                        gears.push((i, 0, 1));
                    }
                    _ => {}
                }
                word.clear();
            }
        }
        if word.len() > 0 {
            word_num = word.parse().unwrap();
            length = word.len();
            nums.push((index, word_num, length));
            word.clear();
        }
        check_overlap(&mut prev_gears, &nums);
        check_overlap(&mut gears, &prev_nums);
        check_overlap(&mut prev_gears, &prev_nums);
        println!("prev_gears {:?}", gears);
        // println!("nums {:?}", nums);

        for &(_, count, ratio) in &prev_gears {
            if count == 2 {
                println!("gear with 2 nums {}", ratio);
                sum += ratio;
            }
        }

        prev_gears = gears;
        prev_nums = nums;
        gears = Vec::new();
        nums = Vec::new();
    }

    Some(sum)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
