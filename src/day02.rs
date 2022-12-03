use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(String, String)> {
    let val: Vec<Vec<&str>> = input
        .lines()
        .map(|v| v.split_whitespace().collect())
        .collect();

    val.iter()
        .map(|x| (String::from(x[0]), String::from(x[1])))
        .collect()
}

#[derive(Clone, PartialEq)]
pub enum RPS {
    Rock,
    Paper,
    Scissors,
}

// Returns points gained by player (6/3/0 for Win/Tie/Loss)
pub fn shoot(opponent: &RPS, player: &RPS) -> i32 {
    match (opponent, player) {
        //win
        (RPS::Rock, RPS::Paper) | (RPS::Paper, RPS::Scissors) | (RPS::Scissors, RPS::Rock) => 6,
        //tie
        (RPS::Rock, RPS::Rock) | (RPS::Paper, RPS::Paper) | (RPS::Scissors, RPS::Scissors) => 3,
        //lose
        _ => 0,
    }
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<(String, String)>) -> i32 {
    let mut score = 0;
    for i in input {
        let o = match i.0.as_str() {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => panic!(),
        };
        let p = match i.1.as_str() {
            "X" => RPS::Rock,
            "Y" => RPS::Paper,
            "Z" => RPS::Scissors,
            _ => panic!(),
        };
        score += shoot(&o, &p);
        score += match p {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };
    }
    score
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<(String, String)>) -> i32 {
    let mut score = 0;
    for i in input {
        let o = match i.0.as_str() {
            "A" => RPS::Rock,
            "B" => RPS::Paper,
            "C" => RPS::Scissors,
            _ => panic!(),
        };
        let p = match i.1.as_str() {
            "X" => match o {
                RPS::Rock => RPS::Scissors,
                RPS::Paper => RPS::Rock,
                RPS::Scissors => RPS::Paper,
            },
            "Y" => o.clone(),
            "Z" => match o {
                RPS::Rock => RPS::Paper,
                RPS::Paper => RPS::Scissors,
                RPS::Scissors => RPS::Rock,
            },
            _ => panic!(),
        };
        score += shoot(&o, &p);
        score += match p {
            RPS::Rock => 1,
            RPS::Paper => 2,
            RPS::Scissors => 3,
        };
    }
    score
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
