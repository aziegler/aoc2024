use itertools::Itertools;

advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u32> {
    let vec = parse(input);
    let starting_points = (0..vec.len()).cartesian_product(0..vec.get(0).unwrap().len());
    let steps = (-1..2).cartesian_product(-1..2);
    let values = starting_points.cartesian_product(steps);

    let mut count = 0;

    for ((x, y), (i, j)) in values {
        if test_direction_ugly(&vec, x, y, i, j) {
            count += 1;
        }
    }
    Some(count)
}

pub fn part_two(input: &str) -> Option<u32> {
    let vec = parse(input);
    let starting_points = (0..vec.len()).cartesian_product(0..vec.get(0).unwrap().len());

    let mut count = 0;

    for (x, y) in starting_points {
        if test_x_mas(&vec, x, y) {
            count += 1;
        }
    }
    Some(count)
}

//NOTE : returning ' ' works because never in the solution so guaranteed to fail.
//It's horrible but I'm bored
pub fn access(input: &Vec<Vec<char>>, i: usize, j: usize) -> char {
    input
        .get(i)
        .unwrap_or(&vec![' '])
        .get(j)
        .unwrap_or(&' ')
        .clone()
}

//NOTE: overflowing works because access will recover and properly fail test. That's still bad, I'm still bored
fn add(u: usize, i: i32) -> usize {
    if i.is_negative() {
        u.overflowing_sub(i.abs() as usize).0
    } else {
        u + i as usize
    }
}

pub fn test_x_mas(input: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let char = access(input, x, y);
    if char != 'A' {
        return false;
    }
    let mut diag_1 = vec![
        access(input, add(x, -1), add(y, -1)),
        access(input, add(x, 1), add(y, 1)),
    ];
    diag_1.sort();
    let mut diag_2 = vec![
        access(input, add(x, -1), add(y, 1)),
        access(input, add(x, 1), add(y, -1)),
    ];
    diag_2.sort();

    return (diag_1 == vec!['M', 'S']) && (diag_2 == vec!['M', 'S']);
}

pub fn test_direction_ugly(
    input: &Vec<Vec<char>>,
    x: usize,
    y: usize,
    incr_x: i32,
    incr_y: i32,
) -> bool {
    let to_test: Vec<char> = "XMAS".chars().collect();

    for idx in 0..4 {
        let i = add(x, idx * incr_x);
        let j = add(y, idx * incr_y);
        if !(*to_test.get(idx as usize).unwrap() == access(input, i, j)) {
            return false;
        }
    }

    return true;
}

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|f| f.chars().collect()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
