use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<usize> {
    let mut disk = parse(input);
    let mut done: bool = false;
    let mut disk_copy = disk
        .clone()
        .into_iter()
        .enumerate()
        .rev()
        .filter(|(_, v)| v.is_some())
        .peekable();
    disk.iter_mut().enumerate().for_each(|(k, v)| match v {
        Some(_) => {
            let (idx, _) = disk_copy.peek().unwrap();
            if done || k > *idx {
                done = true;
                *v = None;
            }
        }
        None => {
            if !done {
                let (_, value) = disk_copy.next().unwrap();
                *v = value;
            }
        }
    });
    Some(check_sum(disk))
}

pub fn part_two(input: &str) -> Option<usize> {
    let (mut files, mut freespace) = parse_as_block(input);
    files.clone().keys().sorted().rev().for_each(|f| {
        let current_slot = files.get(f).unwrap();
        let possible_pos = find_possible_pos(&freespace, current_slot.length);
        if possible_pos.is_some() && possible_pos.unwrap().1.begin < current_slot.begin {
            let (idx, new_pos) = possible_pos.unwrap();
            let mut pos_to_insert = new_pos.clone();
            let mut remaining_space: Option<DiskSlot> = None;
            if new_pos.length > current_slot.length {
                pos_to_insert = DiskSlot {
                    begin: new_pos.begin,
                    length: current_slot.length,
                };
                remaining_space = Some(DiskSlot {
                    begin: new_pos.begin + current_slot.length,
                    length: new_pos.length - current_slot.length,
                });
            }
            files.entry(*f).and_modify(|v| *v = pos_to_insert);
            freespace.remove(idx);
            if remaining_space.is_some() {
                freespace.insert(idx, remaining_space.unwrap());
            }
        }
    });
    Some(check_sum_block(files))
}

fn find_possible_pos(freespace: &Vec<DiskSlot>, size: u32) -> Option<(usize, &DiskSlot)> {
    freespace
        .iter()
        .enumerate()
        .find(|(_, slot)| slot.length >= size)
}

fn check_sum_block(files: HashMap<usize, DiskSlot>) -> usize {
    files
        .iter()
        .map(|(id, slot)| {
            slot.length as usize * id * (2 * slot.begin + slot.length - 1) as usize / 2
        })
        .sum()
}

pub fn check_sum(disk: Vec<Option<usize>>) -> usize {
    disk.iter()
        .enumerate()
        .map(|(k, v)| k * v.unwrap_or(0))
        .sum()
}

fn parse_as_block(input: &str) -> (HashMap<usize, DiskSlot>, Vec<DiskSlot>) {
    let mut files = HashMap::new();
    let mut freespace: Vec<DiskSlot> = Vec::new();
    let mut current_idx = 0;
    input.chars().enumerate().for_each(|(idx, c)| {
        let count = c.to_string().parse::<u32>().unwrap();
        match idx % 2 {
            0 => {
                files.insert(
                    idx / 2,
                    DiskSlot {
                        begin: current_idx,
                        length: count,
                    },
                );
            }
            _ => {
                freespace.push(DiskSlot {
                    begin: current_idx,
                    length: count,
                });
            }
        }
        current_idx = current_idx + count;
    });
    (files, freespace)
}

pub fn parse(input: &str) -> Vec<Option<usize>> {
    let mut result = Vec::new();
    input.chars().enumerate().for_each(|(idx, c)| {
        let count = c.to_string().parse::<u32>().unwrap();
        match idx % 2 {
            0 => (0..count).for_each(|_| {
                result.push(Some(idx / 2));
            }),
            _ => (0..count).for_each(|_| {
                result.push(None);
            }),
        }
    });
    result
}

#[derive(Clone)]
struct DiskSlot {
    begin: u32,
    length: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
