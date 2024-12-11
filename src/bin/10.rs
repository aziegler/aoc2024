use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<usize> {
    let map = parse(input);
    let size = (map.get(0).unwrap().len(), map.len());
    let mut sum = 0;
    for x in 0..size.1 {
        for y in 0..size.0 {
            let head = reachable_9(&map, &HashSet::new(), x, y, 0, size);
            if head.len() > 0 {
                sum += head.len();
            }
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = parse(input);
    let size = (map.get(0).unwrap().len(), map.len());
    let mut sum = 0;
    for x in 0..size.1 {
        for y in 0..size.0 {
            sum += valid_paths(&map, x, y, 0, size);
        }
    }
    Some(sum)
}

pub fn valid_paths(map: &Vec<Vec<u16>>, x: usize, y: usize, h: u16, size: (usize, usize)) -> u32 {
    if *map.get(y).unwrap().get(x).unwrap() != h {
        return 0;
    }
    if h == 9 {
        return 1;
    }
    let mut sum = 0;
    if x > 0 {
        sum += valid_paths(map, x - 1, y, h + 1, size);
    }
    if y > 0 {
        sum += valid_paths(map, x, y - 1, h + 1, size);
    }
    if x < size.0 - 1 {
        sum += valid_paths(map, x + 1, y, h + 1, size);
    }
    if y < size.1 - 1 {
        sum += valid_paths(map, x, y + 1, h + 1, size);
    }
    sum
}

pub fn reachable_9<'a>(
    map: &Vec<Vec<u16>>,
    reached: &HashSet<(usize, usize)>,
    x: usize,
    y: usize,
    h: u16,
    size: (usize, usize),
) -> HashSet<(usize, usize)> {
    if *map.get(y).unwrap().get(x).unwrap() != h {
        return reached.clone();
    }
    if h == 9 {
        let mut val = reached.clone();
        val.insert((x, y));
        return val;
    }
    let mut val = reached.clone();
    if x > 0 {
        val.extend(reachable_9(map, reached, x - 1, y, h + 1, size));
    }
    if y > 0 {
        val.extend(reachable_9(map, reached, x, y - 1, h + 1, size));
    }
    if x < size.0 - 1 {
        val.extend(reachable_9(map, reached, x + 1, y, h + 1, size));
    }
    if y < size.1 - 1 {
        val.extend(reachable_9(map, reached, x, y + 1, h + 1, size));
    }
    val
}

pub fn parse(input: &str) -> Vec<Vec<u16>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_string().parse::<u16>().unwrap())
                .collect()
        })
        .collect()
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
