advent_of_code::solution!(5);

#[derive(Copy, Clone, Debug)]
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
    fn split_range(&self, mut range: Vec<Range>) -> Vec<Range>{
        for key_range in self.keys.iter() {
            let mut new_range : Vec<Range> = Vec::new();
            for r in range {
                let split_by_start = key_range.start != r.start && r.contains(key_range.start);
                let split_by_end = key_range.start + key_range.length != r.start + r.length && r.contains(key_range.start + key_range.length - 1);
                if split_by_start && split_by_end {
                    let part_length = key_range.start - r.start;
                    new_range.push(Range{start: r.start, length : part_length});
                    new_range.push(*key_range);
                    new_range.push(Range{start: key_range.start + key_range.length, length : r.length - part_length - key_range.length});
                } else if split_by_start{
                    let part_length = key_range.start - r.start;
                    new_range.push(Range{start: r.start, length : part_length});
                    new_range.push(Range{start: key_range.start, length : r.length - part_length});
                } else if split_by_end{
                    let part_length = key_range.length + key_range.start - r.start;
                    new_range.push(Range{start: r.start, length: part_length});
                    new_range.push(Range{start: r.start + part_length, length: r.length - part_length});
                } else {
                    new_range.push(r);
                }
            }
            range = new_range;
        }
        let mut new_range: Vec<Range> = Vec::new();
        for r in range{
            new_range.push(Range{start: self.look_up(r.start), length: r.length});
        }
        return new_range;
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
    let humidity_to_location;
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
    let mut curr_seed;
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
    let mut seed_soil = Dictionary::new();
    let mut soil_to_fertilizer = Dictionary::new();
    let mut fertilizer_to_water = Dictionary::new();
    let mut water_to_light = Dictionary::new();
    let mut light_to_temperature = Dictionary::new();
    let mut termpature_to_humidity = Dictionary::new();
    let humidity_to_location;
    let mut seeds: Vec<Range> = Vec::new();
    let mut seed_length: usize = 0;
    let mut seed_start: usize = 0;
    let mut curr_dict = Dictionary::new();
    for line in input.lines() {
        if seeds.len() == 0 {
            for word in line.trim_start_matches("seeds: ").split_whitespace() {
                if seed_length == 0 {
                    seed_start = word.parse().unwrap();
                    seed_length = 1;
                } else {
                    seed_length = word.parse().unwrap();
                    seeds.push(Range{start: seed_start, length: seed_length});
                    seed_start = 0;
                    seed_length = 0;
                }
            }
        } else {
            if line.trim().is_empty() {
                continue;
            } else {
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
    let mut curr_seed;
    curr_seed = seed_soil.split_range(seeds);
    curr_seed = soil_to_fertilizer.split_range(curr_seed);
    curr_seed = fertilizer_to_water.split_range(curr_seed);
    curr_seed = water_to_light.split_range(curr_seed);
    curr_seed = light_to_temperature.split_range(curr_seed);
    curr_seed = termpature_to_humidity.split_range(curr_seed);
    curr_seed = humidity_to_location.split_range(curr_seed);
    for seed in curr_seed {
        if lowest_seed == 0 || seed.start < lowest_seed {
            lowest_seed = seed.start;
        }
    }
    Some(lowest_seed as u32)
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
        assert_eq!(result, Some(46));
    }
}
