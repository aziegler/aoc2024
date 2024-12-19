use std::{
    collections::HashMap,
    io::{self, Read},
};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, space1},
    combinator::{opt, recognize},
    multi::many0,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<isize> {
    let mut robots = parse(input).unwrap().1;
    let field_size = (101, 103);
    (0..200000).for_each(|i| {
        robots.iter_mut().for_each(|r| step(r, field_size));
        let score = score(&robots, field_size);
        let mean_variation = mean_variation(&robots);
        if (mean_variation.0 < 30 && mean_variation.1 < 30) {
            println!("{}", display(&robots, field_size));
            println!(
                "Map {}, score {}, mean_x {}, mean_y {}",
                i + 1,
                score,
                mean_variation.0,
                mean_variation.1
            );
            pause();
        }
    });
    Some(score(&robots, field_size))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

pub fn mean_variation(robots: &Vec<Robot>) -> (usize, usize) {
    let sum = robots
        .iter()
        .tuple_windows()
        .map(|(r1, r2)| (r1.pos.0.abs_diff(r2.pos.0), r1.pos.1.abs_diff(r2.pos.1)))
        .fold((0, 0), |(a, b), (x, y)| (a + x, b + y));
    return (sum.0 / robots.len(), sum.1 / robots.len());
}

fn pause() {
    let mut stdin = io::stdin();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

pub fn display(robots: &Vec<Robot>, field_size: (usize, usize)) -> String {
    let mut map_vis = String::new();
    let map: HashMap<(usize, usize), usize> = robots.iter().map(|r| r.pos).counts();
    for i in 0..field_size.1 {
        for j in 0..field_size.0 {
            if map.contains_key(&(j, i)) {
                map_vis.push_str(map.get(&(j, i)).unwrap().to_string().as_str());
            } else {
                map_vis.push_str(".");
            }
        }
        map_vis.push('\n');
    }
    map_vis
}

pub fn score(robots: &Vec<Robot>, field_size: (usize, usize)) -> isize {
    let x_mid = field_size.0 / 2;
    let y_mid = field_size.1 / 2;
    let mut quadrants: Vec<isize> = vec![0, 0, 0, 0];
    robots.iter().for_each(|robot| {
        if robot.pos.0 < x_mid {
            if robot.pos.1 < y_mid {
                quadrants[0] += 1
            } else if robot.pos.1 > y_mid {
                quadrants[1] += 1
            }
        } else if robot.pos.0 > x_mid {
            if robot.pos.1 < y_mid {
                quadrants[2] += 1
            } else if robot.pos.1 > y_mid {
                quadrants[3] += 1
            }
        }
    });
    return quadrants[0] * quadrants[1] * quadrants[2] * quadrants[3];
}

pub fn step(robot: &mut Robot, field_size: (usize, usize)) {
    robot.pos.0 = wrap(robot.pos.0, robot.velocity.0, field_size.0);
    robot.pos.1 = wrap(robot.pos.1, robot.velocity.1, field_size.1);
}

pub fn wrap(val: usize, step: isize, bound: usize) -> usize {
    let mut return_val: isize = (val as isize) + step;
    if return_val < 0 {
        return_val += bound as isize;
    }
    if return_val >= bound as isize {
        return_val -= bound as isize;
    }
    if (return_val < 0 || return_val >= bound as isize) {
        panic!("Still off by something");
    }
    return_val as usize
}

pub fn parse(input: &str) -> IResult<&str, Vec<Robot>> {
    many0(robot)(input)
}

pub fn robot(input: &str) -> IResult<&str, Robot> {
    let (rem, (p, v)) =
        terminated(separated_pair(position, space1, velocity), opt(newline))(input)?;
    Ok((
        rem,
        Robot {
            pos: p,
            velocity: v,
        },
    ))
}

pub fn position(input: &str) -> IResult<&str, (usize, usize)> {
    let (input, _) = tag("p=")(input)?;
    let (rem, (x, y)) = separated_pair(digit1, tag(","), digit1)(input)?;
    Ok((
        rem,
        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()),
    ))
}

pub fn velocity(input: &str) -> IResult<&str, (isize, isize)> {
    let (input, _) = tag("v=")(input)?;
    separated_pair(signed_number, tag(","), signed_number)(input)
}

pub fn signed_number(input: &str) -> IResult<&str, isize> {
    let (input, string) = recognize(preceded(opt(tag("-")), digit1))(input)?;
    Ok((input, string.parse::<isize>().unwrap()))
}

pub struct Robot {
    pos: (usize, usize),
    velocity: (isize, isize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
