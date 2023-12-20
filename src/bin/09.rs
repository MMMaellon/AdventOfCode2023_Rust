advent_of_code::solution!(9);
fn parse_input(input: &str) -> Vec<Vec<Vec<i64>>> {
    let mut readings: Vec<Vec<Vec<i64>>> = Vec::new();
    for line in input.lines() {
        let mut start = Vec::new();
        for number in line.split_whitespace() {
            start.push(number.parse::<i64>().unwrap());
        }
        readings.push(vec![start]);
    }
    return readings;
}
fn is_all_zeroes(iteration: &Vec<i64>) -> bool {
    for number in iteration.into_iter() {
        if *number != 0 {
            return false;
        }
    }
    return true;
}

fn iterate(reading: &mut Vec<Vec<i64>>) -> bool {
    if reading.len() == 0 {
        return false;
    }
    let last_row = &reading[reading.len() - 1];
    if is_all_zeroes(last_row) {
        return false;
    }

    let mut next_iteration: Vec<i64> = Vec::new();
    let mut last_number = 0;
    for (i, number) in last_row.into_iter().enumerate() {
        if i == 0 {
            last_number = *number;
            continue;
        }
        next_iteration.push(*number - last_number);
        last_number = *number;
    }
    reading.push(next_iteration);
    return true;
}

fn iterate_until_done(reading: &mut Vec<Vec<i64>>) {
    while iterate(reading) {}
}

fn predict_next(reading: &Vec<Vec<i64>>) -> i64 {
    let mut next = 0;
    for iteration in reading.iter().rev() {
        next += iteration[iteration.len() - 1]
    }
    return next;
}
fn predict_prev(reading: &Vec<Vec<i64>>) -> i64 {
    let mut next = 0;
    for iteration in reading.iter().rev() {
        next = iteration[0] - next;
    }
    return next;
}
pub fn part_one(input: &str) -> Option<i64> {
    let mut readings = parse_input(input);
    let mut sum = 0;
    for reading in readings.iter_mut() {
        iterate_until_done(reading);
        sum += predict_next(reading);
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut readings = parse_input(input);
    let mut sum = 0;
    for reading in readings.iter_mut() {
        iterate_until_done(reading);
        sum += predict_prev(reading);
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
