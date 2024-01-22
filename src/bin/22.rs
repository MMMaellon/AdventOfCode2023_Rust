use std::cmp::Ordering;

advent_of_code::solution!(22);

#[derive(Clone)]
struct Brick {
    pos1: (usize, usize, usize),
    pos2: (usize, usize, usize),
    id: usize,
    below: Vec<usize>,
    above: Vec<usize>,
}

impl Brick {
    fn new(pos1: (usize, usize, usize), pos2: (usize, usize, usize), id: usize) -> Self {
        Self {
            pos1,
            pos2,
            id,
            below: Vec::new(),
            above: Vec::new(),
        }
    }
}

fn parse_vector(input: &str) -> Option<(usize, usize, usize)> {
    if let Some((x, rest)) = input.split_once(',') {
        if let Some((y, z)) = rest.split_once(',') {
            if let (Ok(x_val), Ok(y_val), Ok(z_val)) =
                (x.parse::<usize>(), y.parse::<usize>(), z.parse::<usize>())
            {
                return Some((x_val, y_val, z_val));
            }
        }
    }
    return None;
}

fn parse(input: &str) -> Vec<Brick> {
    let mut bricks: Vec<Brick> = Vec::new();
    for line in input.lines() {
        if let Some((start, stop)) = line.split_once('~') {
            if let (Some(start_vector), Some(stop_vector)) =
                (parse_vector(start), parse_vector(stop))
            {
                bricks.push(Brick::new(start_vector, stop_vector, bricks.len()))
            }
        }
    }
    return bricks;
}

fn move_down(bricks: &Vec<Brick>) -> Vec<Brick> {
    let mut fallen_bricks = (*bricks).clone();
    for (i, brick) in bricks.iter().enumerate() {
        let mut max_z = 0;
        let mut max_z_indexes: Vec<usize> = Vec::new();
        for j in (0..i).rev() {
            let other = &fallen_bricks[j];
            if other.pos1.2.max(other.pos2.2) < max_z
                || other.pos2.2 < max_z
                || other.pos1.2.max(other.pos2.2) >= brick.pos1.2.max(brick.pos2.2)
            {
                continue;
            }
            let min_x = brick.pos1.0.min(brick.pos2.0);
            let max_x = brick.pos1.0.max(brick.pos2.0);
            let min_x_other = other.pos1.0.min(other.pos2.0);
            let max_x_other = other.pos1.0.max(other.pos2.0);
            let x_overlap = (min_x >= min_x_other && min_x <= max_x_other)
                || (min_x_other >= min_x && min_x_other <= max_x);
            if !x_overlap {
                continue;
            }
            let min_y = brick.pos1.1.min(brick.pos2.1);
            let max_y = brick.pos1.1.max(brick.pos2.1);
            let min_y_other = other.pos1.1.min(other.pos2.1);
            let max_y_other = other.pos1.1.max(other.pos2.1);
            let y_overlap = (min_y >= min_y_other && min_y <= max_y_other)
                || (min_y_other >= min_y && min_y_other <= max_y);
            if x_overlap && y_overlap {
                if max_z == other.pos1.2.max(other.pos2.2) {
                    max_z_indexes.push(j);
                } else {
                    max_z = other.pos1.2.max(other.pos2.2);
                    max_z_indexes = vec![j];
                }
            }
        }
        if let Some(brick) = fallen_bricks.get_mut(i) {
            let fall_dist = brick.pos1.2.min(brick.pos2.2) - (max_z + 1);
            // println!("brick {} - {}", brick.pos1.2, brick.pos2.2);
            brick.pos1.2 -= fall_dist;
            brick.pos2.2 -= fall_dist;
            for below in max_z_indexes.iter() {
                brick.below.push(*below);
            }
        }
        for below in max_z_indexes.iter() {
            fallen_bricks[*below].above.push(i);
        }
    }

    return fallen_bricks;
}

fn find_disintegratable(fallen: &Vec<Brick>) -> Vec<usize> {
    let mut disintegratable = Vec::new();
    for brick in fallen.iter() {
        let mut ok = true;
        for above_index in brick.above.iter() {
            let above = &fallen[*above_index];
            if above.below.len() < 2 {
                ok = false;
                break;
            }
        }
        if ok {
            disintegratable.push(brick.id);
        }
    }
    return disintegratable;
}

fn generate_edge_vec(fallen: &Vec<Brick>) -> Vec<Vec<bool>> {
    let mut edges = Vec::new();
    for brick in fallen.iter() {
        edges.push(vec![false; brick.below.len()]);
    }
    return edges;
}

fn simulate_delete(
    fallen: &Vec<Brick>,
    index: usize,
    deleted: &mut Vec<bool>,
    memo: &mut Vec<Option<usize>>,
) -> usize {
    if let Some(Some(val)) = memo.get(index) {
        return *val;
    }
    let mut count = 0;
    for above in fallen[index].above.iter() {
        let mut all_fallen = true;
        for below in fallen[*above].below.iter() {
            if !deleted[*below] {
                all_fallen = false;
                break;
            }
        }
        if all_fallen {
            deleted[index] = true;
            count += 1 + simulate_delete(fallen, *above, deleted, memo);
        }
    }
    memo[index] = Some(count);
    return count;
}

fn walk_edges(
    fallen: &Vec<Brick>,
    index: usize,
    memo: &mut Vec<Option<Vec<Vec<bool>>>>,
) -> Vec<Vec<bool>> {
    if let Some(val) = &memo[index] {
        return val.to_vec();
    }
    let mut new_edges = generate_edge_vec(fallen);
    for above in fallen[index].above.iter() {
        for (i, below_index) in fallen[*above].below.iter().enumerate() {
            if *below_index == index {
                new_edges[*above][i] = true;
                break;
            }
        }
    }
    let mut changed;
    loop {
        changed = false;
        for brick_index in 0..fallen.len(){
            if brick_index == index{
                continue;
            }
            let mut all_fallen = !fallen[brick_index].below.is_empty();
            for (i, _below_index) in fallen[brick_index].below.iter().enumerate() {
                if !new_edges[brick_index][i] {
                    all_fallen = false;
                    break;
                }
            }
            if all_fallen {
                for (i, row) in walk_edges(fallen, brick_index, memo).iter().enumerate() {
                    for (j, col) in row.iter().enumerate() {
                        if *col && !new_edges[i][j] {
                            new_edges[i][j] = true;
                            changed = true;
                        }
                    }
                }
            }
        }
        if !changed {
            break;
        }
    }
    memo[index] = Some(new_edges.to_vec());
    return new_edges;
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut bricks: Vec<Brick> = parse(input);
    bricks.sort_by(|x, y| {
        let x_z = x.pos1.2.min(x.pos2.2);
        let y_z = y.pos1.2.min(y.pos2.2);
        if x_z < y_z {
            return Ordering::Less;
        } else if x_z > y_z {
            return Ordering::Greater;
        }
        return std::cmp::Ordering::Equal;
    });
    let fallen_bricks = move_down(&bricks);
    let disintegratable = find_disintegratable(&fallen_bricks);
    return Some(disintegratable.len());
}

fn print_memo(fallen: &Vec<Brick>, memo: &Vec<Option<Vec<Vec<bool>>>>, index: usize) {
    if let Some(val) = &memo[index] {
        for (i, thingy) in val.iter().enumerate() {
            print!("{} -> (", i);
            for (j, below) in thingy.iter().enumerate() {
                if *below {
                    print!("{}[1]", fallen[i].below[j]);
                } else {
                    print!("{}[0]", fallen[i].below[j]);
                }
            }
            println!(")");
        }
    }
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut bricks: Vec<Brick> = parse(input);
    bricks.sort_by(|x, y| {
        let x_z = x.pos1.2.min(x.pos2.2);
        let y_z = y.pos1.2.min(y.pos2.2);
        if x_z < y_z {
            return Ordering::Less;
        } else if x_z > y_z {
            return Ordering::Greater;
        }
        return std::cmp::Ordering::Equal;
    });
    let fallen_bricks = move_down(&bricks);
    let mut max = 0;
    let mut sum = 0;
    let mut count_vec = vec![0; fallen_bricks.len()];
    let mut memo = vec![None; fallen_bricks.len()];
    for brick in fallen_bricks.iter().rev() {
        walk_edges(&fallen_bricks, brick.id, &mut memo);
    }
    for (index, m) in memo.iter().enumerate() {
        if let Some(edges) = m {
            let mut count = 0;
            for i in edges.iter() {
                let mut all = !i.is_empty();
                for below in i.iter() {
                    if !below {
                        all = false;
                        break;
                    }
                }

                if all {
                    count += 1;
                }
            }
            max = max.max(count);
            sum += count;
            count_vec[index] = count;
        }
    }
    // print_memo(&fallen_bricks, &memo, 0);
    // println!("{:?}", count_vec);
    return Some(sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
