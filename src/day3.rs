use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::{HashMap, HashSet};

fn rucksack_split(contents: &str) -> (&str, &str) {
    contents.split_at(contents.len() / 2)
}

fn compartment_set(contents: &str) -> HashSet<char> {
    contents.chars().collect::<HashSet<_>>()
}

fn rucksack_shared_item(compartments: (&str, &str)) -> char {
    let left = compartment_set(compartments.0);
    let right = compartment_set(compartments.1);
    *left.intersection(&right).into_iter().next().unwrap()
}

fn priority(item: char) -> u32 {
    if item.is_ascii_lowercase() {
        u32::from(item) - 96
    } else {
        u32::from(item) - 38
    }
}

#[aoc_generator(day3, part1)]
fn compartment_priorities(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|l| priority(rucksack_shared_item(rucksack_split(l))))
        .collect()
}

// One char will appear once in each group
// Not the most efficient implementation.
fn group_badge(groups: &[&str]) -> char {
    let mut counts = HashMap::<char, u8>::new();
    for group in groups {
        for item in group.chars().collect::<HashSet<char>>() {
            let count = counts.entry(item).or_insert(0);
            *count += 1;
        }
    }
    let mut badge: Option<char> = None;
    for (c, n) in counts {
        if n == 3 {
            badge = Some(c);
        }
    }
    badge.unwrap()
}

#[aoc_generator(day3, part2)]
fn group_priorities(input: &str) -> Vec<u32> {
    let lines = input.lines().collect::<Vec<&str>>();
    let groups = lines[..].chunks(3);
    groups.map(|g| priority(group_badge(g))).collect()
}

#[aoc(day3, part1)]
#[aoc(day3, part2)]
fn priorities_total(priorites: &[u32]) -> u32 {
    priorites.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_rucksack_split() {
        assert_eq!(
            ("vJrwpWtwJgWr", "hcsFMMfFFhFp"),
            rucksack_split(&"vJrwpWtwJgWrhcsFMMfFFhFp")
        );
    }

    #[test]
    fn test_rucksack_shared_item() {
        assert_eq!(
            rucksack_shared_item(rucksack_split(&"vJrwpWtwJgWrhcsFMMfFFhFp")),
            'p'
        );
    }

    #[test]
    fn test_priority() {
        assert_eq!(1, priority('a'));
        assert_eq!(2, priority('b'));
        assert_eq!(26, priority('z'));
        assert_eq!(27, priority('A'));
        assert_eq!(28, priority('B'));
        assert_eq!(52, priority('Z'));
    }

    #[test]
    fn test_priorities_total() {
        assert_eq!(157, priorities_total(&compartment_priorities(&INPUT)));
    }

    #[test]
    fn test_group_badge() {
        assert_eq!(
            'r',
            group_badge(&[
                "vJrwpWtwJgWrhcsFMMfFFhFp",
                "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
                "PmmdzqPrVvPwwTWBwg"
            ])
        );
        assert_eq!(
            'Z',
            group_badge(&[
                "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
                "ttgJtRGJQctTZtZT",
                "CrZsJsPPZsGzwwsLwLmpwMDw"
            ])
        );
    }

    #[test]
    fn test_group_priority_total() {
        assert_eq!(70, priorities_total(&group_priorities(&INPUT)));
    }
}
