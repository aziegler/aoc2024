use std::{cmp::Ordering, collections::HashMap, usize::MAX};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, newline},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let input_values = parse_input(input).unwrap().1;
    Some(
        input_values
            .issues
            .iter()
            .filter(|x| check_issue(x, input_values.clone().update))
            .map(|x| take_middle(x))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_values = parse_input(input).unwrap().1;
    Some(
        input_values
            .issues
            .iter()
            .filter(|x| !check_issue(x, input_values.clone().update))
            .map(|x| reorder(x.clone(), input_values.clone().update))
            .map(|x| take_middle(&x))
            .sum(),
    )
}

pub fn take_middle(input: &Vec<u32>) -> u32 {
    *input.get(input.len() / 2).unwrap()
}

// That yields the right result and si (obviously) much faster but feel very wrong. Maybe think if it could be right ?
pub fn check_issue(issue: &Vec<u32>, rules: Vec<(u32, u32)>) -> bool {
    issue
        .windows(2)
        .all(|w| order(w[0], w[1], &rules) != Ordering::Greater)
}

pub fn check_rule(mut issue: Vec<u32>, rule: (u32, u32)) -> bool {
    let first = issue.iter().position(|n| *n == rule.0);
    issue.reverse();
    let second = issue.iter().position(|n| *n == rule.1);
    first.is_none() || second.is_none() || first.unwrap() < (issue.len() - second.unwrap() - 1)
}

pub fn reorder(mut issue: Vec<u32>, rules: Vec<(u32, u32)>) -> Vec<u32> {
    issue.sort_by(|a, b| order(*a, *b, &rules));
    issue
}

pub fn order(a: u32, b: u32, rules: &Vec<(u32, u32)>) -> Ordering {
    let find = rules.iter().find(|v| **v == (a, b) || **v == (b, a));
    if find.unwrap().0 == a {
        return Ordering::Less;
    }
    return Ordering::Greater;
}

pub fn parse_input(input: &str) -> IResult<&str, Input> {
    let value = map_res(
        separated_pair(many1(update), line_ending, many1(issues)),
        |f| {
            Ok::<_, nom::error::Error<&str>>(Input {
                update: f.0,
                issues: f.1,
            })
        },
    )(input);
    value
}

pub fn issues(input: &str) -> IResult<&str, Vec<u32>> {
    terminated(separated_list1(tag(","), number), line_ending)(input)
}

pub fn update(input: &str) -> IResult<&str, (u32, u32)> {
    terminated(separated_pair(number, tag("|"), number), line_ending)(input)
}

fn number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

#[derive(Clone)]
pub struct Input {
    update: Vec<(u32, u32)>,
    issues: Vec<Vec<u32>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(76));
    }
}
