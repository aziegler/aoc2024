use std::{cmp::min, usize::MAX};

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map_res, opt},
    multi::many1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<isize> {
    let (rem, mut machine) =
        many1(terminated(parse_claw, tuple((newline, newline))))(input).unwrap();
    let (_, last_machine) = parse_claw(rem).unwrap();
    machine.push(last_machine);
    let values: Vec<Option<isize>> = machine.iter().map(|m| solve_machine(m, 0)).collect();
    Some(
        values
            .iter()
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<isize> {
    let (rem, mut machine) =
        many1(terminated(parse_claw, tuple((newline, newline))))(input).unwrap();
    let (_, last_machine) = parse_claw(rem).unwrap();
    machine.push(last_machine);
    let values: Vec<Option<isize>> = machine
        .iter()
        .map(|m| solve_machine(m, 10000000000000))
        .collect();
    Some(
        values
            .iter()
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .sum(),
    )
}
fn solve_machine(machine: &ClawMachine, offset: isize) -> Option<isize> {
    let prize = (machine.prize.0 + offset, machine.prize.1 + offset);
    let det = machine.a.0 * machine.b.1 - machine.a.1 * machine.b.0;
    let a = (prize.0 * machine.b.1 - prize.1 * machine.b.0) / det;
    let b = (machine.a.0 * prize.1 - machine.a.1 * prize.0) / det;
    if (
        machine.a.0 * a + machine.b.0 * b,
        machine.a.1 * a + machine.b.1 * b,
    ) == (prize.0, prize.1)
    {
        Some(a * 3 + b)
    } else {
        None
    }
}

pub fn parse_claw(input: &str) -> IResult<&str, ClawMachine> {
    let (input, a) = parse_button("A", input).unwrap();
    let (input, b) = parse_button("B", input).unwrap();
    let (rem, prize) = parse_prize(input).unwrap();
    return Ok((
        rem,
        ClawMachine {
            a: a,
            b: b,
            prize: prize,
        },
    ));
}

pub fn parse_prize(input: &str) -> IResult<&str, (isize, isize)> {
    preceded(
        tag("Prize: X="),
        separated_pair(number, tag(", Y="), number),
    )(input)
}

pub fn parse_button<'a>(name: &str, input: &'a str) -> IResult<&'a str, (isize, isize)> {
    terminated(
        preceded(
            tuple((tag("Button "), tag(name), tag(": "))),
            separated_pair(parse_x, tag(", "), parse_y),
        ),
        opt(newline),
    )(input)
}

pub fn parse_x(input: &str) -> IResult<&str, isize> {
    preceded(tuple((tag("X"), tag("+"))), number)(input)
}

pub fn parse_y(input: &str) -> IResult<&str, isize> {
    preceded(tuple((tag("Y"), tag("+"))), number)(input)
}

pub fn number(input: &str) -> IResult<&str, isize> {
    map_res(digit1, |s: &str| s.parse::<isize>())(input)
}

pub struct ClawMachine {
    a: (isize, isize),
    b: (isize, isize),
    prize: (isize, isize),
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
