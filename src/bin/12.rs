use std::collections::HashMap;
advent_of_code::solution!(12);

fn find_combinations(
    pieces: &[usize],
    line: &str,
    mut memo: &mut HashMap<String, Option<usize>>,
) -> Option<usize> {
    let key = format!("{:?}{}",pieces,line);
    let pieces_count = pieces.len();
    if pieces_count == 0 {
        for c in line.chars() {
            if c == '#' {
                memo.insert(key, None);
                return None;
            }
        }
        memo.insert(key.clone(), Some(0));
        return Some(0);
    }
    let mut block_len = pieces[0];
    if block_len > line.len() {
        memo.insert(key.clone(), None);
        return None;
    }
    if let Some(value) = memo.get(&key) {
        return *value;
    }
    let mut prefix_matches = true;
    for (i, c) in line.char_indices() {
        if i == block_len {
            if c != '?' && c != '.' {
                prefix_matches = false;
            }
            block_len += 1;
            break;
        }
        if c != '?' && c != '#' {
            prefix_matches = false;
            break;
        }
    }
    let mut count = 0;
    if prefix_matches {
        if pieces.len() == 1 {
            if let Some(_combos) = find_combinations(&[], &line[block_len..], &mut memo) {
                count = 1;
            }
        } else {
            if let Some(combos) = find_combinations(&pieces[1..], &line[block_len..], &mut memo) {
                count = combos;
            }
        }
    }
    if !line.starts_with("#") {
        if let Some(combos) = find_combinations(&pieces, &line[1..], &mut memo) {
            count += combos;
        }
    }
    if count == 0 {
        memo.insert(key.clone(), None);
        return None;
    }
    memo.insert(key.clone(), Some(count));
    return Some(count);
}

pub fn part_one(input: &str) -> Option<u128> {
    let mut sum: usize = 0;
    for row in input.lines() {
        if let Some((line, piece_str)) = row.split_once(' ') {
            let mut pieces = Vec::new();
            for piece in piece_str.split(',') {
                if let Ok(piece_num) = piece.parse::<usize>() {
                    pieces.push(piece_num);
                }
            }
            if let Some(combinations) = find_combinations(&pieces, line.trim(), &mut HashMap::new())
            {
                // println!("combos: {}", combinations);
                sum += combinations;
            }
        }
    }
    Some(sum as u128)
}

pub fn part_two(input: &str) -> Option<u128> {
    let mut sum: usize = 0;
    let mut hash = HashMap::new();
    for row in input.lines() {
        if let Some((line, piece_str)) = row.split_once(' ') {
            let mut pieces = Vec::new();
            for piece in piece_str.split(',') {
                if let Ok(piece_num) = piece.parse::<usize>() {
                    pieces.push(piece_num);
                }
            }
            let pieces = pieces.repeat(5);
            let unfolded_line = format!("{0}?{0}?{0}?{0}?{0}", line.trim());
            // println!("unfolded: {} {:?}", unfolded_line, pieces);
            if let Some(combinations) = find_combinations(&pieces, &unfolded_line, &mut hash)
            {
                // println!("combos: {}", combinations);
                sum += combinations;
            }
            // println!("asdfasdf: {}", hash.len());
        }
    }
    Some(sum as u128)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
