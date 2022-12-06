use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day6)]
pub fn input_generator(input: &str) -> Vec<char> {
    input.chars().collect()
}

#[aoc(day6, part1)]
pub fn part1(input: &[char]) -> i32 {
    use std::collections::HashSet;
    let mut i = 4;
    let mut found = false;
    while !found {
        let slice = &input[(i-4)..i];
        let mut hs: HashSet<char> = HashSet::new();
        for &s in slice.iter(){
            hs.insert(s);
        }
        if hs.len() == 4 {
            found = true;
        } else {
            i += 1;
        }
    }
    i as i32
}

#[aoc(day6, part2)]
pub fn part2(input: &[char]) -> i32 {
    use std::collections::HashSet;
    let mut i = 14;
    let mut found = false;
    while !found {
        let slice = &input[(i-14)..i];
        let mut hs: HashSet<char> = HashSet::new();
        for &s in slice.iter(){
            hs.insert(s);
        }
        if hs.len() == 14 {
            found = true;
        } else {
            i += 1;
        }
    }
    i as i32
}

#[cfg(test)]
mod tests {

    use super::*;

    static EX1: &'static str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    static EX2: &'static str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    static EX3: &'static str = "nppdvjthqldpwncqszvftbrmjlhg";
    static EX4: &'static str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    static EX5: &'static str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn ex1_part1() {
        assert_eq!(part1(&input_generator(EX1)), 7);
    }

    #[test]
    fn ex2_part1() {
        assert_eq!(part1(&input_generator(EX2)), 5);
    }

    #[test]
    fn ex3_part1() {
        assert_eq!(part1(&input_generator(EX3)), 6);
    }

    #[test]
    fn ex4_part1() {
        assert_eq!(part1(&input_generator(EX4)), 10);
    }

    #[test]
    fn ex5_part1() {
        assert_eq!(part1(&input_generator(EX5)), 11);
    }

    #[test]
    fn ex1_part2() {
        assert_eq!(part2(&input_generator(EX1)), 19);
    }

    #[test]
    fn ex2_part2() {
        assert_eq!(part2(&input_generator(EX2)), 23);
    }

    #[test]
    fn ex3_part2() {
        assert_eq!(part2(&input_generator(EX3)), 23);
    }

    #[test]
    fn ex4_part2() {
        assert_eq!(part2(&input_generator(EX4)), 29);
    }

    #[test]
    fn ex5_part2() {
        assert_eq!(part2(&input_generator(EX5)), 26);
    }
}
