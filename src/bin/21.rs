use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

advent_of_code::solution!(21);

fn parse(input: &str) -> Vec<Vec<char>> {
    let mut garden = Vec::<Vec<char>>::new();
    for line in input.lines() {
        let mut new_row = Vec::<char>::new();
        for character in line.chars() {
            new_row.push(character);
        }
        garden.push(new_row);
    }
    return garden;
}

fn find_start(garden: &Vec<Vec<char>>) -> (usize, usize) {
    for (i, row) in garden.iter().enumerate() {
        for (j, col) in row.iter().enumerate() {
            if *col == 'S' {
                return (j, i);
            }
        }
    }
    return (0, 0);
}

fn take_step(
    garden: &Vec<Vec<char>>,
    current_steps: &HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let mut new_steps = HashSet::<(usize, usize)>::new();
    for (x, y) in current_steps {
        if *x > 0 && garden[*y][x - 1] != '#' {
            new_steps.insert((x - 1, *y));
        }
        if *x < garden[*y].len() - 1 && garden[*y][x + 1] != '#' {
            new_steps.insert((x + 1, *y));
        }
        if *y > 0 && garden[*y - 1][*x] != '#' {
            new_steps.insert((*x, *y - 1));
        }
        if *y < garden.len() - 1 && garden[*y + 1][*x] != '#' {
            new_steps.insert((*x, *y + 1));
        }
    }
    return new_steps;
}

fn take_step_infinite(
    garden: &Vec<Vec<char>>,
    current_steps: &HashSet<(i128, i128)>,
) -> HashSet<(i128, i128)> {
    let mut new_steps = HashSet::<(i128, i128)>::new();
    for (raw_x, raw_y) in current_steps {
        let y = (raw_y.rem_euclid(garden.len() as i128)) as usize;
        let x = (raw_x.rem_euclid(garden[y].len() as i128)) as usize;
        if x > 0 {
            if garden[y][x - 1] != '#' {
                new_steps.insert((raw_x - 1, *raw_y));
            }
        } else {
            if garden[y][garden[y].len() - 1] != '#' {
                new_steps.insert((raw_x - 1, *raw_y));
            }
        }
        if x < garden[y].len() - 1 {
            if garden[y][x + 1] != '#' {
                new_steps.insert((raw_x + 1, *raw_y));
            }
        } else {
            if garden[y][0] != '#' {
                new_steps.insert((raw_x + 1, *raw_y));
            }
        }
        if y > 0 {
            if garden[y - 1][x] != '#' {
                new_steps.insert((*raw_x, *raw_y - 1));
            }
        } else {
            if garden[garden.len() - 1][x] != '#' {
                new_steps.insert((*raw_x, *raw_y - 1));
            }
        }
        if y < garden.len() - 1 {
            if garden[y + 1][x] != '#' {
                new_steps.insert((*raw_x, *raw_y + 1));
            }
        } else {
            if garden[0][x] != '#' {
                new_steps.insert((*raw_x, *raw_y + 1));
            }
        }
    }
    return new_steps;
}

#[derive(Clone)]
struct StepResult {
    steps: HashSet<(usize, usize)>,
    left: Vec<usize>,
    right: Vec<usize>,
    top: Vec<usize>,
    bot: Vec<usize>,
}

impl StepResult {
    fn new() -> Self {
        Self {
            steps: HashSet::new(),
            left: Vec::new(),
            right: Vec::new(),
            top: Vec::new(),
            bot: Vec::new(),
        }
    }
}

fn take_step_tiled(
    garden: &Vec<Vec<char>>,
    steps: &HashSet<(usize, usize)>,
    memo: &mut HashMap<BTreeSet<(usize, usize)>, StepResult>,
) -> StepResult {
    let memo_key: BTreeSet<(usize, usize)> = BTreeSet::from_iter(steps.clone().into_iter());
    if memo.contains_key(&memo_key) {
        return memo[&memo_key].clone();
    }
    let mut result = StepResult::new();
    for (x, y) in steps {
        if *x > 0 {
            if garden[*y][x - 1] != '#' {
                result.steps.insert((x - 1, *y));
            }
        } else {
            if garden[*y][garden[*y].len() - 1] != '#' {
                result.left.push(*y);
            }
        }
        if *x < garden[*y].len() - 1 {
            if garden[*y][x + 1] != '#' {
                result.steps.insert((x + 1, *y));
            }
        } else {
            if garden[*y][0] != '#' {
                result.right.push(*y);
            }
        }
        if *y > 0 {
            if garden[y - 1][*x] != '#' {
                result.steps.insert((*x, *y - 1));
            }
        } else {
            if garden[garden.len() - 1][*x] != '#' {
                result.top.push(*x);
            }
        }
        if *y < garden.len() - 1 {
            if garden[y + 1][*x] != '#' {
                result.steps.insert((*x, *y + 1));
            }
        } else {
            if garden[0][*x] != '#' {
                result.bot.push(*x);
            }
        }
    }
    memo.insert(memo_key, result.clone());
    return result;
}

fn process_garden_tiles(
    garden: &Vec<Vec<char>>,
    tiles: &HashMap<(i128, i128), HashSet<(usize, usize)>>,
    memo: &mut HashMap<BTreeSet<(usize, usize)>, StepResult>,
) -> HashMap<(i128, i128), HashSet<(usize, usize)>> {
    let mut results = HashMap::new();
    for (key, tile) in tiles.iter() {
        results.insert(key, take_step_tiled(garden, tile, memo));
    }
    let mut new_tiles = HashMap::new();
    for (key, _tile) in tiles.iter() {
        new_tiles.insert(*key, results[key].steps.clone());
    }
    for ((x, y), result) in results.iter() {
        if !result.left.is_empty() {
            if !new_tiles.contains_key(&(x - 1, *y)) {
                new_tiles.insert((x - 1, *y), HashSet::new());
            }
            let len = garden[0].len();
            for incoming in result.left.iter() {
                new_tiles
                    .get_mut(&(x - 1, *y))
                    .unwrap()
                    .insert((len - 1, *incoming));
            }
        }
        if !result.right.is_empty() {
            if !new_tiles.contains_key(&(x + 1, *y)) {
                new_tiles.insert((x + 1, *y), HashSet::new());
            }
            for incoming in result.right.iter() {
                new_tiles
                    .get_mut(&(x + 1, *y))
                    .unwrap()
                    .insert((0, *incoming));
            }
        }
        if !result.top.is_empty() {
            if !new_tiles.contains_key(&(*x, *y - 1)) {
                new_tiles.insert((*x, *y - 1), HashSet::new());
            }
            let len = garden.len();
            for incoming in result.top.iter() {
                new_tiles
                    .get_mut(&(*x, *y - 1))
                    .unwrap()
                    .insert((*incoming, len - 1));
            }
        }
        if !result.bot.is_empty() {
            if !new_tiles.contains_key(&(*x, *y + 1)) {
                new_tiles.insert((*x, *y + 1), HashSet::new());
            }
            for incoming in result.bot.iter() {
                new_tiles
                    .get_mut(&(*x, *y + 1))
                    .unwrap()
                    .insert((*incoming, 0));
            }
        }
    }
    return new_tiles;
}

pub fn part_one(input: &str) -> Option<u128> {
    let garden = parse(input);
    let mut steps = HashSet::new();
    steps.insert(find_start(&garden));
    for _ in 0..64 {
        steps = take_step(&garden, &steps);
    }
    return Some(steps.len() as u128);
}

fn pretty_print_tiles(
    garden: &Vec<Vec<char>>,
    tiles: &HashMap<(i128, i128), HashSet<(usize, usize)>>,
) {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;
    for ((x, y), _value) in tiles.iter() {
        min_x = min_x.min(*x);
        min_y = min_y.min(*y);
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }
    let row_len = garden[0].len();
    let col_len = garden.len();
    for y in min_y..=max_y {
        let mut tile_row = Vec::new();
        for x in min_x..=max_x {
            tile_row.push(tiles.get(&(x, y)));
        }

        for (i, row) in garden.iter().enumerate() {
            for tile in tile_row.iter() {
                match tile {
                    Some(t) => {
                        for (j, c) in row.iter().enumerate() {
                            if t.contains(&(j, i)) {
                                print!("O");
                            } else {
                                print!("{}", c);
                            }
                        }
                    }
                    _ => {
                        for (j, c) in row.iter().enumerate() {
                            print!(" ");
                        }
                    }
                }
            }
            println!("");
        }
    }
}

#[derive(Clone)]
struct StepRecord {
    garden_width: usize,
    garden_height: usize,
    min_x: i128,
    min_y: i128,
    steps: VecDeque<VecDeque<Vec<Vec<bool>>>>,
}

impl StepRecord {
    fn new(width: usize, height: usize) -> Self {
        let mut s = Self {
            garden_width: width,
            garden_height: height,
            min_x: 0,
            min_y: 0,
            steps: VecDeque::with_capacity(500),
        };
        let mut row = VecDeque::with_capacity(500);
        row.push_back(vec![vec![false; width]; height]);
        s.steps.push_back(row);
        return s;
    }
    fn contains(&self, x: i128, y: i128) -> bool {
        if x < self.min_x
            || x - self.min_x >= (self.garden_width * self.steps[0].len()) as i128
            || y < self.min_y
            || y - self.min_y >= (self.garden_height * self.steps.len()) as i128
        {
            return false;
        }
        let tile_x = (x - self.min_x) as usize / self.garden_width;
        let tile_y = (y - self.min_y) as usize / self.garden_height;
        let inner_x = (x as usize).rem_euclid(self.garden_width);
        let inner_y = (y as usize).rem_euclid(self.garden_height);
        return self.steps[tile_y][tile_x][inner_y][inner_x];
    }

    fn insert(&mut self, x: i128, y: i128) {
        if x < self.min_x {
            self.min_x -= self.garden_width as i128;
            for row in self.steps.iter_mut() {
                row.push_front(vec![vec![false; self.garden_width]; self.garden_height]);
            }
        } else if x - self.min_x >= (self.garden_width * self.steps[0].len()) as i128 {
            for row in self.steps.iter_mut() {
                row.push_back(vec![vec![false; self.garden_width]; self.garden_height]);
            }
        }

        if y < self.min_y {
            self.min_y -= self.garden_height as i128;
            let mut new_row = VecDeque::new();
            for _ in self.steps[0].iter() {
                new_row.push_back(vec![vec![false; self.garden_width]; self.garden_height])
            }
            self.steps.push_front(new_row);
        } else if y - self.min_y >= (self.garden_height * self.steps.len()) as i128 {
            let mut new_row = VecDeque::new();
            for _ in self.steps[0].iter() {
                new_row.push_back(vec![vec![false; self.garden_width]; self.garden_height])
            }
            self.steps.push_back(new_row);
        }
        let tile_x = (x - self.min_x) as usize / self.garden_width;
        let tile_y = (y - self.min_y) as usize / self.garden_height;
        let inner_x = (x as usize).rem_euclid(self.garden_width);
        let inner_y = (y as usize).rem_euclid(self.garden_height);
        // println!("biboo");
        // for row in self.steps.iter(){
        //     println!("row: {}", row.len());
        // }
        self.steps[tile_y][tile_x][inner_y][inner_x] = true;
    }

    fn len(&self) -> usize {
        let mut count = 0;
        for x in self.steps.iter() {
            for y in x.iter() {
                for z in y.iter() {
                    for w in z.iter() {
                        if *w {
                            count += 1;
                        }
                    }
                }
            }
        }
        return count;
    }
}

fn even_odd_record_step(
    garden: &Vec<Vec<char>>,
    iteration: u128,
    steps: &Vec<(i128, i128)>,
    evens: &mut StepRecord,
    odds: &mut StepRecord,
) -> Vec<(i128, i128)> {
    let mut new_steps = Vec::new();
    for (raw_x, raw_y) in steps {
        let y = (raw_y.rem_euclid(garden.len() as i128)) as usize;
        let x = (raw_x.rem_euclid(garden[y].len() as i128)) as usize;
        let odd = iteration % 2 == 1;
        if x > 0 {
            if garden[y][x - 1] != '#' {
                if !odds.contains(*raw_x - 1, *raw_y) && !evens.contains(*raw_x - 1, *raw_y) {
                    new_steps.push((raw_x - 1, *raw_y));
                    if odd {
                        evens.insert(raw_x - 1, *raw_y);
                    } else {
                        odds.insert(raw_x - 1, *raw_y);
                    }
                }
            }
        } else {
            if garden[y][garden[y].len() - 1] != '#' {
                if !odds.contains(*raw_x - 1, *raw_y) && !evens.contains(*raw_x - 1, *raw_y) {
                    new_steps.push((raw_x - 1, *raw_y));
                    if odd {
                        evens.insert(*raw_x - 1, *raw_y);
                    } else {
                        odds.insert(*raw_x - 1, *raw_y);
                    }
                }
            }
        }
        if x < garden[y].len() - 1 {
            if garden[y][x + 1] != '#' {
                if !odds.contains(*raw_x + 1, *raw_y) && !evens.contains(*raw_x + 1, *raw_y) {
                    new_steps.push((raw_x + 1, *raw_y));
                    if odd {
                        evens.insert(*raw_x + 1, *raw_y);
                    } else {
                        odds.insert(*raw_x + 1, *raw_y);
                    }
                }
            }
        } else {
            if garden[y][0] != '#' {
                if !odds.contains(*raw_x + 1, *raw_y) && !evens.contains(*raw_x + 1, *raw_y) {
                    new_steps.push((raw_x + 1, *raw_y));
                    if odd {
                        evens.insert(*raw_x + 1, *raw_y);
                    } else {
                        odds.insert(*raw_x + 1, *raw_y);
                    }
                }
            }
        }
        if y > 0 {
            if garden[y - 1][x] != '#' {
                if !odds.contains(*raw_x, *raw_y - 1) && !evens.contains(*raw_x, *raw_y - 1) {
                    new_steps.push((*raw_x, *raw_y - 1));
                    if odd {
                        evens.insert(*raw_x, *raw_y - 1);
                    } else {
                        odds.insert(*raw_x, *raw_y - 1);
                    }
                }
            }
        } else {
            if garden[garden.len() - 1][x] != '#' {
                if !odds.contains(*raw_x, *raw_y - 1) && !evens.contains(*raw_x, *raw_y - 1) {
                    new_steps.push((*raw_x, *raw_y - 1));
                    if odd {
                        evens.insert(*raw_x, *raw_y - 1);
                    } else {
                        odds.insert(*raw_x, *raw_y - 1);
                    }
                }
            }
        }
        if y < garden.len() - 1 {
            if garden[y + 1][x] != '#' {
                if !odds.contains(*raw_x, *raw_y + 1) && !evens.contains(*raw_x, *raw_y + 1) {
                    new_steps.push((*raw_x, *raw_y + 1));
                    if odd {
                        evens.insert(*raw_x, *raw_y + 1);
                    } else {
                        odds.insert(*raw_x, *raw_y + 1);
                    }
                }
            }
        } else {
            if garden[0][x] != '#' {
                if !odds.contains(*raw_x, *raw_y + 1) && !evens.contains(*raw_x, *raw_y + 1) {
                    new_steps.push((*raw_x, *raw_y + 1));
                    if odd {
                        evens.insert(*raw_x, *raw_y + 1);
                    } else {
                        odds.insert(*raw_x, *raw_y + 1);
                    }
                }
            }
        }
    }
    // if iteration % 2 == 0 {
    //     for (x, y) in new_steps.iter() {
    //         if !odds.contains(*x, *y) {
    //             odds.insert(*x, *y);
    //         }
    //     }
    // } else {
    //     for (x, y) in new_steps.iter() {
    //         if !evens.contains(*x, *y) {
    //             evens.insert(*x, *y);
    //         }
    //     }
    // }
    return new_steps;
}

fn even_odd_step(
    garden: &Vec<Vec<char>>,
    iteration: u128,
    steps: &HashSet<(i128, i128)>,
    evens: &mut HashSet<(i128, i128)>,
    odds: &mut HashSet<(i128, i128)>,
) -> HashSet<(i128, i128)> {
    let mut new_steps = HashSet::new();
    for (raw_x, raw_y) in steps {
        let y = (raw_y.rem_euclid(garden.len() as i128)) as usize;
        let x = (raw_x.rem_euclid(garden[y].len() as i128)) as usize;
        if x > 0 {
            if garden[y][x - 1] != '#' {
                new_steps.insert((raw_x - 1, *raw_y));
            }
        } else {
            if garden[y][garden[y].len() - 1] != '#' {
                new_steps.insert((raw_x - 1, *raw_y));
            }
        }
        if x < garden[y].len() - 1 {
            if garden[y][x + 1] != '#' {
                new_steps.insert((raw_x + 1, *raw_y));
            }
        } else {
            if garden[y][0] != '#' {
                new_steps.insert((raw_x + 1, *raw_y));
            }
        }
        if y > 0 {
            if garden[y - 1][x] != '#' {
                new_steps.insert((*raw_x, *raw_y - 1));
            }
        } else {
            if garden[garden.len() - 1][x] != '#' {
                new_steps.insert((*raw_x, *raw_y - 1));
            }
        }
        if y < garden.len() - 1 {
            if garden[y + 1][x] != '#' {
                new_steps.insert((*raw_x, *raw_y + 1));
            }
        } else {
            if garden[0][x] != '#' {
                new_steps.insert((*raw_x, *raw_y + 1));
            }
        }
    }
    let mut real_new_steps = HashSet::new();
    if iteration % 2 == 0 {
        for step in new_steps {
            if !odds.contains(&step) {
                real_new_steps.insert(step);
                odds.insert(step);
            }
        }
    } else {
        for step in new_steps {
            if !evens.contains(&step) {
                real_new_steps.insert(step);
                evens.insert(step);
            }
        }
    }
    return real_new_steps;
}

fn find_min_manhattan_dist(
    garden: &Vec<Vec<char>>,
    (start_x, start_y): (usize, usize),
) -> Vec<Vec<usize>> {
    let mut distances = vec![vec![0; garden[0].len()]; garden.len()];
    let mut steps = HashSet::new();
    steps.insert((start_x, start_y));
    let mut loop_count = 0;
    loop {
        let new_steps = take_step(&garden, &steps);
        loop_count += 1;
        steps.clear();
        let mut changed = false;
        for (x, y) in new_steps {
            if distances[y][x] == 0 {
                distances[y][x] = loop_count;
                steps.insert((x, y));
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }
    distances[start_y][start_x] = 1001;
    return distances;
}

fn count_every_other(
    distances: &Vec<Vec<usize>>,
    limit: i128,
    maxes_memo: &mut HashMap<Vec<Vec<usize>>, u128>,
    value_memo: &mut HashMap<(Vec<Vec<usize>>, i128), u128>,
) -> u128 {
    if limit < 0 {
        return 0;
    }
    if let Some(max) = maxes_memo.get(distances) {
        let mut new_limit = limit.min(*max as i128);
        if limit % 2 == 0 && new_limit % 2 != 0 {
            new_limit += 1;
        }
        if let Some(value) = value_memo.get(&(distances.clone(), new_limit)) {
            return *value;
        }
    }
    let mut count = 0;
    let mut max = 0;
    if limit % 2 == 0 {
        for row in distances {
            for distance in row {
                if *distance > 0
                    && ((*distance <= limit as usize && distance % 2 == 0) || *distance == 1001)
                {
                    count += 1;
                }
                max = max.max(*distance)
            }
        }
    } else {
        for row in distances {
            for distance in row {
                if *distance > 0
                    && *distance <= limit as usize
                    && distance % 2 == 1
                    && *distance != 1001
                {
                    count += 1;
                }
                max = max.max(*distance)
            }
        }
    }
    maxes_memo.insert(distances.clone(), max as u128);
    let mut new_limit = limit.min(max as i128);
    if limit % 2 == 0 && new_limit % 2 != 0 {
        new_limit += 1;
    }
    value_memo.insert((distances.clone(), new_limit), count);
    return count;
}

pub fn part_two(input: &str) -> Option<u128> {
    let garden = parse(input);
    let mut steps = Vec::new();
    let (x, y) = find_start(&garden);
    steps.push((x as i128, y as i128));
    let mut evens = StepRecord::new(garden[0].len(), garden.len());
    let mut odds = StepRecord::new(garden[0].len(), garden.len());
    for i in 0..16 {
        steps = even_odd_record_step(&garden, i, &steps, &mut evens, &mut odds);
        // println!("{}", steps.len());
    }
    // return Some(odds.len() as u128);
    // let garden = parse(input);
    // let mut steps = HashSet::new();
    // steps.insert(find_start(&garden));
    // let mut tiles = HashMap::new();
    // let mut memo = HashMap::new();
    // tiles.insert((0, 0), steps);
    // for _ in 0..327 {
    //     tiles = process_garden_tiles(&garden, &tiles, &mut memo);
    // }
    // // pretty_print_tiles(&garden, &tiles);
    // let mut sum = 0;
    // for (_key, tile) in tiles.iter() {
    //     sum += tile.len() as u128;
    // }
    // return Some(sum);
    let garden = parse(input);
    let start = find_start(&garden);
    let start_distances = find_min_manhattan_dist(&garden, start);
    let left_distances = find_min_manhattan_dist(&garden, (0, start.1)); //when you start from the very left
    let right_distances = find_min_manhattan_dist(&garden, (garden[0].len() - 1, start.1)); //when you start from the very right
    let top_distances = find_min_manhattan_dist(&garden, (start.0, 0)); //when you start from the very top
    let bot_distances = find_min_manhattan_dist(&garden, (start.0, garden.len() - 1)); //when you start from the very bot
    let top_left = find_min_manhattan_dist(&garden, (0, 0));
    let top_right = find_min_manhattan_dist(&garden, (garden[0].len() - 1, 0));
    let bot_left = find_min_manhattan_dist(&garden, (0, garden.len() - 1));
    let bot_right = find_min_manhattan_dist(&garden, (garden[0].len() - 1, garden.len() - 1));
    let actual_step_count = 26501365;
    // let actual_step_count = 16;
    // let actual_step_count = 12;
    let mut count = 0;
    let width = garden[0].len();
    let height = garden.len();
    let mut value_memo: HashMap<(Vec<Vec<usize>>, i128), u128> = HashMap::new();
    let mut maxes_memo: HashMap<Vec<Vec<usize>>, u128> = HashMap::new();
    // let tile_x_count = 1 + actual_step_count / garden[0].len();
    // let tile_y_count = 1 + actual_step_count / garden.len();
    // for tile_y in -(tile_y_count as i128)..=(tile_y_count as i128) {
    //     for tile_x in -(tile_x_count as i128)..=(tile_x_count as i128) {
    //         let mut steps_to_start = 0;
    //         if tile_x > 0 {
    //             steps_to_start += width - start.0; //distance from start to right border;
    //         } else if tile_x < 0 {
    //             steps_to_start += start.0 + 1; //distance from start to right border;
    //         }
    //         if tile_y > 0 {
    //             steps_to_start += height - start.1; //distance from start to right border;
    //         } else if tile_y < 0 {
    //             steps_to_start += start.1 + 1; //distance from start to right border;
    //         }
    //         if tile_x.abs() > 1 {
    //             steps_to_start += tile_x.abs() as usize * width;
    //         }
    //         if tile_y.abs() > 1 {
    //             steps_to_start += tile_y.abs() as usize * height;
    //         }
    //         let remaining_steps = (actual_step_count as i128) - (steps_to_start as i128);
    //         let distances;
    //         match (tile_x, tile_y) {
    //             (x, 0) if x < 0 => {
    //                 distances = &right_distances;
    //             }
    //             (x, 0) if x > 0 => {
    //                 distances = &left_distances;
    //             }
    //             (0, y) if y < 0 => {
    //                 distances = &bot_distances;
    //             }
    //             (0, y) if y > 0 => {
    //                 distances = &top_distances;
    //             }
    //             (x, y) if x < 0 && y < 0 => {
    //                 distances = &bot_right;
    //             }
    //             (x, y) if x > 0 && y < 0 => {
    //                 distances = &bot_left;
    //             }
    //             (x, y) if x < 0 && y > 0 => {
    //                 distances = &top_right;
    //             }
    //             (x, y) if x > 0 && y > 0 => {
    //                 distances = &top_left;
    //             }
    //             _ => {distances = &start_distances;}
    //         }
    //         if(remaining_steps < 0){
    //             continue;
    //         }
    //         // println!("TILE {} {} remaining {}", tile_x, tile_y, remaining_steps);
    //         // println!("count {} += {}", count,  count_every_other(distances, remaining_steps, &mut maxes_memo, &mut value_memo));
    //         count += count_every_other(distances, remaining_steps, &mut maxes_memo, &mut value_memo);
    //     }
    // }
    println!("height {} width {}", height, width);
    println!("start_x {} start_y {}", start.0, start.1);
    //starting tile
    count += count_every_other(
        &start_distances,
        actual_step_count as i128,
        &mut maxes_memo,
        &mut value_memo,
    );
    println!("start {}", count);
    //inner parts
    let perpendicular_length = actual_step_count - start.0;
    let perp_tile_count = (perpendicular_length / width) - 1; //subtract one to take off the tips
    let mut total_tiles: u128 = 0;
    let mut odd_count = 0;
    let mut even_count = 0;
    for i in 1..=perp_tile_count {
        total_tiles += i as u128;
        odd_count += i / 2;
        even_count += i / 2;
        if i % 2 == 1 {
            if width % 2 == 0 {
                even_count += 1;
            } else {
                odd_count += 1;
            }
        }
    }
    println!("total {}", total_tiles);
    println!("odds count {} evens count {}", odd_count, even_count);
    for dist in [&left_distances, &right_distances, &top_distances, &bot_distances] {
        let odds_full = count_every_other(&dist, 1001, &mut maxes_memo, &mut value_memo);
        let evens_full = count_every_other(&dist, 1002, &mut maxes_memo, &mut value_memo);
        println!("odds {} evens {}", odds_full, evens_full);
        count += odd_count as u128 * odds_full;
        count += even_count as u128 * evens_full;
        //tip
        let tip = count_every_other(&dist, width as i128 - 1, &mut maxes_memo, &mut value_memo);
        println!("tip {}", tip);
        count += tip;
    }
    for dist in [&bot_left, &bot_right, &top_left, &top_right] {
        let little = count_every_other(dist, start.0 as i128 - 1, &mut maxes_memo, &mut value_memo);
        let big = count_every_other(dist, (width + start.0) as i128 - 1, &mut maxes_memo, &mut value_memo);

        println!("little {} big {} with {} each", little, big, perp_tile_count);
        count += perp_tile_count as u128 * little;
        count += perp_tile_count as u128 * big;
        count += little;
    }
    //border pieces
    //there's one directly adjacent to the tips
    // for dist in [&bot_left, &bot_right, &top_left, &top_right]{
    //     let little = count_every_other(dist, start.0 as i128 - 1, &mut maxes_memo, &mut value_memo);
    //     let big = count_every_other(dist, width as i128, &mut maxes_memo, &mut value_memo);
    //     count += perp_tile_count as u128 * (little + big);
    //     count += little;//extra one adjacent to the tip
    // }
    Some(count as u128)
}

fn find_max(distances: &Vec<Vec<usize>>) -> usize {
    let mut max = 0;
    for row in distances.iter() {
        for tile in row.iter() {
            if *tile == 1001 {
                continue;
            }
            max = max.max(*tile);
        }
    }
    return max;
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(13));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
