use itertools::Itertools;

fn map_values(v: &str) -> isize {
    match v {
        "A" | "X" => 1, // Rock
        "B" | "Y" => 2, // Paper
        "C" | "Z" => 3, // Scissors
        _ => 0,
    }
}

pub fn part_one(input: &str) -> isize {
    let result = input
        .lines()
        .map(|round| {
            let (a, b) = round.split_whitespace().collect_tuple().unwrap();
            return (a, b);
        })
        .map(|(x, y)| (map_values(x), map_values(y)))
        .map(|(x, y)| {
            let result = (y - x).rem_euclid(3);
            match result {
                0 => y + 3, // draw
                1 => y + 6, // win
                2 => y + 0, // lost
                _ => 0,
            }
        })
        .sum();

    return result;
}

fn map_values2(a: &str, b: &str) -> (String, String) {
    let result: String = match b {
        "Y" => a.into(), // draw
        "X" => match a {
            // lose
            "A" => "C",
            "B" => "A",
            "C" => "B",
            _ => "",
        }
        .into(),
        "Z" => match a {
            // win
            "A" => "B",
            "B" => "C",
            "C" => "A",
            _ => "",
        }
        .into(),
        _ => "".into(),
    };

    (a.into(), result)
}

pub fn part_two(input: &str) -> isize {
    let result = input
        .lines()
        .map(|round| {
            let (a, b) = round.split_whitespace().collect_tuple().unwrap();
            return (a, b);
        })
        .map(|(a, b)| map_values2(a, b))
        .map(|(x, y)| (map_values(&x), map_values(&y)))
        .map(|(x, y)| {
            let result = (y - x).rem_euclid(3);
            match result {
                0 => y + 3, // draw
                1 => y + 6, // win
                2 => y + 0, // lost
                _ => 0,
            }
        })
        .sum();

    return result;
}

mod test {
    #[allow(unused_imports)]
    use super::*;

    #[allow(dead_code)]
    const INPUT: &str = "A Y\nB X\nC Z";

    #[test]
    fn part_one_sample() {
        assert_eq!(15, part_one(INPUT));
    }

    #[test]
    fn part_one_result() {
        let input = include_str!("../data/day_two.txt");
        assert_eq!(12645, part_one(&input));
    }

    #[test]
    fn part_two_sample() {
        assert_eq!(12, part_two(INPUT));
    }

    #[test]
    fn part_two_result() {
        let input = include_str!("../data/day_two.txt");
        assert_eq!(12645, part_two(&input));
    }
}
