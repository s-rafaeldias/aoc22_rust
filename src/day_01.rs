use std::{collections::HashMap, usize};

pub fn part_one(input: &str, bound: usize) -> usize {
    let mut elves: HashMap<usize, usize> = HashMap::new();
    let mut current_elve: usize = 0;

    let input_parsed = input.split("\n").map(|x| x.parse::<usize>().unwrap_or(0));

    for item in input_parsed {
        elves
            .entry(current_elve)
            .and_modify(|v| *v += item)
            .or_insert(item);

        if item == 0 {
            current_elve += 1;
        }
    }

    let mut calories: Vec<usize> = elves.into_values().collect();

    calories.sort();
    calories.reverse();

    let x = calories.into_iter().take(bound).sum();
    return x
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
