advent_of_code::solution!(14);
fn parse_input(input: &str) ->Vec<Vec<char>>{
    let mut char_array : Vec<Vec<char>>= Vec::new();
    for line in input.lines(){
        char_array.push(line.chars().collect());
    }
    return char_array;
}
fn move_north(rocks : &Vec<Vec<char>>) -> usize{
    if rocks.is_empty() || rocks[0].is_empty(){
        return 0;
    }
    let mut last_blocker = vec![0; rocks[0].len()];
    let mut sum = 0;
    let row_count = rocks.len();
    for (i,row) in rocks.into_iter().enumerate(){
        for (j, c) in row.into_iter().enumerate(){
            match c{
                '#' => {
                    last_blocker[j] = i + 1;
                }
                'O' => {
                    let weight = ((row_count as i32) - (last_blocker[j] as i32)) as usize;
                    sum += weight;
                    last_blocker[j] += 1;
                }
                _ => {}
            }
        }
    }
    return sum;
}
pub fn part_one(input: &str) -> Option<u32> {
    let char_array = parse_input(input);
    let north = move_north(&char_array);
    Some(north as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
