advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let mut times: Vec<usize> = Vec::new();
    let mut records: Vec<usize> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            let (_, numbers) = line.split_once(":").unwrap();
            for number in numbers.split_whitespace() {
                times.push(number.parse::<usize>().unwrap());
            }
        } else {
            let (_, numbers) = line.split_once(":").unwrap();
            for number in numbers.split_whitespace() {
                records.push(number.parse::<usize>().unwrap());
            }
        }
    }

    println!("Time: {:?}", times);
    println!("Distance: {:?}", records);

    let mut product: usize = 1;
    let mut left: usize;
    let mut right: usize;
    let mut middle: usize;
    let mut middle_distance: usize;
    for (i, time) in times.iter().enumerate() {
        left = 0;
        right = *time / 2;
        middle = right / 2;
        println!("finding time to beat {}", records[i]);
        middle_distance = (time - middle) * middle;
        println!("current middle {} gets {}", middle, middle_distance);
        while middle > left && middle < right && middle_distance != records[i] {
            if middle_distance < records[i] {
                left = middle;
            } else {
                right = middle;
            }
            middle = (right - left) / 2 + left;
            middle_distance = (time - middle) * middle;
            println!("new middle {} gets {}", middle, middle_distance);
        }
        if middle_distance <= records[i] {
            middle += 1;
        }
        println!("answer is {}", middle);
        println!("multiplying product by {}", (time - middle) - middle + 1);
        product *= 1 + ((time - middle) - middle);
    }
    Some(product as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut times: Vec<usize> = Vec::new();
    let mut records: Vec<usize> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        if i == 0 {
            let (_, numbers) = line.split_once(":").unwrap();
            times.push(numbers.replace(' ', "").parse::<usize>().unwrap());
        } else {
            let (_, numbers) = line.split_once(":").unwrap();
            records.push(numbers.replace(' ', "").parse::<usize>().unwrap());
        }
    }

    println!("Time: {:?}", times);
    println!("Distance: {:?}", records);

    let mut product: usize = 1;
    let mut left: usize;
    let mut right: usize;
    let mut middle: usize;
    let mut middle_distance: usize;
    for (i, time) in times.iter().enumerate() {
        left = 0;
        right = *time / 2;
        middle = right / 2;
        println!("finding time to beat {}", records[i]);
        middle_distance = (time - middle) * middle;
        println!("current middle {} gets {}", middle, middle_distance);
        while middle > left && middle < right && middle_distance != records[i] {
            if middle_distance < records[i] {
                left = middle;
            } else {
                right = middle;
            }
            middle = (right - left) / 2 + left;
            middle_distance = (time - middle) * middle;
            println!("new middle {} gets {}", middle, middle_distance);
        }
        if middle_distance <= records[i] {
            middle += 1;
        }
        println!("answer is {}", middle);
        println!("multiplying product by {}", (time - middle) - middle + 1);
        product *= 1 + ((time - middle) - middle);
    }
    Some(product as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
