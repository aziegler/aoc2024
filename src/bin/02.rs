advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    let datas = parse_file(input);
    Some(datas.iter().filter(|d| is_safe(d.as_slice())).count())
}

pub fn part_two(input: &str) -> Option<usize> {
    let datas = parse_file(input);
    Some(
        datas
            .iter()
            .filter(|d: &&Vec<i32>| is_safe_with_dampener(d.as_slice()))
            .count(),
    )
}

pub fn is_safe(data: &[i32]) -> bool {
    let (result, _) = data.windows(2).fold((true, 0), |(safe, sign), v| {
        if !safe
            || v[0].abs_diff(v[1]) > 3
            || v[0].abs_diff(v[1]) == 0
            || (sign != 0 && sign * (v[0] - v[1]) < 0)
        {
            return (false, sign);
        }

        return (true, v[0] - v[1]);
    });
    result
}

fn is_safe_with_dampener(data: &[i32]) -> bool {
    let sub_slices = (0..data.len()).map(|c| {
        let mut new = data.to_vec();
        new.remove(c);
        new
    });
    sub_slices.fold(is_safe(data), |s, ss| s || is_safe(&ss))
}

pub fn parse_file(input: &str) -> Vec<Vec<i32>> {
    input.lines().map(|l| parse_line(l)).collect()
}

pub fn parse_line(input: &str) -> Vec<i32> {
    input
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
    #[test]
    fn test_final() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(660));
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(689));
    }

    #[test]
    fn test_parse() {
        let result = parse_file(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            result,
            vec![
                [7, 6, 4, 2, 1],
                [1, 2, 7, 8, 9],
                [9, 7, 6, 2, 1],
                [1, 3, 2, 4, 5],
                [8, 6, 4, 4, 1],
                [1, 3, 6, 7, 9]
            ]
        );
    }
}
