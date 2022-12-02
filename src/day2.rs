use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, PartialEq, Clone, Copy)]
enum RPSPlay {
    Rock,
    Paper,
    Scissors,
}

impl RPSPlay {
    fn score(&self) -> i32 {
        match &self {
            &RPSPlay::Rock => 1,
            &RPSPlay::Paper => 2,
            &RPSPlay::Scissors => 3,
        }
    }
}

impl From<&str> for RPSPlay {
    fn from(s: &str) -> Self {
        match s {
            "A" => RPSPlay::Rock,
            "B" => RPSPlay::Paper,
            "C" => RPSPlay::Scissors,
            "X" => RPSPlay::Rock,
            "Y" => RPSPlay::Paper,
            "Z" => RPSPlay::Scissors,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq)]
enum RPSResult {
    Win,
    Loss,
    Tie,
}

impl RPSResult {
    fn score(&self) -> i32 {
        match &self {
            RPSResult::Win => 6,
            RPSResult::Tie => 3,
            RPSResult::Loss => 0,
        }
    }
}

impl From<&str> for RPSResult {
    fn from(s: &str) -> Self {
        match s {
            "X" => RPSResult::Loss,
            "Y" => RPSResult::Tie,
            "Z" => RPSResult::Win,
            _ => panic!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct RPSRound {
    opponent: RPSPlay,
    me: RPSPlay,
}

impl RPSRound {
    fn round_result(&self) -> RPSResult {
        if &self.opponent == &self.me {
            RPSResult::Tie
        } else {
            match (&self.opponent, &self.me) {
                (RPSPlay::Rock, RPSPlay::Scissors) => RPSResult::Loss,
                (RPSPlay::Paper, RPSPlay::Rock) => RPSResult::Loss,
                (RPSPlay::Scissors, RPSPlay::Paper) => RPSResult::Loss,
                _ => RPSResult::Win,
            }
        }
    }

    fn score(&self) -> i32 {
        &self.me.score() + &self.round_result().score()
    }
}

impl From<&RPSRoundStrategy> for RPSRound {
    fn from(strat: &RPSRoundStrategy) -> Self {
        RPSRound {
            opponent: strat.opponent,
            me: strat.my_play(),
        }
    }
}

pub struct RPSRoundStrategy {
    opponent: RPSPlay,
    res: RPSResult,
}

impl RPSRoundStrategy {
    fn my_play(&self) -> RPSPlay {
        match self.res {
            RPSResult::Tie => self.opponent,
            RPSResult::Win => match self.opponent {
                RPSPlay::Rock => RPSPlay::Paper,
                RPSPlay::Paper => RPSPlay::Scissors,
                RPSPlay::Scissors => RPSPlay::Rock,
            },
            RPSResult::Loss => match self.opponent {
                RPSPlay::Rock => RPSPlay::Scissors,
                RPSPlay::Paper => RPSPlay::Rock,
                RPSPlay::Scissors => RPSPlay::Paper,
            },
        }
    }
}

#[aoc_generator(day2, part1)]
pub fn parse_rps_strategy(input: &str) -> Vec<RPSRound> {
    input
        .lines()
        .map(|l| {
            let mut plays = l.split(' ');
            let opp = plays.next().unwrap();
            let me = plays.next().unwrap();
            RPSRound {
                opponent: RPSPlay::from(opp),
                me: RPSPlay::from(me),
            }
        })
        .collect()
}

#[aoc_generator(day2, part2)]
#[allow(non_snake_case)]
pub fn parse_rps_strategy_FINAL(input: &str) -> Vec<RPSRoundStrategy> {
    input
        .lines()
        .map(|l| {
            let mut plays = l.split(' ');
            let opp = plays.next().unwrap();
            let res = plays.next().unwrap();
            RPSRoundStrategy {
                opponent: RPSPlay::from(opp),
                res: RPSResult::from(res),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn day2part1(rounds: &Vec<RPSRound>) -> i32 {
    rounds.iter().fold(0, |acc, r| acc + r.score())
}

#[aoc(day2, part2)]
fn day2part2(rounds: &Vec<RPSRoundStrategy>) -> i32 {
    rounds
        .iter()
        .map(|strat| RPSRound::from(strat))
        .fold(0, |acc, r| acc + r.score())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rps_scores() {
        assert_eq!(RPSPlay::Rock.score(), 1);
        assert_eq!(RPSPlay::Paper.score(), 2);
        assert_eq!(RPSPlay::Scissors.score(), 3);
    }

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_parser() {
        let rounds = parse_rps_strategy(INPUT);
        assert_eq!(
            rounds,
            vec![
                RPSRound {
                    opponent: RPSPlay::Rock,
                    me: RPSPlay::Paper
                },
                RPSRound {
                    opponent: RPSPlay::Paper,
                    me: RPSPlay::Rock
                },
                RPSRound {
                    opponent: RPSPlay::Scissors,
                    me: RPSPlay::Scissors
                },
            ]
        );
    }

    #[test]
    fn test_results() {
        let rounds = parse_rps_strategy(INPUT);
        let mut results = rounds.iter().map(|r| r.round_result());
        assert_eq!(results.next(), Some(RPSResult::Win));
        assert_eq!(results.next(), Some(RPSResult::Loss));
        assert_eq!(results.next(), Some(RPSResult::Tie));
        assert_eq!(results.next(), None);
    }

    #[test]
    fn test_part1() {
        let rounds = parse_rps_strategy(INPUT);
        assert_eq!(15, day2part1(&rounds));
    }

    #[test]
    fn test_part2() {
        let rounds = parse_rps_strategy_FINAL(INPUT);
        assert_eq!(12, day2part2(&rounds));
    }
}
