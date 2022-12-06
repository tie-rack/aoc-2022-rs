use aoc_runner_derive::aoc;
use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;

fn is_all_different<T: Eq + Hash>(window: impl Iterator<Item = T>) -> bool {
    let mut set = HashSet::new();
    for i in window {
        if set.contains(&i) {
            return false
        }
        set.insert(i);
    }
    true
}

fn find_start(stream: &str, marker_length: usize) -> usize {
    for (i, window) in Vec::from(stream).windows(marker_length).enumerate() {
        if is_all_different(window.iter()) {
            return marker_length + i;
        }
    }
    panic!("No marker found!")
}

#[aoc(day6, part1)]
fn find_start_of_packet(stream: &str) -> usize {
    find_start(stream, 4)
}

#[aoc(day6, part2)]
fn find_start_of_message(stream: &str) -> usize {
    find_start(stream, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn test_find_start_of_packet() {
        assert_eq!(7, find_start_of_packet(&INPUT1));
        assert_eq!(5, find_start_of_packet(&INPUT2));
        assert_eq!(6, find_start_of_packet(&INPUT3));
        assert_eq!(10, find_start_of_packet(&INPUT4));
        assert_eq!(11, find_start_of_packet(&INPUT5));
    }

    #[test]
    fn test_find_start_of_message() {
        assert_eq!(19, find_start_of_message(&INPUT1));
        assert_eq!(23, find_start_of_message(&INPUT2));
        assert_eq!(23, find_start_of_message(&INPUT3));
        assert_eq!(29, find_start_of_message(&INPUT4));
        assert_eq!(26, find_start_of_message(&INPUT5));
    }
}
