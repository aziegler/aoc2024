use std::usize;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, line_ending, newline},
    combinator::map_res,
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u64> {
    compute_sum(input, vec![Operators::PLUS, Operators::MUL])
}

pub fn part_two(input: &str) -> Option<u64> {
    compute_sum(
        input,
        vec![Operators::PLUS, Operators::MUL, Operators::CONCAT],
    )
}

fn compute_sum(input: &str, possible_ops: Vec<Operators>) -> Option<u64> {
    let input = parse(input).unwrap().1;
    Some(
        input
            .par_iter()
            .filter(|l| is_possible(&l, &possible_ops))
            .map(|d| d.result)
            .sum(),
    )
}

fn is_possible(input: &&Data, possible_ops: &Vec<Operators>) -> bool {
    let ops_list = generate_combination(possible_ops, input.slots());
    let result = ops_list.par_iter().find_any(|l| is_valid(input, l));
    result.is_some()
}

fn generate_combination(possible_ops: &Vec<Operators>, size: usize) -> Vec<Vec<Operators>> {
    let mut result: Vec<Vec<Operators>> = vec![vec![]];
    for _ in 0..size {
        result = step_generate_ops_combination(&result, possible_ops);
    }
    result
}

fn step_generate_ops_combination(
    acc: &Vec<Vec<Operators>>,
    possible_ops: &Vec<Operators>,
) -> Vec<Vec<Operators>> {
    let mut result: Vec<Vec<Operators>> = Vec::new();
    for op in possible_ops {
        for current_list in acc {
            let mut new_list = current_list.clone();
            new_list.push(*op);
            result.push(new_list.to_vec());
        }
    }
    result
}

pub fn is_valid(data: &Data, ops: &Vec<Operators>) -> bool {
    let result = compute(data, ops);
    //  println!("Applied {:?} found {}", ops, result.abs_diff(data.result));
    result == data.result
}

pub fn compute(data: &Data, ops: &Vec<Operators>) -> u64 {
    let init = data.inputs[0];
    data.inputs[1..]
        .iter()
        .zip(ops)
        .fold(init, |acc: u64, (v, o)| match o {
            Operators::PLUS => acc.saturating_add(*v),
            Operators::MUL => acc.saturating_mul(*v),
            Operators::CONCAT => acc * 10u64.pow(v.ilog10() + 1) + v,
        })
}

#[derive(Debug, Clone, Copy)]
pub enum Operators {
    PLUS,
    MUL,
    CONCAT,
}

#[derive(Debug)]
pub struct Data {
    inputs: Vec<u64>,
    result: u64,
}

impl Data {
    pub fn slots(self: &Data) -> usize {
        return self.inputs.len() - 1;
    }
}

fn number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, |s: &str| s.parse::<u64>())(input)
}

pub fn parse_line(input: &str) -> IResult<&str, Data> {
    map_res(
        separated_pair(number, tag(": "), separated_list1(tag(" "), number)),
        |(result, inputs)| -> Result<Data, nom::error::Error<&str>> {
            Ok(Data {
                result: result,
                inputs: inputs,
            })
        },
    )(input)
}

pub fn parse(input: &str) -> IResult<&str, Vec<Data>> {
    separated_list1(line_ending, parse_line)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
