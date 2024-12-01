use std::collections::HashMap;

use num::abs;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let (mut left_list, mut right_list) = parse_file(input);
    left_list.sort();
    right_list.sort();
    Some(
        left_list
            .iter()
            .zip(right_list)
            .fold(0, |acc, (left, right)| (acc + abs(left - right))),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let (left_list, right_list) = parse_file(input);
    let right_list_count =
        right_list
            .iter()
            .fold(HashMap::new(), |mut map: HashMap<i32, i32>, value| {
                *map.entry(*value).or_default() += 1;
                map
            });

    Some(left_list.iter().fold(0, |acc, value| {
        acc + *value * right_list_count.get(value).unwrap_or(&0)
    }))
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
        assert_eq!(result, Some(31));
    }

    #[test]
    fn test_final() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(1580061));
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(23046913));
    }
}
