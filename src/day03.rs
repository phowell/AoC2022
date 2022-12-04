use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashSet;

pub fn score(ch: char) -> i32{
    if ch.is_lowercase(){
        (ch as i32) - 96
    } else {
        (ch as i32) - 38
    }
}

#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|v| v.chars().collect()).collect()
}

#[aoc(day3, part1)]
pub fn part1(input: &Vec<Vec<char>>) -> i32 {
    let mut sum = 0;
    for line in input {
        let (comp1, comp2) = line.split_at(line.len() / 2);
        let hs1: HashSet<char> = comp1.into_iter().copied().collect();
        let hs2: HashSet<char> = comp2.into_iter().copied().collect();
        let intersec: Vec<&char> = hs1.intersection(&hs2).collect();
        sum += score(*intersec[0]);
    }
    sum
}

#[aoc(day3, part2)]
pub fn part2(input: &Vec<Vec<char>>) -> i32 {
    use itertools::Itertools;
    use intersection::hash_set;
    let mut sum = 0;
    for (elf1, elf2, elf3) in input.into_iter().tuples(){
        let elf1: HashSet<char> = elf1.into_iter().copied().collect();
        let elf2: HashSet<char> = elf2.into_iter().copied().collect();
        let elf3: HashSet<char> = elf3.into_iter().copied().collect();
        
        let common = hash_set::intersection([elf1, elf2, elf3]);

        sum += score(common.into_iter().collect::<Vec<char>>()[0]);

    }
    sum
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &'static str = indoc! {"
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "};

    #[test]
    fn example_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 157);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 70);
    }
}
