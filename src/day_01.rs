use std::{collections::HashMap, usize};

pub fn part_one(input: &str, bound: usize) -> usize {
    let mut input_parsed: Vec<usize> = input
        .split("\n\n")
        .map(|x| {
            return x
                .lines()
                .map(|x| x.parse::<usize>().unwrap_or(0))
                .sum::<usize>();
        })
        .collect();

    // input_parsed.sort_by(|a, b| b.cmp(a));
    input_parsed.reverse();

    input_parsed.into_iter().take(bound).sum()
}

mod test {
    use super::*;
    use std::fs;

    const INPUT: &str = r"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

    #[test]
    fn part_one_sample() {
        assert_eq!(24000, part_one(INPUT, 1));
    }

    #[test]
    fn part_one_result() {
        let input = fs::read_to_string("data/day_one.txt").unwrap();
        assert_eq!(67622, part_one(&input, 1));
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(45000, part_one(INPUT, 3));
    }

    #[test]
    fn part_two_result() {
        let input = fs::read_to_string("data/day_one.txt").unwrap();
        assert_eq!(201491, part_one(&input, 3));
    }
}
