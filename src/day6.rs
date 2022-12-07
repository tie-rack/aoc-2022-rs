use aoc_runner_derive::aoc;
use std::collections::VecDeque;

/// Find the position in the datastream after a marker of length
/// `marker_size` of distinct characters.
///
/// How does it work? We create a buffer to hold a window of
/// characters as we go through stream. For each character we pull of
/// the stream, we check to see if it's in the current buffer. If it
/// is, we remove characters off the front of the buffer until we pull
/// of its duplicate. If it's not in the buffer and the buffer is
/// full, then we're at the end of the marker, and the next character
/// is the position we're looking for. If we need to keep going, we
/// push the character onto the buffer and move ahead.
fn find_start(stream: &str, marker_length: usize) -> usize {
    let mut buf = VecDeque::with_capacity(marker_length);
    for (i, c) in stream.chars().enumerate() {
        let mut dup_i = 0;
        let mut drop_dupe = false;
        for possible_dup in &buf {
            dup_i += 1;
            if possible_dup == &c {
                drop_dupe = true;
                break;
            }
        }
        if drop_dupe {
            for _ in 0..dup_i {
                buf.pop_front();
            }
        } else if dup_i == marker_length - 1 {
            return i + 1;
        }
        buf.push_back(c);
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
    const INPUT6: &str = "abcccdeccfghijklmnopqrstuvwxyz";

    #[test]
    fn test_find_start_of_packet() {
        assert_eq!(7, find_start_of_packet(&INPUT1));
        assert_eq!(5, find_start_of_packet(&INPUT2));
        assert_eq!(6, find_start_of_packet(&INPUT3));
        assert_eq!(10, find_start_of_packet(&INPUT4));
        assert_eq!(11, find_start_of_packet(&INPUT5));
        assert_eq!(12, find_start_of_packet(&INPUT6));
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
