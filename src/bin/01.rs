use std::collections::{HashMap, HashSet};

use num::abs;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let (mut left_list, mut right_list) = parse_file(input);
    left_list.sort();
    right_list.sort();
    let mut result = 0;
    for i in 0..left_list.len() {
        result += abs(left_list[i] - right_list[i]);
    }
    Some(result as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (left_list, right_list) = parse_file(input);
    let mut values: HashMap<i32, i32> = HashMap::new();
    let mut already_seen: HashSet<i32> = HashSet::new();
    let mut result = 0;
    for value in right_list.iter() {
        *values.entry(*value).or_default() += 1;
    }
    for value in left_list.iter() {
        if already_seen.contains(value) || !values.contains_key(value) {
            continue;
        }
        result += *value * values.get(value).unwrap();
        already_seen.insert(*value);
    }

    Some(result as u32)
}

fn parse_file(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    for line in input.lines() {
        if line.trim().is_empty() {
            break;
        }
        let mut parsing = line.split_whitespace();
        left_list.push(parsing.next().unwrap().trim().parse::<i32>().unwrap());
        right_list.push(parsing.next().unwrap().trim().parse::<i32>().unwrap());
    }
    (left_list, right_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_final() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1580061));
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(23046913));
    }
}
