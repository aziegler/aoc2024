use std::{
    collections::{HashMap, HashSet},
    usize,
};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let regions = split_into_regions(parse(input));
    Some(regions.iter().map(|r| size(r)).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let regions = split_into_regions(parse(input));
    Some(regions.iter().map(|r| size_part_two(r)).sum())
}

pub fn size(region: &Vec<(usize, usize)>) -> usize {
    let size: usize = region
        .iter()
        .map(|(x, y)| {
            4 - region
                .iter()
                .filter(|(x1, y1)| x1.abs_diff(*x) + y1.abs_diff(*y) == 1)
                .count()
        })
        .sum();
    size * region.len()
}

pub fn size_part_two(region: &Vec<(usize, usize)>) -> usize {
    let horizontal_sides = get_sides(&region.clone(), true);
    let vertical_sides = get_sides(&region.clone(), false);
    println!(
        "Region {:?} has {} horizontal sides and {} vertical sides",
        region,
        horizontal_sides.len(),
        vertical_sides.len()
    );
    horizontal_sides.len() * region.len() + vertical_sides.len() * region.len()
}

pub fn get_sides(borders: &Vec<(usize, usize)>, horizontal: bool) -> Vec<Vec<(usize, usize)>> {
    let mut sides: Vec<Vec<(usize, usize)>> = Vec::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    for border in borders {
        if visited.contains(border) {
            continue;
        }
        let side = &mut Vec::new();
        let side = compute_side(*border, side, &mut borders.clone(), horizontal);
        visited.extend(side.clone());
        sides.push(side.clone());
    }
    sides
}

pub fn compute_side<'a>(
    point: (usize, usize),
    current_side: &'a mut Vec<(usize, usize)>,
    border: &mut Vec<(usize, usize)>,
    horizontal: bool,
) -> &'a Vec<(usize, usize)> {
    current_side.push(point);

    let neighbors: Vec<(usize, usize)>;
    if horizontal {
        neighbors = get_horizontal_neighbors(point);
    } else {
        neighbors = get_vertical_neighbors(point);
    }

    for neighbor in neighbors {
        let pos = border.iter().position(|c| c == &neighbor);
        if pos.is_some() {
            current_side.push(neighbor);
            let neighbor = border.remove(pos.unwrap());
            compute_side(neighbor, current_side, border, horizontal);
        }
    }
    current_side
}

fn get_vertical_neighbors((x, y): (usize, usize)) -> Vec<(usize, usize)> {
    vec![(x.wrapping_sub(1), y), (x + 1, y)]
}

fn get_horizontal_neighbors((x, y): (usize, usize)) -> Vec<(usize, usize)> {
    vec![(x, y.wrapping_sub(1)), (x, y + 1)]
}

pub fn split_into_regions(map: Vec<Vec<char>>) -> Vec<Vec<(usize, usize)>> {
    let mut regions: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    map.iter().enumerate().for_each(|(y, line)| {
        line.iter().enumerate().for_each(|(x, c)| {
            regions.entry(*c).or_default().push((x, y));
        })
    });

    regions
        .into_iter()
        .flat_map(|(_, region)| split_region(region))
        .collect()
}

fn split_region(region: Vec<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
    let mut visited = HashSet::new();
    let mut components = Vec::new();

    for &cell in &region {
        if !visited.contains(&cell) {
            let component = find_component(&region, cell, &mut visited);
            components.push(component);
        }
    }

    components
}

fn find_component(
    region: &Vec<(usize, usize)>,
    start: (usize, usize),
    visited: &mut HashSet<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let mut component = Vec::new();
    let mut queue = Vec::new();
    queue.push(start);

    while let Some(cell) = queue.pop() {
        if visited.insert(cell) {
            component.push(cell);

            for &neighbor in &get_neighbors(cell) {
                if region.contains(&neighbor) && !visited.contains(&neighbor) {
                    queue.push(neighbor);
                }
            }
        }
    }
    component
}

fn get_neighbors((x, y): (usize, usize)) -> Vec<(usize, usize)> {
    vec![
        (x.wrapping_sub(1), y),
        (x + 1, y),
        (x, y.wrapping_sub(1)),
        (x, y + 1),
    ]
}

pub fn parse(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|c| c.chars().collect()).collect()
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
