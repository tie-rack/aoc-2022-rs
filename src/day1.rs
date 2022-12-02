use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
pub fn elf_load_generator(input: &str) -> Vec<i32> {
    input.lines().fold(vec![0], |mut acc, l| {
        if l.is_empty() {
            acc.push(0);
        } else {
            let len = acc.len();
            acc[len - 1] += l.parse::<i32>().unwrap();
        }
        acc
    })
}

#[aoc(day1, part1)]
fn day1part1(loads: &[i32]) -> i32 {
    *loads.iter().max().unwrap()
}

#[aoc(day1, part2)]
fn day1part2(loads: &[i32]) -> i32 {
    let loads = &mut loads.to_owned();
    loads.sort_unstable();
    loads.reverse();
    loads.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
2000

2000
3000

40
1000
2000

10000
1000
100
10
1";

    #[test]
    fn generator_test() {
        assert_eq!(elf_load_generator(INPUT), vec![3000, 5000, 3040, 11111]);
    }

    #[test]
    fn part1_test() {
        assert_eq!(day1part1(&elf_load_generator(INPUT)), 11111);
    }

    #[test]
    fn part2_test() {
        assert_eq!(day1part2(&elf_load_generator(INPUT)), 11111 + 5000 + 3040);
    }
}
