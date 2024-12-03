use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map_res,
    sequence::{delimited, separated_pair},
    IResult,
};

advent_of_code::solution!(3);

pub fn part_one(mut input: &str) -> Option<u32> {
    let mut result = 0;
    loop {
        if input.len() == 0 {
            return Some(result);
        }
        match mul(input) {
            Ok((rest, Instruction::Mul(value))) => {
                result += value;

                input = rest;
            }
            _ => {
                input = &input[1..];
            }
        }
    }
}

pub fn part_two(mut input: &str) -> Option<u32> {
    let mut result = 0;
    let mut enabled: bool = true;
    loop {
        if input.len() == 0 {
            return Some(result);
        }
        match parse_instruction(input) {
            Ok((rest, Instruction::Mul(value))) => {
                if enabled {
                    result += value;
                }
                input = rest;
            }
            Ok((rest, Instruction::Enabled(value))) => {
                enabled = value;
                input = rest;
            }
            Err(_) => {
                input = &input[1..];
            }
        }
    }
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_do_dont, mul))(input)
}

enum Instruction {
    Enabled(bool),
    Mul(u32),
}

fn parse_do_dont(input: &str) -> IResult<&str, Instruction> {
    alt((
        map_res(tag("do()"), |_| {
            Ok::<Instruction, nom::error::Error<&str>>(Instruction::Enabled(true))
        }),
        map_res(tag("don't()"), |_| {
            Ok::<Instruction, nom::error::Error<&str>>(Instruction::Enabled(false))
        }),
    ))(input)
}

fn mul(input: &str) -> IResult<&str, Instruction> {
    map_res(
        delimited(
            tag("mul("),
            separated_pair(number, tag(","), number),
            tag(")"),
        ),
        |(a, b)| Ok::<Instruction, nom::error::Error<&str>>(Instruction::Mul(a * b)),
    )(input)
}

fn number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
