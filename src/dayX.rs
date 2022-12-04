use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(dayX)]
pub fn input_generator(input: &str) -> Vec<&str> {
    input.lines()
}

#[aoc(dayX, part1)]
pub fn part1(input: &Vec<&str>) -> i32 {
    todo!();
}

#[aoc(dayX, part2)]
pub fn part2(input: &Vec<&str>) -> i32 {
    todo!();
}

#[cfg(test)]
mod tests {
    use indoc::indoc;

    use super::*;

    static EXAMPLE: &'static str = indoc! {"
        A Y
        B X
        C Z
	"};

    #[test]
    fn example_part1() {
        assert_eq!(part1(&input_generator(EXAMPLE)), 15);
    }

    #[test]
    fn example_part2() {
        assert_eq!(part2(&input_generator(EXAMPLE)), 12);
    }
}
