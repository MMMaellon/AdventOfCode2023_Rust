use std::f64::INFINITY;

advent_of_code::solution!(24);

#[derive(Clone, Debug)]
struct Hail {
    x: f64,
    y: f64,
    z: f64,
    vx: f64,
    vy: f64,
    vz: f64,
}

impl Hail {
    fn xy(&self) -> (f64, f64) {
        return (self.x, self.y);
    }
    fn vxy(&self) -> (f64, f64) {
        return (self.vx, self.vy);
    }
    fn xyz(&self) -> (f64, f64, f64) {
        return (self.x, self.y, self.z);
    }
    fn vxyz(&self) -> (f64, f64, f64) {
        return (self.vx, self.vy, self.vz);
    }

    fn calc_xy_intersect(&self, other: &Hail) -> Option<(f64, f64)> {
        let dist_vector = (other.x - self.x, other.y - self.y);
        // println!("dist_vector {:?}", dist_vector);
        let dist_projection = projection(dist_vector, self.vxy());
        // println!("dist_projection {:?}", dist_projection);
        let dist_inverse_projection = (
            dist_vector.0 - dist_projection.0,
            dist_vector.1 - dist_projection.1,
        );
        // println!("dist_inverse_projection {:?}", dist_inverse_projection);
        let other_vel_dist_projection = projection(other.vxy(), dist_inverse_projection);
        // println!("other_vel_dist_projection  {:?}", other_vel_dist_projection );
        let dist_dot = (other_vel_dist_projection.0 * dist_inverse_projection.0)
            + (other_vel_dist_projection.1 * dist_inverse_projection.1);
        //in order for the gap to ever close, these two must be headed in opposite directions
        // println!("dist_dot {:?}", dist_dot);
        if dist_dot > 0.0 {
            return None;
        }

        let other_proj_mag = magnitude(other_vel_dist_projection);
        let dist_inverse_proj_mag = magnitude(dist_inverse_projection);
        // println!("other_proj_mag {:?}", other_proj_mag);
        // println!("dist_inverse_proj_mag {:?}", dist_inverse_proj_mag);

        let time = dist_inverse_proj_mag / other_proj_mag;
        let intersection = (other.x + (time * other.vx), other.y + (time * other.vy));
        let intersection_dot =
            (self.vx * (intersection.0 - self.x)) + (self.vy * (intersection.1 - self.y));
        // println!("intersection {:?}", intersection);
        // println!("intersection dot {}", intersection_dot);
        if intersection_dot > 0.0 {
            return Some(intersection);
        }
        return None;
    }
    fn calc_xyz_intersect(&self, other: &Hail) -> Option<(f64, f64, f64)> {
        let dist_vector = (other.x - self.x, other.y - self.y, other.z - self.z);
        // println!("dist_vector {:?}", dist_vector);
        let dist_projection = projection_xyz(dist_vector, self.vxyz());
        // println!("dist_projection {:?}", dist_projection);
        let dist_inverse_projection = (
            dist_vector.0 - dist_projection.0,
            dist_vector.1 - dist_projection.1,
            dist_vector.2 - dist_projection.2,
        );
        // println!("dist_inverse_projection {:?}", dist_inverse_projection);
        let other_vel_dist_projection = projection_xyz(other.vxyz(), dist_inverse_projection);
        // println!("other_vel_dist_projection  {:?}", other_vel_dist_projection );
        let dist_dot = (other_vel_dist_projection.0 * dist_inverse_projection.0)
            + (other_vel_dist_projection.1 * dist_inverse_projection.1)
            + (other_vel_dist_projection.2 * dist_inverse_projection.2);
        //in order for the gap to ever close, these two must be headed in opposite directions
        // println!("dist_dot {:?}", dist_dot);
        if dist_dot > 0.0 {
            return None;
        }

        let other_proj_mag = magnitude_xyz(other_vel_dist_projection);
        let dist_inverse_proj_mag = magnitude_xyz(dist_inverse_projection);
        // println!("other_proj_mag {:?}", other_proj_mag);
        // println!("dist_inverse_proj_mag {:?}", dist_inverse_proj_mag);
        if other_proj_mag == 0.0 {
            return None;
        }

        let time = dist_inverse_proj_mag / other_proj_mag;
        let intersection = (
            other.x + (time * other.vx),
            other.y + (time * other.vy),
            other.z + (time * other.vz),
        );

        if intersection.0 == INFINITY || intersection.1 == INFINITY || intersection.2 == INFINITY {
            return None;
        }
        let intersection_dot = (self.vx * (intersection.0 - self.x))
            + (self.vy * (intersection.1 - self.y))
            + (self.vz * (intersection.2 - self.z));
        // println!("intersection {:?}", intersection);
        // println!("intersection dot {}", intersection_dot);
        if intersection_dot > 0.0 {
            return Some(intersection);
        }
        return None;
    }

    fn calc_xyz_intersect_time(self, other: &Hail) -> usize {
        let rel_vx = self.vx - other.vx;
        let distance = (other.x - self.x, other.y - self.y, other.z - self.z);
        if (rel_vx.is_sign_positive() && distance.0.is_sign_negative())
            || (rel_vx.is_sign_negative() && distance.0.is_sign_positive())
        {
            return 0;
        }
        if rel_vx == 0.0 || distance.0 % rel_vx != 0.0 {
            return 0;
        }
        let time = distance.0 / rel_vx;
        if self.y + (time * self.vy) == other.y + (time * self.vy)
            && self.z + (time * self.vz) == other.z + (time * self.vz)
        {
            return time as usize;
        }
        0
    }
}

fn magnitude(a: (f64, f64)) -> f64 {
    if a.0 == 0.0 && a.1 == 0.0 {
        return 0.0;
    }
    return ((a.0 * a.0) + (a.1 * a.1)).sqrt();
}
fn magnitude_xyz(a: (f64, f64, f64)) -> f64 {
    if a.0 == 0.0 && a.1 == 0.0 && a.2 == 0.0 {
        return 0.0;
    }
    return ((a.0 * a.0) + (a.1 * a.1) + (a.2 * a.2)).sqrt();
}

//projects A onto B
fn projection(a: (f64, f64), b: (f64, f64)) -> (f64, f64) {
    if (a.0 == 0.0 && a.1 == 0.0) || (b.0 == 0.0 && b.1 == 0.0) {
        return (0.0, 0.0);
    }
    let dot = (a.0 * b.0) + (a.1 * b.1);
    let mag_2 = (b.0 * b.0) + (b.1 * b.1);
    let scalar = dot / mag_2;
    return (b.0 * scalar, b.1 * scalar);
}
fn projection_xyz(a: (f64, f64, f64), b: (f64, f64, f64)) -> (f64, f64, f64) {
    if (a.0 == 0.0 && a.1 == 0.0 && a.2 == 0.0) || (b.0 == 0.0 && b.1 == 0.0 && b.2 == 0.0) {
        return (0.0, 0.0, 0.0);
    }
    let dot = (a.0 * b.0) + (a.1 * b.1) + (a.2 * b.2);
    let mag_2 = (b.0 * b.0) + (b.1 * b.1) + (b.2 * b.2);
    let scalar = dot / mag_2;
    return (b.0 * scalar, b.1 * scalar, b.2 * scalar);
}

fn parse(input: &str) -> Vec<Hail> {
    let mut hail = Vec::new();

    for line in input.lines() {
        if let Some((pos, vel)) = line.split_once(" @ ") {
            if let Some((x_str, pos_yz)) = pos.split_once(", ") {
                if let Some((y_str, z_str)) = pos_yz.split_once(", ") {
                    if let Some((vx_str, vel_yz)) = vel.split_once(", ") {
                        if let Some((vy_str, vz_str)) = vel_yz.split_once(", ") {
                            if let (Ok(x), Ok(y), Ok(z), Ok(vx), Ok(vy), Ok(vz)) = (
                                x_str.trim().parse::<f64>(),
                                y_str.trim().parse::<f64>(),
                                z_str.trim().parse::<f64>(),
                                vx_str.trim().parse::<f64>(),
                                vy_str.trim().parse::<f64>(),
                                vz_str.trim().parse::<f64>(),
                            ) {
                                hail.push(Hail {
                                    x,
                                    y,
                                    z,
                                    vx,
                                    vy,
                                    vz,
                                })
                            }
                        }
                    }
                }
            }
        }
    }

    return hail;
}

pub fn part_one(input: &str) -> Option<u32> {
    let hail = parse(input);
    // let lower = 7.0;
    // let upper = 27.0;
    let lower = 200000000000000.0;
    let upper = 400000000000000.0;
    let mut count = 0;
    // println!("a: {:?}", hail[0].xy());
    // println!("a: {:?}", hail[1].xy());
    // println!("test: {:?}", hail[0].calc_xy_intersect(&hail[1]));
    for i in 0..hail.len() {
        for j in (i + 1)..hail.len() {
            if let Some(intersection) = hail[i].calc_xy_intersect(&hail[j]) {
                // println!("intersection found between {} and {}", i, j);
                if intersection.0 >= lower
                    && intersection.0 <= upper
                    && intersection.1 >= lower
                    && intersection.1 <= upper
                {
                    count += 1;
                }
            }
        }
    }
    return Some(count);
}

fn find_commmon_denominator(velocity_matches: &Vec<(f64, f64)>) -> Vec<i128> {
    let mut output = Vec::new();
    let mut lower: i128 = -10000;
    let mut upper: i128 = 10000;
    // for x in velocity_matches.iter() {
    //     lower = lower.min((x.0 + x.1) as i128);
    //     lower = lower.min((x.0 - x.1) as i128);
    //     upper = upper.max((x.0 + x.1) as i128);
    //     upper = upper.max((x.0 - x.1) as i128);
    // }
    for p in lower..=upper {
        let mut all_matches = true;
        for x in velocity_matches.iter() {
            let excess = p - x.0 as i128;
            if excess == 0 || x.1 as i128 % excess != 0 {
                all_matches = false;
                break;
            }
        }
        if all_matches {
            output.push(p);
        }
    }
    return output;
}

fn find_start(vx: i128, vy: i128, vz: i128, hail: &Vec<Hail>) -> (f64, f64, f64) {
    let mut start = (0.0, 0.0, 0.0);

    for i in 0..hail.len() {
        let first_hail = &hail[i];
        let mut first_rock: Hail = first_hail.to_owned();
        first_rock.vx -= vx as f64;
        first_rock.vy -= vy as f64;
        first_rock.vz -= vz as f64;
        let mut last_collision = (0.0, 0.0, 0.0);
        for j in (i + 1)..hail.len() {
            let last_hail = &hail[j];
            let mut last_rock: Hail = last_hail.to_owned();
            last_rock.vx -= vx as f64;
            last_rock.vy -= vy as f64;
            last_rock.vz -= vz as f64;

            if let Some((x, y, z)) = first_rock.calc_xyz_intersect(&last_rock) {
                if x != 0.0
                    && y != 0.0
                    && z != 0.0
                    && x % 1.0 == 0.0
                    && y % 1.0 == 0.0
                    && z % 1.0 == 0.0
                {
                    if last_collision != (0.0, 0.0, 0.0) {
                        if last_collision == (x, y, z) {
                            start = last_collision;
                        } else {
                            println!("mismatch!");
                            break;
                        }
                    }
                    last_collision = (x, y, z);
                }
            } else {
            }
        }
    }

    return start;
}

pub fn part_two(input: &str) -> Option<f64> {
    let hail = parse(input);
    //find matching vx and calc distance
    let mut x_matches = Vec::new();
    let mut y_matches = Vec::new();
    let mut z_matches = Vec::new();
    for i in 0..hail.len() {
        for j in (i + 1)..hail.len() {
            if hail[i].vx == hail[j].vx {
                let vel = hail[i].vx;
                let dist = hail[i].x.max(hail[j].x) - hail[i].x.min(hail[j].x);
                x_matches.push((vel, dist));
            }
            if hail[i].vy == hail[j].vy {
                let vel = hail[i].vy;
                let dist = hail[i].y.max(hail[j].y) - hail[i].y.min(hail[j].y);
                y_matches.push((vel, dist));
            }
            if hail[i].vz == hail[j].vz {
                let vel = hail[i].vz;
                let dist = hail[i].z.max(hail[j].z) - hail[i].z.min(hail[j].z);
                z_matches.push((vel, dist));
            }
        }
    }
    // let rock = Hail{
    //     x: 24.0,
    //     y: 13.0,
    //     z: 10.0,
    //     vx: -3.0,
    //     vy: 1.0,
    //     vz: 2.0,
    // };

    // rock.calc_xyz_intersect(&hail[4]);

    let mut x_den = find_commmon_denominator(&x_matches);
    let mut y_den = find_commmon_denominator(&y_matches);
    let mut z_den = find_commmon_denominator(&z_matches);

    println!("x {:?}", x_den);
    println!("y {:?}", y_den);
    println!("z {:?}", z_den);

    // println!("test {:?}", find_start(-3, 1, 2, &hail));

    for vx in x_den.iter() {
        for vy in y_den.iter() {
            for vz in z_den.iter() {
                // println!("trying {} {} {}", vx, vy, vz);
                let (x, y, z) = find_start(*vx, *vy, *vz, &hail);
                if (x + y + z) != 0.0 {
                    println!("We found it! {} {} {}", x, y, z);
                    return Some(x + y + z);
                }
            }
        }
    }
    // println!("x matches {:?}", x_matches);
    // println!("y matches {:?}", y_matches);
    // println!("z matches {:?}", z_matches);

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let a = Hail {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            vx: 1.0,
            vy: 0.0,
            vz: 0.0,
        };
        let b = Hail {
            x: 1.0,
            y: 2.0,
            z: 0.0,
            vx: 1.0,
            vy: -1.0,
            vz: 0.0,
        };
        // let intersect_test =a.calc_xy_intersect(&b);
        // // println!("test: {:?}", intersect_test);
        // assert_eq!(intersect_test, Some((3.0, 0.0)));

        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        // assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
