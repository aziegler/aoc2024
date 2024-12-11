use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    let mut init_stones = parse(input);
    for _ in 0..25 {
        init_stones = step(&mut init_stones);
    }
    Some(init_stones.len())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut map: HashMap<u64, u64> =
        parse(input)
            .iter()
            .fold(HashMap::new(), |mut map: HashMap<u64, u64>, c| {
                *map.entry(*c).or_default() += 1;
                map
            });
    for _ in 0..75 {
        map = step_map(map);
    }
    let mut memo: HashMap<(u64, u32), u64> = HashMap::new();
    let init_stones = parse(input);
    let mut sum = 0;
    for stone in init_stones {
        sum += with_memo(stone, 75, &mut memo);
    }
    println!("{}", sum);
    Some(map.values().sum())
}

pub fn with_memo(value: u64, step: u32, mut memo: &mut HashMap<(u64, u32), u64>) -> u64 {
    if step == 0 {
        return 1;
    }
    if memo.get(&(value, step)).is_some() {
        return *memo.get(&(value, step)).unwrap();
    }
    if value == 0 {
        return with_memo(1, step - 1, memo);
    }
    let digit_count = (value.ilog10() + 1) as usize;
    let result;
    if digit_count % 2 == 0 {
        let k1 = value / 10u64.pow((digit_count / 2) as u32);
        let k2 = value % 10u64.pow((digit_count / 2) as u32);
        result = with_memo(k1, step - 1, &mut memo) + with_memo(k2, step - 1, &mut memo);
    } else {
        result = with_memo(value * 2024, step - 1, &mut memo)
    }
    memo.insert((value, step), result);
    result
}

pub fn step_map(map: HashMap<u64, u64>) -> HashMap<u64, u64> {
    let mut next_map = HashMap::new();
    map.iter().for_each(|(k, c)| {
        if *k == 0 {
            *next_map.entry(1).or_insert(0) += c;
        } else {
            let digit_count = (k.ilog10() + 1) as usize;
            if digit_count % 2 == 0 {
                let k1 = k / 10u64.pow((digit_count / 2) as u32);
                let k2 = k % 10u64.pow((digit_count / 2) as u32);
                *next_map.entry(k1).or_insert(0) += c;
                *next_map.entry(k2).or_insert(0) += c;
            } else {
                *next_map.entry(k * 2024).or_insert(0) += c;
            }
        }
    });
    next_map
}

pub fn step(state: &mut Vec<u64>) -> Vec<u64> {
    state
        .iter()
        .map(|s| {
            let mut vec = Vec::new();
            if *s == 0 {
                vec.push(1);
                return vec;
            }
            let string_value = s.to_string();
            if string_value.len() % 2 == 0 {
                vec.push(
                    string_value[0..string_value.len() / 2]
                        .parse::<u64>()
                        .unwrap(),
                );
                vec.push(
                    string_value[string_value.len() / 2..]
                        .parse::<u64>()
                        .unwrap(),
                );
                return vec;
            }
            vec.push(s * 2024);
            return vec;
        })
        .flatten()
        .collect()
}

pub fn parse(input: &str) -> Vec<u64> {
    input
        .split_ascii_whitespace()
        .map(|c| c.trim().parse::<u64>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(55312));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
