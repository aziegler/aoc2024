use std::{
    collections::{HashMap, HashSet, VecDeque},
    ops::BitXor,
    u128,
};

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u128> {
    let input_numbers = input.lines().map(|c| c.parse::<u128>().unwrap());
    let result = input_numbers
        .map(|c| compute_iter(c, 2000, &mut HashMap::new()))
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u16> {
    let input_numbers = input.lines().map(|c| c.parse::<u128>().unwrap());
    let values = &mut HashMap::new();
    let _ = input_numbers.for_each(|c| {
        compute_iter(c, 2000, values);
    });
    let mut max = 0;
    let mut result = (0, 0, 0, 0);
    for (k, v) in values {
        if *v > max {
            result = *k;
            max = *v;
        }
    }
    println!("{:?}", result);
    Some(max)
}

pub fn compute_iter(
    mut secret: u128,
    iteration: u16,
    values: &mut HashMap<(i16, i16, i16, i16), u16>,
) -> u128 {
    let init_secret = secret;
    let mut change_list = VecDeque::new();
    let mut visited: HashSet<(i16, i16, i16, i16)> = HashSet::new();
    for i in (0..3) {
        let (new_secret, _, change) = compute_next(secret);
        secret = new_secret;
        change_list.push_back(change);
    }

    for i in (3..iteration) {
        let (new_secret, value, change) = compute_next(secret);
        change_list.push_back(change);
        let test = (
            change_list[0],
            change_list[1],
            change_list[2],
            change_list[3],
        );
        secret = new_secret;
        change_list.pop_front();
        if visited.contains(&test) {
            continue;
        }
        *values.entry(test).or_default() += value;
        visited.insert(test);
    }
    secret
}

pub fn compute_next(orig_secret: u128) -> (u128, u16, i16) {
    let mut secret = (orig_secret * 64) ^ orig_secret;
    secret = secret % 16777216;
    secret = (secret / 32) ^ secret;
    secret = secret % 16777216;
    secret = (secret * 2048) ^ secret;
    secret = secret % 16777216;
    (
        secret,
        (secret % 10) as u16,
        ((secret % 10) as i16 - (orig_secret % 10) as i16),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = compute_next(123);
        assert_eq!(result, (15887950, 0, 0));
    }

    #[test]
    fn test_iter() {
        let result: Vec<u128> = vec![1, 10, 100, 2024]
            .iter()
            .map(|c| compute_iter(*c, 2000, &mut HashMap::new()))
            .collect();
        assert_eq!(result, vec![8685429, 4700978, 15273692, 8667524]);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(23));
    }
}
