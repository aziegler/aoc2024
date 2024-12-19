use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, newline},
    combinator::{map_res, opt},
    multi::{many1, separated_list1},
    sequence::{preceded, separated_pair, terminated},
    IResult,
};
use num::Integer;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let (rem, registers) = many1(register)(input).unwrap();
    let (rem, _) = newline::<_, nom::error::Error<_>>(rem).unwrap();
    let (rem, program) = program(rem).unwrap();
    let mut registers: HashMap<&str, u64> = registers.iter().map(|c| *c).collect();
    let mut addr = 0;
    let mut result_list: Vec<u64> = Vec::new();
    while (addr < program.len()) {
        let (next, result) = run_instruction(&mut registers, &program, addr);
        if result.is_some() {
            result_list.push(result.unwrap());
        }
        addr = next;
    }
    let results: String = result_list
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(",");
    Some(results)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (rem, registers) = many1(register)(input).unwrap();
    let (rem, _) = newline::<_, nom::error::Error<_>>(rem).unwrap();
    let (rem, program) = program(rem).unwrap();
    let registers: HashMap<&str, u64> = registers.iter().map(|c| *c).collect();
    let test_value = program
        .iter()
        .flat_map(|(i, j)| vec![*i as u64, *j as u64])
        .collect::<Vec<u64>>();
    let mut value: u64 = 37213107740000;
    loop {
        let mut register_impl = registers.clone();
        register_impl.insert("A", value);
        let result = run_program(&mut register_impl, &program);
        if result == test_value {
            return Some(value);
        }
        if value % 10000 == 0 {
            println!("{} : {:?}", value, result);
        }
        if result.len() < 16 {
            value += 100000;
        }
        if result[12..] == vec![0, 3, 3, 0] {
            if result[8..] == vec![1, 3, 5, 5, 0, 3, 3, 0] {
                if result[4..] == vec![7, 5, 4, 7, 1, 3, 5, 5, 0, 3, 3, 0] {
                    println!("Reached {:?} at {}", result, value);
                    value += 1;
                } else {
                    value += 10000;
                }
            } else {
                value += 100000;
            }
        } else {
            value += 10000000;
        }
    }
}

pub fn run_program(mut registers: &mut HashMap<&str, u64>, program: &Vec<(u8, u8)>) -> Vec<u64> {
    let mut addr = 0;
    let mut result_list: Vec<u64> = Vec::new();
    while (addr < program.len()) {
        let (next, result) = run_instruction(&mut registers, &program, addr);
        if result.is_some() {
            result_list.push(result.unwrap());
        }
        addr = next;
    }
    result_list
}

pub fn eval_operand(registers: &HashMap<&str, u64>, opcode: u8) -> u64 {
    match opcode {
        0 => 0,
        1 => 1,
        2 => 2,
        3 => 3,
        4 => *registers.get("A").unwrap(),
        5 => *registers.get("B").unwrap(),
        6 => *registers.get("C").unwrap(),
        _ => panic!("Invalid opcode"),
    }
}

pub fn run_instruction(
    registers: &mut HashMap<&str, u64>,
    program: &Vec<(u8, u8)>,
    idx: usize,
) -> (usize, Option<u64>) {
    let (instr, opcode) = program.get(idx).unwrap();
    match instr {
        0 => {
            registers.insert(
                "A",
                registers
                    .get("A")
                    .unwrap()
                    .div_floor(&2u64.pow(eval_operand(registers, *opcode).try_into().unwrap())),
            );
            (idx + 1, None)
        }
        1 => {
            registers.insert(
                "B",
                registers.get("B").unwrap() ^ (eval_operand(registers, *opcode)),
            );
            (idx + 1, None)
        }
        2 => {
            registers.insert("B", eval_operand(registers, *opcode) % 8);
            (idx + 1, None)
        }
        3 => {
            let jmp = registers.get("A").unwrap();
            if *jmp == 0 {
                return (idx + 1, None);
            } else {
                return (eval_operand(registers, *opcode) as usize, None);
            }
        }
        4 => {
            registers.insert(
                "B",
                registers.get("B").unwrap() ^ registers.get("C").unwrap(),
            );
            (idx + 1, None)
        }
        5 => (idx + 1, Some(eval_operand(registers, *opcode) % 8)),
        6 => {
            registers.insert(
                "B",
                registers
                    .get("A")
                    .unwrap()
                    .div_floor(&2u64.pow(eval_operand(registers, *opcode).try_into().unwrap())),
            );
            (idx + 1, None)
        }
        7 => {
            registers.insert(
                "C",
                registers
                    .get("A")
                    .unwrap()
                    .div_floor(&2u64.pow(eval_operand(registers, *opcode).try_into().unwrap())),
            );
            (idx + 1, None)
        }
        _ => panic!("Invalid instruction"),
    }
}

pub fn program(input: &str) -> IResult<&str, Vec<(u8, u8)>> {
    preceded(
        tag("Program: "),
        separated_list1(
            tag(","),
            separated_pair(
                map_res(digit1, |c: &str| c.parse::<u8>()),
                tag(","),
                map_res(digit1, |c: &str| c.parse::<u8>()),
            ),
        ),
    )(input)
}

pub fn register(input: &str) -> IResult<&str, (&str, u64)> {
    let (rem, _) = tag("Register ")(input)?;
    let (rem, reg_name) = alt((tag("A"), tag("B"), tag("C")))(rem)?;
    let (rem, _) = tag(": ")(rem)?;
    let (rem, value) = map_res(digit1, |c: &str| c.parse::<u64>())(rem)?;
    let (rem, _) = newline(rem)?;
    Ok((rem, (reg_name, value)))
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
