use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<Vec<(i32, i32)>> {
    input
        .lines()
        .filter_map(|v| v.split_once(','))
        .map(|x| vec![vals(x.0), vals(x.1)])
        .collect()
}

pub fn vals(v: &str) -> (i32, i32) {
    let nums = v.split_once("-").unwrap();
    (nums.0.parse::<i32>().unwrap(), nums.1.parse::<i32>().unwrap())
}

// Returns true if either input is a subset of the other
pub fn contained(x: (i32, i32), y: (i32, i32)) -> bool {
    (x.0 <= y.0 && x.1 >= y.1) || (y.0 <= x.0 && y.1 >= x.1)
}

pub fn overlap(x: (i32, i32), y: (i32, i32)) -> bool {
    
    (y.0 >= x.0 && y.0 <= x.1) ||
    (y.1 >= x.0 && y.1 <= x.1) ||
    (x.0 >= y.0 && x.0 <= y.1) ||
    (x.1 >= y.0 && x.1 <= y.1) 
}

#[aoc(day4, part1)]
pub fn part1(input: &Vec<Vec<(i32, i32)>>) -> i32 {
    let mut sum = 0;
    for line in input{
        if contained(line[0], line[1]) {
            sum += 1;
        }
    }
    sum
}

#[aoc(day4, part2)]
pub fn part2(input: &Vec<Vec<(i32, i32)>>) -> i32 {
    let mut sum = 0;
    for line in input{
        if overlap(line[0], line[1]) {
            sum += 1;
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &'static str = indoc! {"
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "};

    #[test]
    fn example_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 2);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 4);
    }
}
