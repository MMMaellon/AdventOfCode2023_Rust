advent_of_code::solution!(5);

struct Range {
    start: usize,
    length: usize,
}

impl Range {
    fn contains(&self, index: usize) -> bool {
        return index >= self.start && index < self.start + self.length;
    }

    fn find_mapped_index(&self, other_range_start: usize, index: usize) -> usize {
        self.start + index - other_range_start
    }
}

struct Dictionary {
    keys: Vec<Range>,
    values: Vec<Range>,
}

impl Dictionary {
    fn new() -> Self {
        Self {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }
    fn look_up(&self, num: usize) -> usize {
        for (i, range) in self.keys.iter().enumerate() {
            if range.contains(num) {
                return self.values[i].find_mapped_index(range.start, num);
            }
        }
        return num;
    }
    fn add_from_str(&mut self, input: String) {
        for line in input.lines() {
            let (mut dest, mut source, mut len): (usize, usize, usize) = (0, 0, 0);
            for (i, num_str) in line.splitn(3, ' ').enumerate() {
                match i {
                    0 => dest = num_str.parse::<usize>().unwrap(),
                    1 => source = num_str.parse::<usize>().unwrap(),
                    2 => len = num_str.parse::<usize>().unwrap(),
                    _ => {}
                }
            }
            self.keys.push(Range {
                start: source,
                length: len,
            });
            self.values.push(Range {
                start: dest,
                length: len,
            });
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut seed_soil = Dictionary::new();
    let mut soil_to_fertilizer = Dictionary::new();
    let mut fertilizer_to_water = Dictionary::new();
    let mut water_to_light = Dictionary::new();
    let mut light_to_temperature = Dictionary::new();
    let mut termpature_to_humidity = Dictionary::new();
    let mut humidity_to_location = Dictionary::new();
    let mut seeds: Vec<usize> = Vec::new();

    let mut curr_dict = Dictionary::new();
    for line in input.lines() {
        if seeds.len() == 0 {
            for word in line.trim_start_matches("seeds: ").split_whitespace() {
                seeds.push(word.parse().unwrap());
            }
        } else {
            if line.trim().is_empty() {
                continue;
            } else {
                println!( "matching {}", line);
                match line.trim() {
                    "seed-to-soil map:" => {}
                    "soil-to-fertilizer map:" => {
                        seed_soil = curr_dict;
                        curr_dict = Dictionary::new();
                    }
                    "fertilizer-to-water map:" => {
                        soil_to_fertilizer = curr_dict;
                        curr_dict = Dictionary::new();
                    }
                    "water-to-light map:" => {
                        fertilizer_to_water = curr_dict;
                        curr_dict = Dictionary::new();
                    }
                    "light-to-temperature map:" => {
                        water_to_light = curr_dict;
                        curr_dict = Dictionary::new();
                    }

                    "temperature-to-humidity map:" => {
                        light_to_temperature = curr_dict;
                        curr_dict = Dictionary::new();
                    }
                    "humidity-to-location map:" => {
                        termpature_to_humidity = curr_dict;
                        curr_dict = Dictionary::new();
                    }

                    _ => curr_dict.add_from_str(line.to_string()),
                }
            }
        }
    }
    humidity_to_location = curr_dict;
    let mut lowest_seed = 0;
    let mut curr_seed = 0;
    for seed in seeds {
        curr_seed = seed_soil.look_up(seed);
        curr_seed = soil_to_fertilizer.look_up(curr_seed);
        curr_seed = fertilizer_to_water.look_up(curr_seed);
        curr_seed = water_to_light.look_up(curr_seed);
        curr_seed = light_to_temperature.look_up(curr_seed);
        curr_seed = termpature_to_humidity.look_up(curr_seed);
        curr_seed = humidity_to_location.look_up(curr_seed);
        if lowest_seed == 0 || curr_seed < lowest_seed {
            lowest_seed = curr_seed;
        }
    }
    Some(lowest_seed as u32)
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
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
