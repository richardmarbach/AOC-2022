use std::cmp::Reverse;

const DATA: &str = include_str!("../../day1.prod");

fn parse_calories_per_elf(data: &str) -> Vec<i32> {
    let calories: Vec<Option<i32>> = data
        .lines()
        .map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.parse::<i32>().unwrap())
            }
        })
        .collect();

    calories
        .split(|l| l.is_none())
        .map(|per_elf| per_elf.into_iter().flat_map(|c| c).sum::<i32>())
        .collect()
}

fn part1(data: &str) -> i32 {
    *parse_calories_per_elf(data).iter().max().unwrap()
}

fn part2(data: &str) -> i32 {
    let mut calories_per_elf = parse_calories_per_elf(data);
    calories_per_elf.sort_unstable_by_key(|d| Reverse(*d));
    calories_per_elf.iter().take(3).sum()
}

fn main() {
    println!("Part 1: {}", part1(DATA));
    println!("Part 2: {}", part2(DATA));
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = include_str!("../../day1.test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(SAMPLE), 24000);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(SAMPLE), 45_000);
    }
}
