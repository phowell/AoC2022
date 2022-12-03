use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Elf {
    rations: Vec<i32>,
}

impl Elf {
    fn total_cals(&self) -> i32 {
        self.rations.iter().sum()
    }
}

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Elf> {
    input
        .split("\n\n")
        .map(|e| Elf {
            rations: e.lines().map(|v| v.parse::<i32>().unwrap()).collect(),
        })
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<Elf>) -> i32 {
    input.iter().map(|v| v.total_cals()).max().unwrap()
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<Elf>) -> i32 {
    use itertools::Itertools;
    input
        .iter()
        .map(|v| v.total_cals())
        .sorted()
        .rev()
        .take(3)
        .collect::<Vec<i32>>()
        .iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &'static str = indoc! {"
        1000
        2000
        3000
        
        4000
        
        5000
        6000
        
        7000
        8000
        9000
        
        10000
	"};

    #[test]
    fn example_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 24000);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 45000);
    }
}
