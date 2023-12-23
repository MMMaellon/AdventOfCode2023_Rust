use std::collections::{HashMap, HashSet};

advent_of_code::solution!(13);

fn parse_input(input: &str) -> Vec<Vec<Vec<char>>> {
    let mut patterns = Vec::new();
    let mut pattern = Vec::new();
    for line in input.lines() {
        if line.is_empty() {
            if !pattern.is_empty() {
                patterns.push(pattern.clone());
                pattern.clear();
            }
            continue;
        }
        pattern.push(line.chars().collect());
    }
    if !pattern.is_empty() {
        patterns.push(pattern.clone());
    }
    return patterns;
}

fn extract_features(patterns: Vec<Vec<Vec<char>>>) -> Vec<Vec<Vec<usize>>> {
    let mut features = Vec::new();
    for pattern in patterns.into_iter() {
        let mut pattern_vec = Vec::new();
        for line in pattern.into_iter() {
            let mut line_vec = Vec::new();
            for (i, c) in line.into_iter().enumerate() {
                if c == '#' {
                    line_vec.push(i);
                }
            }
            pattern_vec.push(line_vec);
        }
        features.push(pattern_vec);
    }

    return features;
}

fn generate_possible_centers(input: &Vec<usize>) -> Vec<usize> {
    let mut possible_centers: Vec<usize> = Vec::new();
    if input.len() < 2 {
        return possible_centers;
    }
    for i in 1..input.len() {
        possible_centers.push((input[i] + input[i - 1]) / 2);
    }
    return possible_centers;
}
fn detect_reflection(feature_set: &Vec<Vec<usize>>) -> Option<usize> {
    if feature_set.is_empty() {
        return None;
    }
    for i in 0..(feature_set.len() - 1) {
        if feature_set[i] == feature_set[i + 1] {
            let mut range_matches = true;
            for j in 0..i {
                let distance = (i as i32) - (j as i32);
                let mirror = ((i as i32) + distance + 1) as usize;
                if mirror >= feature_set.len() {
                    continue;
                }
                if feature_set[j] != feature_set[mirror] {
                    range_matches = false;
                    break;
                }
            }

            if range_matches {
                return Some(i);
            }
        }
    }
    return None;
}
fn detect_imperfect_reflection(feature_set: &Vec<Vec<usize>>) -> Option<usize> {
    if feature_set.is_empty() {
        return None;
    }
    for possibility in 0..(feature_set.len() - 1) {
        // println!("possibility: {}", possibility);
        let mut mismatches = 0;
        for i in 0..(possibility + 1) {
            let distance = (possibility as i32) - (i as i32);
            let mirror = ((possibility as i32) + distance + 1) as usize;
            if mirror >= feature_set.len() {
                continue;
            }
            // println!(
                // "comparing {:?} with {:?}",
                // feature_set[i], feature_set[mirror]
            // );
            if feature_set[i] != feature_set[mirror] {
                if mismatches > 0 {
                    mismatches = 1001;
                    break;
                }

                for f in feature_set[i].iter() {
                    if !feature_set[mirror].contains(f) {
                        mismatches += 1;
                        if mismatches > 1 {
                            break;
                        }
                    }
                }
                // println!("mismatches: {}", mismatches);
                if mismatches == 0 && feature_set[i].len() + 1 == feature_set[mirror].len(){
                    mismatches += 1;
                } else if mismatches == 1 && feature_set[mirror].len() + 1 == feature_set[i].len(){
                    continue;
                } else {
                    mismatches = 1001;
                    break;
                }
            }
        }
        // println!("mismatches: {}", mismatches);
        if mismatches == 1 {
            return Some(possibility);
        }
    }
    return None;
}

fn fix_smudge(feature_set: &mut Vec<Vec<usize>>) -> bool {
    if feature_set.is_empty() {
        return false;
    }
    for possibility in 0..(feature_set.len() - 1) {
        // println!("possibility: {}", possibility);
        let mut mismatches = 0;
        let mut last_i = 0;
        let mut last_mirror = 0;
        for i in 0..(possibility + 1) {
            let distance = (possibility as i32) - (i as i32);
            let mirror = ((possibility as i32) + distance + 1) as usize;
            if mirror >= feature_set.len() {
                continue;
            }
            // println!("comparing {:?} with {:?}", feature_set[i], feature_set[mirror]);
            if feature_set[i] != feature_set[mirror] {
                if mismatches > 0 {
                    mismatches = 1001;
                    break;
                }

                for f in feature_set[i].iter() {
                    if !feature_set[mirror].contains(f) {
                        mismatches += 1;
                        last_i = i;
                        last_mirror = mirror;
                        if mismatches > 1 {
                            break;
                        }
                    }
                }
                if feature_set[mirror].len() > feature_set[i].len() {
                    last_i = i;
                    last_mirror = mirror;
                    mismatches += feature_set[mirror].len() - feature_set[i].len();
                }
            }
        }
        // println!("mismatches: {}", mismatches);
        if mismatches == 1 {
            feature_set.insert(last_i, feature_set[last_mirror].clone());
            return true;
        }
    }
    return true;
}
fn transpose(input: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let width = input.len();
    if width == 0 {
        return Vec::new();
    }
    let height = input[0].len();
    if height == 0 {
        return Vec::new();
    }
    let mut output = Vec::with_capacity(height);
    for i in 0..height {
        let mut line = Vec::new();
        for j in 0..width {
            line.push(input[j][i]);
        }
        output.push(line);
    }

    return output;
}

pub fn part_one(input: &str) -> Option<u32> {
    let patterns = parse_input(input);
    let mut transposed_patterns = Vec::new();
    for p in patterns.iter() {
        transposed_patterns.push(transpose(&p));
    }
    let features = extract_features(patterns);
    let transposed_features = extract_features(transposed_patterns);

    let mut sum = 0;
    for (i, f) in features.iter().enumerate() {
        if let Some(center) = detect_reflection(f) {
            sum += 100 * (center + 1);
        }

        if let Some(center) = detect_reflection(&transposed_features[i]) {
            sum += center + 1;
        }
    }

    return Some(sum as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
    let patterns = parse_input(input);
    let mut transposed_patterns = Vec::new();
    for p in patterns.iter() {
        transposed_patterns.push(transpose(&p));
    }
    let features = extract_features(patterns);
    let transposed_features = extract_features(transposed_patterns);

    let mut sum = 0;
    for (i, f) in features.iter().enumerate() {
        if let Some(center) = detect_imperfect_reflection(f) {
            sum += 100 * (center + 1);
        }

        if let Some(center) = detect_imperfect_reflection(&transposed_features[i]) {
            sum += center + 1;
        }
    }

    return Some(sum as u32);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
