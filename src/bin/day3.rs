use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Item(u8);

impl Item {
    fn priority(&self) -> u32 {
        self.0 as u32
    }
}

impl From<u8> for Item {
    fn from(b: u8) -> Self {
        if b > b'Z' {
            Item(b - b'a' + 1)
        } else {
            Item(b - b'A' + 27)
        }
    }
}

#[derive(Debug)]
struct Rucksack(HashSet<Item>);

impl Rucksack {
    fn from_compartments(c1: &str, c2: &str) -> Rucksack {
        let c1: HashSet<Item> = c1.bytes().map(|b| Item::from(b)).collect();
        let c2: HashSet<Item> = c2.bytes().map(|b| Item::from(b)).collect();
        Rucksack(c1.intersection(&c2).cloned().collect())
    }

    fn from_line(line: &str) -> Rucksack {
        let items: HashSet<Item> = line.bytes().map(|b| Item::from(b)).collect();
        Rucksack(items)
    }

    fn items(&self) -> &HashSet<Item> {
        &self.0
    }

    fn priorities(&self) -> u32 {
        self.items()
            .iter()
            .fold(0, |acc, item| acc + item.priority())
    }
}

fn parse(input: &str, f: fn(&str) -> Rucksack) -> Vec<Rucksack> {
    input.lines().map(f).collect()
}

fn part1(input: &str) -> u32 {
    let rucksacks = parse(input, |line| {
        let (left, right) = line.split_at(line.len() / 2);
        Rucksack::from_compartments(left, right)
    });
    rucksacks
        .iter()
        .fold(0, |acc, rucksack| acc + rucksack.priorities())
}

fn part2(input: &str) -> u32 {
    let rucksacks = parse(input, |line| Rucksack::from_line(line));
    rucksacks.chunks(3).fold(0, |acc, group| {
        let badge = group
            .into_iter()
            .map(|r| {
                let items: HashSet<Item> = HashSet::from_iter(r.items().clone());
                items
            })
            .reduce(|l, r| &l & &r)
            .map(|s| Rucksack(s))
            .unwrap();

        acc + badge.priorities()
    })
}

const PROD_INPUT: &str = include_str!("../../day3.prod");

fn main() {
    println!("Part 1 {}", part1(PROD_INPUT));
    println!("Part 2 {}", part2(PROD_INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &str = include_str!("../../day3.test");

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 157);
    }

    const TEST_INPUT2: &str = include_str!("../../day3.test2");
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT2), 70);
    }
}
