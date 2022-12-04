use aoc_runner_derive::{aoc, aoc_generator};
use std::ops::RangeInclusive;

#[derive(Debug, PartialEq)]
struct Assignment {
    rooms: RangeInclusive<u32>,
}

impl Assignment {
    fn new(start: u32, end: u32) -> Self {
        Self {
            rooms: (start..=end),
        }
    }

    fn fully_contains(&self, other: &Assignment) -> bool {
        self.rooms.contains(other.rooms.start()) && self.rooms.contains(other.rooms.end())
    }

    fn overlaps(&self, other: &Assignment) -> bool {
        self.rooms.contains(other.rooms.start()) || other.rooms.contains(self.rooms.start())
    }
}

impl From<&str> for Assignment {
    fn from(s: &str) -> Self {
        let (start, end) = s.split_once('-').unwrap();
        Self::new(start.parse().unwrap(), end.parse().unwrap())
    }
}

fn parse_pair(line: &str) -> (Assignment, Assignment) {
    let assignments = line.split_once(',').unwrap();
    (
        Assignment::from(assignments.0),
        Assignment::from(assignments.1),
    )
}

#[aoc_generator(day4)]
fn parse_assignments(input: &str) -> Vec<(Assignment, Assignment)> {
    input.lines().map(parse_pair).collect()
}

#[aoc(day4, part1)]
fn count_full_containments(assignment_pairs: &[(Assignment, Assignment)]) -> usize {
    assignment_pairs
        .iter()
        .filter(|p| p.0.fully_contains(&p.1) || p.1.fully_contains(&p.0))
        .count()
}

#[aoc(day4, part2)]
fn count_overlaps(assignment_pairs: &[(Assignment, Assignment)]) -> usize {
    assignment_pairs
        .iter()
        .filter(|p| p.0.overlaps(&p.1))
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_assignment_from_str() {
        assert_eq!(Assignment::new(2, 4), Assignment::from("2-4"));
    }

    #[test]
    fn test_pair_from_str() {
        assert_eq!(
            (Assignment::new(2, 8), Assignment::new(3, 7)),
            parse_pair("2-8,3-7")
        );
    }

    #[test]
    fn test_generator() {
        assert_eq!(
            vec![
                (Assignment::new(2, 4), Assignment::new(6, 8)),
                (Assignment::new(2, 3), Assignment::new(4, 5)),
                (Assignment::new(5, 7), Assignment::new(7, 9)),
                (Assignment::new(2, 8), Assignment::new(3, 7)),
                (Assignment::new(6, 6), Assignment::new(4, 6)),
                (Assignment::new(2, 6), Assignment::new(4, 8))
            ],
            parse_assignments(&INPUT)
        );
    }

    #[test]
    fn test_count_full_containments() {
        assert_eq!(
            2,
            count_full_containments(&vec![
                (Assignment::new(2, 4), Assignment::new(6, 8)),
                (Assignment::new(2, 3), Assignment::new(4, 5)),
                (Assignment::new(5, 7), Assignment::new(7, 9)),
                (Assignment::new(2, 8), Assignment::new(3, 7)),
                (Assignment::new(6, 6), Assignment::new(4, 6)),
                (Assignment::new(2, 6), Assignment::new(4, 8))
            ])
        );
    }

    #[test]
    fn test_fully_contains() {
        assert!(Assignment::new(2, 8).fully_contains(&Assignment::new(3, 7)));
        assert!(Assignment::new(4, 6).fully_contains(&Assignment::new(6, 6)));

        assert!(!Assignment::new(4, 6).fully_contains(&Assignment::new(6, 7)));
        assert!(!Assignment::new(1, 3).fully_contains(&Assignment::new(4, 7)));
    }

    #[test]
    fn test_overlaps() {
        assert!(!Assignment::new(2, 4).overlaps(&Assignment::new(6, 8)));
        assert!(!Assignment::new(2, 3).overlaps(&Assignment::new(4, 5)));
        assert!(Assignment::new(5, 7).overlaps(&Assignment::new(7, 9)));
        assert!(Assignment::new(2, 8).overlaps(&Assignment::new(3, 7)));
        assert!(Assignment::new(6, 6).overlaps(&Assignment::new(4, 6)));
        assert!(Assignment::new(2, 6).overlaps(&Assignment::new(4, 8)));
    }
}
