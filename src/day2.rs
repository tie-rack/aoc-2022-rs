use aoc_runner_derive::{aoc, aoc_generator};

#[allow(non_snake_case)]
mod RPS {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub enum Play {
        Rock,
        Paper,
        Scissors,
    }

    impl Play {
        pub fn score(&self) -> i32 {
            match self {
                Play::Rock => 1,
                Play::Paper => 2,
                Play::Scissors => 3,
            }
        }
    }

    impl From<&str> for Play {
        fn from(s: &str) -> Self {
            match s {
                "A" => Play::Rock,
                "B" => Play::Paper,
                "C" => Play::Scissors,
                "X" => Play::Rock,
                "Y" => Play::Paper,
                "Z" => Play::Scissors,
                _ => panic!(),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Result {
        Win,
        Loss,
        Tie,
    }

    impl Result {
        pub fn score(&self) -> i32 {
            match &self {
                Result::Win => 6,
                Result::Tie => 3,
                Result::Loss => 0,
            }
        }
    }

    impl From<&str> for Result {
        fn from(s: &str) -> Self {
            match s {
                "X" => Result::Loss,
                "Y" => Result::Tie,
                "Z" => Result::Win,
                _ => panic!(),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct Round {
        pub opponent: Play,
        pub me: Play,
    }

    impl Round {
        pub fn round_result(&self) -> Result {
            if self.opponent == self.me {
                Result::Tie
            } else {
                match (&self.opponent, &self.me) {
                    (Play::Rock, Play::Scissors) => Result::Loss,
                    (Play::Paper, Play::Rock) => Result::Loss,
                    (Play::Scissors, Play::Paper) => Result::Loss,
                    _ => Result::Win,
                }
            }
        }

        pub fn score(&self) -> i32 {
            self.me.score() + self.round_result().score()
        }
    }

    impl From<&RoundStrategy> for Round {
        fn from(strat: &RoundStrategy) -> Self {
            Round {
                opponent: strat.opponent,
                me: strat.my_play(),
            }
        }
    }

    pub struct RoundStrategy {
        pub opponent: Play,
        pub res: Result,
    }

    impl RoundStrategy {
        fn my_play(&self) -> Play {
            match self.res {
                Result::Tie => self.opponent,
                Result::Win => match self.opponent {
                    Play::Rock => Play::Paper,
                    Play::Paper => Play::Scissors,
                    Play::Scissors => Play::Rock,
                },
                Result::Loss => match self.opponent {
                    Play::Rock => Play::Scissors,
                    Play::Paper => Play::Rock,
                    Play::Scissors => Play::Paper,
                },
            }
        }
    }
}

#[aoc_generator(day2, part1)]
pub fn parse_rps_strategy(input: &str) -> Vec<RPS::Round> {
    input
        .lines()
        .map(|l| {
            let mut plays = l.split(' ');
            let opp = plays.next().unwrap();
            let me = plays.next().unwrap();
            RPS::Round {
                opponent: RPS::Play::from(opp),
                me: RPS::Play::from(me),
            }
        })
        .collect()
}

#[aoc_generator(day2, part2)]
#[allow(non_snake_case)]
pub fn parse_rps_strategy_FINAL(input: &str) -> Vec<RPS::RoundStrategy> {
    input
        .lines()
        .map(|l| {
            let mut plays = l.split(' ');
            let opp = plays.next().unwrap();
            let res = plays.next().unwrap();
            RPS::RoundStrategy {
                opponent: RPS::Play::from(opp),
                res: RPS::Result::from(res),
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn day2part1(rounds: &[RPS::Round]) -> i32 {
    rounds.iter().fold(0, |acc, r| acc + r.score())
}

#[aoc(day2, part2)]
fn day2part2(rounds: &[RPS::RoundStrategy]) -> i32 {
    rounds
        .iter()
        .map(RPS::Round::from)
        .fold(0, |acc, r| acc + r.score())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rps_scores() {
        assert_eq!(RPS::Play::Rock.score(), 1);
        assert_eq!(RPS::Play::Paper.score(), 2);
        assert_eq!(RPS::Play::Scissors.score(), 3);
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
                RPS::Round {
                    opponent: RPS::Play::Rock,
                    me: RPS::Play::Paper
                },
                RPS::Round {
                    opponent: RPS::Play::Paper,
                    me: RPS::Play::Rock
                },
                RPS::Round {
                    opponent: RPS::Play::Scissors,
                    me: RPS::Play::Scissors
                },
            ]
        );
    }

    #[test]
    fn test_results() {
        let rounds = parse_rps_strategy(INPUT);
        let mut results = rounds.iter().map(|r| r.round_result());
        assert_eq!(results.next(), Some(RPS::Result::Win));
        assert_eq!(results.next(), Some(RPS::Result::Loss));
        assert_eq!(results.next(), Some(RPS::Result::Tie));
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
