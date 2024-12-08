use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    usize::MAX,
};

use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    compute(input, true)
}

pub fn part_two(input: &str) -> Option<usize> {
    compute(input, false)
}

fn compute(input: &str, part_one: bool) -> Option<usize> {
    let field = parse(input);
    let antinodes: HashSet<(usize, usize)> = field
        .antennas
        .iter()
        .map(|(c, locs)| {
            let set: HashSet<(usize, usize)> = locs
                .iter()
                .combinations(2)
                .flat_map(|v| {
                    let a1 = v.get(0).unwrap();
                    let a2 = v.get(1).unwrap();
                    find_antinodes(**a1, **a2, field.size, part_one)
                })
                .collect();
            set
        })
        .flatten()
        .filter(|(x, y)| *x < field.size.0 && *y < field.size.1)
        .collect();
    Some(antinodes.iter().len())
}

pub fn find_antinodes(
    a1: (usize, usize),
    a2: (usize, usize),
    limits: (usize, usize),
    limit_to_1: bool,
) -> Vec<(usize, usize)> {
    {
        let (x1, y1) = a1;
        let (x2, y2) = a2;
        let count = min(
            (limits.0).checked_div(x2.abs_diff(x1)).unwrap_or(MAX),
            (limits.1).checked_div(y2.abs_diff(y2)).unwrap_or(MAX),
        );
        let mut range = 0..count;
        if limit_to_1 {
            range = 1..2;
        }
        range
            .flat_map(|i| {
                vec![
                    (
                        x1.wrapping_add(i.wrapping_mul(x1.wrapping_sub(x2))),
                        y1.wrapping_add(i.wrapping_mul(y1.wrapping_sub(y2))),
                    ),
                    (
                        x2.wrapping_add(i.wrapping_mul(x2.wrapping_sub(x1))),
                        y2.wrapping_add(i.wrapping_mul(y2.wrapping_sub(y1))),
                    ),
                ]
            })
            .collect()
    }
}

pub fn parse(input: &str) -> Field {
    let mut field: Field = Field {
        antennas: HashMap::new(),
        size: (0, 0),
    };
    for (y, line) in input.lines().enumerate() {
        if line.len() == 0 {
            break;
        }
        field.size.1 += 1;
        for (x, char) in line.chars().enumerate() {
            field.size.0 = line.len();
            if char != '.' {
                field
                    .antennas
                    .entry(char)
                    .or_insert(Vec::new())
                    .push((x, y));
            }
        }
    }
    field
}

#[derive(Debug)]
pub struct Field {
    size: (usize, usize),
    antennas: HashMap<char, Vec<(usize, usize)>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
