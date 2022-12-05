use aoc_runner_derive::{aoc, aoc_generator};
use regex::Regex;
use std::collections::VecDeque;

type Stacks = Vec<VecDeque<char>>;
type Moves = Vec<(usize, usize, usize)>;

fn split_input(input: &str) -> (&str, &str) {
    input.split_once("\n\n").unwrap()
}

fn stack_count(stacks_layout: &str) -> usize {
    (stacks_layout.lines().next().unwrap().len() + 1) / 4
}

fn parse_stacks(stacks_layout: &str) -> Stacks {
    let count = stack_count(stacks_layout);
    let mut stacks = Vec::with_capacity(count);

    for _ in 0..count {
        stacks.push(VecDeque::new());
    }

    for line in stacks_layout.lines() {
        let mut chars = line.chars().skip(1).step_by(4);
        for i in 0..count {
            let c = chars.next().unwrap();
            if c.is_ascii_alphabetic() {
                stacks.get_mut(i).unwrap().push_back(c);
            }
        }
    }

    stacks
}

fn parse_moves(moves: &str) -> Moves {
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    re.captures_iter(moves)
        .map(|mov| {
            (
                mov.get(1).unwrap().as_str().parse().unwrap(),
                mov.get(2).unwrap().as_str().parse().unwrap(),
                mov.get(3).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect()
}

fn perform_moves(mut stacks: Stacks, moves: Moves) -> Stacks {
    for (n, from, to) in moves {
        for _ in 0..n {
            let item = stacks.get_mut(from - 1).unwrap().pop_front().unwrap();
            let to_stack = stacks.get_mut(to - 1).unwrap();
            to_stack.push_front(item);
        }
    }
    stacks
}

fn perform_moves_9001(mut stacks: Stacks, moves: Moves) -> Stacks {
    for (n, from, to) in moves {
        let mut items = VecDeque::new();
        for _ in 0..n {
            items.push_back(stacks.get_mut(from - 1).unwrap().pop_front().unwrap());
        }
        let to_stack = stacks.get_mut(to - 1).unwrap();
        for _ in 0..n {
            to_stack.push_front(items.pop_back().unwrap());
        }
    }
    stacks
}

fn stack_tops(stacks: &Stacks) -> String {
    stacks.iter().map(|stack| stack.front().unwrap()).collect()
}

#[aoc_generator(day5)]
fn parse_input(input: &str) -> (Stacks, Moves) {
    let (stack_def, move_def) = split_input(input);
    (parse_stacks(stack_def), parse_moves(move_def))
}

#[aoc(day5, part1)]
fn find_tops_after_moves((stacks, moves): &(Stacks, Moves)) -> String {
    stack_tops(&perform_moves(stacks.clone(), moves.clone()))
}

#[aoc(day5, part2)]
fn find_tops_after_moves_9001((stacks, moves): &(Stacks, Moves)) -> String {
    stack_tops(&perform_moves_9001(stacks.clone(), moves.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Having your editor strip trailing whitespace is usally cool,
    // but not when you have multi-line strings with trailing
    // whitespace that matters.
    const INPUT: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 \n
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn test_split_input() {
        let res = (
            "    [D]    \n[N] [C]    \n[Z] [M] [P]\n 1   2   3 ",
            "move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2",
        );
        assert_eq!(res, split_input(&INPUT));
    }

    #[test]
    fn test_stack_count() {
        assert_eq!(3, stack_count(split_input(&INPUT).0));
    }

    #[test]
    fn test_parse_stacks() {
        assert_eq!(
            vec![
                VecDeque::from(['N', 'Z']),
                VecDeque::from(['D', 'C', 'M']),
                VecDeque::from(['P']),
            ],
            parse_stacks(split_input(&INPUT).0)
        );
    }

    #[test]
    fn test_parse_moves() {
        assert_eq!(
            vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)],
            parse_moves(split_input(&INPUT).1)
        );
    }

    #[test]
    fn test_perform_moves() {
        let (stack_def, move_def) = split_input(&INPUT);
        assert_eq!(
            vec![
                VecDeque::from(['C']),
                VecDeque::from(['M']),
                VecDeque::from(['Z', 'N', 'D', 'P']),
            ],
            perform_moves(parse_stacks(stack_def), parse_moves(move_def))
        );
    }

    #[test]
    fn test_stack_tops() {
        assert_eq!(
            String::from("CMZ"),
            stack_tops(&vec![
                VecDeque::from(['C']),
                VecDeque::from(['M']),
                VecDeque::from(['Z', 'N', 'D', 'P']),
            ])
        );
    }

    #[test]
    fn test_find_tops_after_moves() {
        assert_eq!(
            String::from("CMZ"),
            find_tops_after_moves(&(
                vec![
                    VecDeque::from(['N', 'Z']),
                    VecDeque::from(['D', 'C', 'M']),
                    VecDeque::from(['P']),
                ],
                vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)]
            ))
        );
    }

    #[test]
    fn test_perform_moves_9001() {
        let (stack_def, move_def) = split_input(&INPUT);
        assert_eq!(
            vec![
                VecDeque::from(['M']),
                VecDeque::from(['C']),
                VecDeque::from(['D', 'N', 'Z', 'P']),
            ],
            perform_moves_9001(parse_stacks(stack_def), parse_moves(move_def))
        );
    }

    #[test]
    fn test_find_tops_after_moves_9001() {
        assert_eq!(
            String::from("MCD"),
            find_tops_after_moves_9001(&(
                vec![
                    VecDeque::from(['N', 'Z']),
                    VecDeque::from(['D', 'C', 'M']),
                    VecDeque::from(['P']),
                ],
                vec![(1, 2, 1), (3, 1, 3), (2, 2, 1), (1, 1, 2)]
            ))
        );
    }
}
