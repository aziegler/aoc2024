use std::cmp::min;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map_res, opt},
    multi::many1,
    sequence::{preceded, separated_pair, terminated, tuple},
    IResult,
};

advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<usize> {
    let (rem, mut machine) =
        many1(terminated(parse_claw, tuple((newline, newline))))(input).unwrap();
    let (_, last_machine) = parse_claw(rem).unwrap();
    machine.push(last_machine);
    let values: Vec<Option<usize>> = machine.iter().map(|m| compute_machine(m)).collect();
    Some(
        values
            .iter()
            .filter(|c| c.is_some())
            .map(|c| c.unwrap())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

pub fn compute_machine(machine: &ClawMachine) -> Option<usize> {
    let start_y = machine.prize.1 / &machine.b.1;
    let start_x: usize = machine.prize.0 / &machine.b.0;
    let start = min(start_x, start_y);

    let mut current_cost = None;
    for b in (0..start).rev() {
        let remaining_steps = machine.prize.1 - (b * machine.b.1);
        let modulo_a = remaining_steps % machine.a.1;

        if modulo_a == 0 {
            let a = remaining_steps / machine.a.1;
            if machine.prize.0 == (a * machine.a.0) + (b * machine.b.0) {
                current_cost = Some(current_cost.map_or(3 * a + b, |c| min(c, 3 * a + b)));
            }
        }
    }
    current_cost
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

pub fn parse_prize(input: &str) -> IResult<&str, (usize, usize)> {
    preceded(
        tag("Prize: X="),
        separated_pair(number, tag(", Y="), number),
    )(input)
}

pub fn parse_button<'a>(name: &str, input: &'a str) -> IResult<&'a str, (usize, usize)> {
    terminated(
        preceded(
            tuple((tag("Button "), tag(name), tag(": "))),
            separated_pair(parse_x, tag(", "), parse_y),
        ),
        opt(newline),
    )(input)
}

pub fn parse_x(input: &str) -> IResult<&str, usize> {
    preceded(tuple((tag("X"), tag("+"))), number)(input)
}

pub fn parse_y(input: &str) -> IResult<&str, usize> {
    preceded(tuple((tag("Y"), tag("+"))), number)(input)
}

pub fn number(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

pub struct ClawMachine {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
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
