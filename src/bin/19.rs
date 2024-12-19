use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    multi::separated_list1,
    sequence::{separated_pair, tuple},
    IResult,
};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u64> {
    let (towels, targets) = pattern(input).unwrap().1;
    let mut trie = TrieNode {
        is_end: false,
        childs: HashMap::new(),
    };
    towels.iter().enumerate().for_each(|(i, prefix)| {
        println!("Parsing {} idx : {}", prefix, i);
        add_to_trie(prefix, &mut trie);
    });
    Some(
        targets
            .iter()
            .map(|t| recursive_find(t, &trie.childs, &trie, &mut HashMap::new()))
            .sum(),
    )
}

pub fn recursive_find(
    word: &str,
    sub_trie: &HashMap<char, TrieNode>,
    whole_trie: &TrieNode,
    eliminated: &mut HashMap<String, u64>,
) -> u64 {
    let next = sub_trie.get(&word.chars().nth(0).unwrap());
    if next.is_none() {
        return 0;
    }
    let next_node = next.unwrap();
    if word.len() == 1 {
        if next_node.is_end {
            return 1;
        } else {
            return 0;
        }
    }
    let s = &word[1..];
    let finishes = recursive_find(s, &next_node.childs, &whole_trie, eliminated);
    if next_node.is_end {
        if eliminated.contains_key(s) {
            return finishes + eliminated.get(s).unwrap();
        }
        let whole_computation = recursive_find(s, &whole_trie.childs, whole_trie, eliminated);
        eliminated.insert(s.to_string(), whole_computation);
        return whole_computation + finishes;
    }
    return finishes;
}

pub fn add_to_trie(word: &str, current_node: &mut TrieNode) {
    if word.len() == 0 {
        current_node.is_end = true;
        return;
    }
    let first_letter = word.chars().nth(0).unwrap();
    add_to_trie(
        &word[1..],
        &mut current_node.childs.entry(first_letter).or_insert(TrieNode {
            is_end: false,
            childs: HashMap::new(),
        }),
    );
}

pub struct TrieNode {
    is_end: bool,
    childs: HashMap<char, TrieNode>,
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

pub fn pattern(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    separated_pair(patterns, tuple((newline, newline)), targets)(input)
}

pub fn patterns(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alpha1)(input)
}

pub fn targets(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(newline, alpha1)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_build_trie() {
        let words = vec!["ab", "ac"];
        let mut trie = TrieNode {
            is_end: false,
            childs: HashMap::new(),
        };
        words.iter().for_each(|prefix| {
            add_to_trie(prefix, &mut trie);
        });
        assert_eq!(trie.childs.len(), 1);
        let a_child = trie.childs.get(&'a').unwrap();
        assert_eq!(a_child.is_end, false);
        assert_eq!(a_child.childs.len(), 2);
        let c_child = a_child.childs.get(&'c').unwrap();
        assert_eq!(c_child.is_end, true);
        assert_eq!(c_child.childs.len(), 0);
    }

    #[test]
    fn test_build_trie_2() {
        let words = vec!["ab", "ac", "acd", "e", "acd"];
        let mut trie = TrieNode {
            is_end: false,
            childs: HashMap::new(),
        };
        words.iter().for_each(|prefix| {
            add_to_trie(prefix, &mut trie);
        });
        assert_eq!(trie.childs.len(), 2);
        let e_child = trie.childs.get(&'e').unwrap();
        assert_eq!(e_child.is_end, true);
        assert_eq!(e_child.childs.len(), 0);
        let a_child = trie.childs.get(&'a').unwrap();
        assert_eq!(a_child.is_end, false);
        assert_eq!(a_child.childs.len(), 2);
        let c_child = a_child.childs.get(&'c').unwrap();
        assert_eq!(c_child.is_end, true);
        assert_eq!(c_child.childs.len(), 1);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
