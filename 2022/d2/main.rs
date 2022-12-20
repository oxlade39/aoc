use std::{str::FromStr};


fn main() {
    let input = include_str!("input.txt");
    let total: i32 = total_score(input);
    println!("total score: {total}");
    let total_2: i32 = total_score_2(input);
    println!("total score 2: {total_2}");
}

#[derive(Debug, PartialEq)]
enum HandShape {
    Rock,
    Paper,
    Sissors
}

#[derive(Debug, PartialEq)]
struct Round(HandShape, HandShape);

struct RoundStrategy(HandShape, PlayResult);

#[derive(Debug, PartialEq)]
enum RoundParseError {
    BadLength,
    BadOpponent(String),
    BadOurs(String),
}

#[derive(Debug, PartialEq)]
enum RoundStategyParseError {
    BadLength,
    BadOpponent(String),
    BadResult(String),
}

impl FromStr for Round {
    type Err = RoundParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() != 2 {
            return Err(RoundParseError::BadLength);
        }

        let oppononent = match parts[0] {
            "A" => Ok(HandShape::Rock),
            "B" => Ok(HandShape::Paper),
            "C" => Ok(HandShape::Sissors),
            other => Err(RoundParseError::BadOpponent(other.into()))
        }?;

        let ours = match parts[1] {
            "X" => Ok(HandShape::Rock),
            "Y" => Ok(HandShape::Paper),            
            "Z" => Ok(HandShape::Sissors),
            other => Err(RoundParseError::BadOurs(other.into()))
        }?;

        Ok(Round(oppononent, ours))
    }
}

impl FromStr for RoundStrategy {
    type Err = RoundStategyParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" ").collect();
        if parts.len() != 2 {
            return Err(RoundStategyParseError::BadLength);
        }

        let oppononent = match parts[0] {
            "A" => Ok(HandShape::Rock),
            "B" => Ok(HandShape::Paper),
            "C" => Ok(HandShape::Sissors),
            other => Err(RoundStategyParseError::BadOpponent(other.into()))
        }?;

        let pr = match parts[1] {
            "X" => Ok(PlayResult::Lose),
            "Y" => Ok(PlayResult::Draw),            
            "Z" => Ok(PlayResult::Win),
            other => Err(RoundStategyParseError::BadResult(other.into()))
        }?;

        Ok(RoundStrategy(oppononent, pr))
    }
}

#[derive(Debug, PartialEq)]
enum PlayResult {
    Draw,
    Win,
    Lose
}

impl RoundStrategy {
    fn to_round(&self) -> Round {
        match self {
            RoundStrategy(HandShape::Paper, PlayResult::Draw) => Round(HandShape::Paper, HandShape::Paper),
            RoundStrategy(HandShape::Paper, PlayResult::Lose) => Round(HandShape::Paper, HandShape::Rock),
            RoundStrategy(HandShape::Paper, PlayResult::Win) => Round(HandShape::Paper, HandShape::Sissors),

            RoundStrategy(HandShape::Rock, PlayResult::Draw) => Round(HandShape::Rock, HandShape::Rock),
            RoundStrategy(HandShape::Rock, PlayResult::Lose) => Round(HandShape::Rock, HandShape::Sissors),
            RoundStrategy(HandShape::Rock, PlayResult::Win) => Round(HandShape::Rock, HandShape::Paper),

            RoundStrategy(HandShape::Sissors, PlayResult::Draw) => Round(HandShape::Sissors, HandShape::Sissors),
            RoundStrategy(HandShape::Sissors, PlayResult::Lose) => Round(HandShape::Sissors, HandShape::Paper),
            RoundStrategy(HandShape::Sissors, PlayResult::Win) => Round(HandShape::Sissors, HandShape::Rock)
        }
    }
}

impl Round {
    fn play(&self) -> PlayResult {
        match self {
            Round(HandShape::Paper, HandShape::Rock) => PlayResult::Lose,
            Round(HandShape::Rock, HandShape::Paper) => PlayResult::Win,

            Round(HandShape::Sissors, HandShape::Paper) => PlayResult::Lose,
            Round(HandShape::Paper, HandShape::Sissors) => PlayResult::Win,

            Round(HandShape::Rock, HandShape::Sissors) => PlayResult::Lose,
            Round(HandShape::Sissors, HandShape::Rock) => PlayResult::Win,

            _ => PlayResult::Draw
        }
    }

    fn score(&self) -> i32 {
        let result_score = match self.play() {
            PlayResult::Lose => 0,
            PlayResult::Draw => 3,
            PlayResult::Win => 6
        };
        let choice_score = match self.1 {
            HandShape::Rock => 1, 
            HandShape::Paper => 2, 
            HandShape::Sissors => 3 
        };

        result_score + choice_score
    }
}

fn total_score(input: &str) -> i32 {
    input
    .lines()
    .map(|l| l.parse::<Round>().unwrap())
    .map(|round| round.score())
    .sum()
}

fn total_score_2(input: &str) -> i32 {
    input
    .lines()
    .map(|l| l.parse::<RoundStrategy>().unwrap())
    .map(|rs| rs.to_round())
    .map(|r| r.score())
    .sum()
}

#[test]
fn test_parse_round() {
    let line = "A Y";
    let round = line.parse().unwrap();

    assert_eq!(Round(HandShape::Rock, HandShape::Paper), round);
}

#[test]
fn test_play_rock() {
    assert_eq!(PlayResult::Win, Round(HandShape::Sissors, HandShape::Rock).play());
    assert_eq!(PlayResult::Lose, Round(HandShape::Paper, HandShape::Rock).play());
    assert_eq!(PlayResult::Draw, Round(HandShape::Rock, HandShape::Rock).play());
}

#[test]
fn test_play_paper() {
    assert_eq!(PlayResult::Win, Round(HandShape::Rock, HandShape::Paper).play());
    assert_eq!(PlayResult::Lose, Round(HandShape::Sissors, HandShape::Paper).play());
    assert_eq!(PlayResult::Draw, Round(HandShape::Paper, HandShape::Paper).play());
}

#[test]
fn test_play_scissors() {
    assert_eq!(PlayResult::Win, Round(HandShape::Paper, HandShape::Sissors).play());
    assert_eq!(PlayResult::Lose, Round(HandShape::Rock, HandShape::Sissors).play());
    assert_eq!(PlayResult::Draw, Round(HandShape::Sissors, HandShape::Sissors).play());
}

#[test]
fn test_total_score_example() {
    assert_eq!(15, total_score(include_str!("input.example.txt")))
}

#[test]
fn test_total_score2_example() {
    assert_eq!(12, total_score_2(include_str!("input.example.txt")))
}