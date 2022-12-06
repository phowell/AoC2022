use std::collections::HashMap;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> (Vec<Vec<char>>, Vec<(i32, i32, i32)>) {
    let (stacks, commands) = input.split_once("\n\n").unwrap();
    (parse_stacks(stacks), parse_commands(commands))
}

pub fn parse_stacks(s: &str) -> Vec<Vec<char>> {
    let stack_str: Vec<&str> = s.lines().rev().collect();
    let mut stack_nums: HashMap<usize, usize> = HashMap::new();
    for (e, x) in stack_str[0].chars().enumerate() {
        if x.is_numeric() {
            stack_nums.insert(e, (x.to_digit(10).unwrap_or(0) as usize) - 1);
        }
    }

    let mut stacks = vec![Vec::<char>::new(); stack_nums.len()];

    let mut stack_vals = stack_str.iter();
    let _ = stack_vals.next();
    for &v in stack_vals {
        for (e, x) in v.chars().enumerate() {
            if x.is_alphabetic() {
                stacks[stack_nums[&e]].push(x);
            }
        }
    }
    stacks
}

pub fn parse_commands(s: &str) -> Vec<(i32, i32, i32)> {
    s.lines()
        .map(|x| x.split_whitespace().collect())
        .map(|x: Vec<&str>| {
            (
                x[1].parse::<i32>().unwrap(),
                x[3].parse::<i32>().unwrap()-1,
                x[5].parse::<i32>().unwrap()-1,
            )
        })
        .collect()
}

#[aoc(day5, part1)]
pub fn part1((stacks, commands): &(Vec<Vec<char>>, Vec<(i32, i32, i32)>)) -> String {
    let mut stacks = stacks.clone();
    for com in commands {
        let mut holding = Vec::<char>::new();
        for _ in 0..com.0 {
            holding.push(stacks[com.1 as usize].pop().unwrap());
        }
        holding.reverse();
        for _ in 0..com.0 {
            stacks[com.2 as usize].push(holding.pop().unwrap());
        }
    }
    let mut answer = String::new();
    for mut s in stacks {
        answer.push(s.pop().unwrap());
    }
    answer
}

#[aoc(day5, part2)]
pub fn part2((stacks, commands): &(Vec<Vec<char>>, Vec<(i32, i32, i32)>)) -> String {
    let mut stacks = stacks.clone();
    for com in commands {
        let mut holding = Vec::<char>::new();
        for _ in 0..com.0 {
            holding.push(stacks[com.1 as usize].pop().unwrap());
        }
        for _ in 0..com.0 {
            stacks[com.2 as usize].push(holding.pop().unwrap());
        }
    }
    let mut answer = String::new();
    for mut s in stacks {
        answer.push(s.pop().unwrap());
    }
    answer
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &'static str = indoc! {"
            [D]    
        [N] [C]    
        [Z] [M] [P]
         1   2   3 
    
        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2
	"};

    #[test]
    fn example_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), "CMZ");
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), "MCD");
    }
}
