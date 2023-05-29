use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, space1},
    sequence::separated_pair,
    IResult,
};
use std::ops::RangeInclusive;

static RAW_INPUT: &str = include_str!("input.txt");

struct PasswordPolicy<'a> {
    range: RangeInclusive<u64>,
    character: &'a str,
    password: &'a str
}

impl<'a> PasswordPolicy<'a> {
    fn is_valid_p1(&self) -> bool {
        let count = self.password.matches(self.character).count() as u64;
        self.range.contains(&count)
    }

    fn is_valid_p2(&self) -> bool {
        let (start, end) = ((*self.range.start() as usize) - 1, (*self.range.end() as usize) - 1);
        let bytes = self.password.as_bytes();
        let character = self.character.as_bytes();
        let (first, second) = (bytes[start] == character[0],bytes[end] == character[0]);
        match (first,second) {
            (true, true) => false,
            (false, false) => false,
            _ => true
        }
    }
}

fn main() {
    //part1
    let part1: Vec<bool> = RAW_INPUT
        .lines()
        .map(|line| parse_policy(line).expect("failed to parse as PasswordPolicy").1.is_valid_p1()).collect();
    println!("p1 => {}",part1.iter().filter(|&&i| i).count());

    //part2
    let part2: Vec<bool> = RAW_INPUT
        .lines()
        .map(|line| parse_policy(line).expect("failed to parse as PasswordPolicy").1.is_valid_p2()).collect();
    println!("p2 => {}",part2.iter().filter(|&&i| i).count());
}

fn parse_policy(input: &str) -> IResult<&str, PasswordPolicy> {
    let (input, range) = parse_range(input)?;
    let (input, _) = space1(input)?;
    let (input, character) = parse_character(input)?;
    let (input, _) = space1(input)?;
    let (input, password) = parse_password(input)?;
    Ok((
        input,
        PasswordPolicy {
            range,
            character,
            password,
        },
    ))
}

fn parse_range(input: &str) -> IResult<&str, RangeInclusive<u64>> {
    let (input, (start, end)) = separated_pair(complete::u64, tag("-"), complete::u64)(input)?;
    Ok((input, start..=end))
}

fn parse_character(input: &str) -> IResult<&str, &str> {
    let (input, character) = alpha1(input)?;
    let (input, _) = tag(":")(input)?;
    Ok((input, character))
}

fn parse_password(input: &str) -> IResult<&str, &str> {
    let (input, password) = alpha1(input)?;
    Ok((input, password))
}