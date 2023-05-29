use itertools::Itertools;
static RAW_INPUT: &str = include_str!("input.txt");

fn main() {
    let parsed_input: Vec<i64> = RAW_INPUT
        .lines()
        .map(|line| line.parse::<i64>().expect("failed to parse as i64"))
        .collect();

    let part1 = parsed_input
        .iter()
        .tuple_combinations::<(&i64, &i64)>()
        .find(|&(a, b)| a + b == 2020)
        .map(|(a, b)| a * b)
        .expect("didn't find a valid solution in the input");
    println!("{:?}", part1);

    let part2 = parsed_input
        .iter()
        .tuple_combinations::<(&i64, &i64, &i64)>()
        .find(|&(a, b, c)| a + b + c == 2020)
        .map(|(a, b, c)| a * b * c)
        .expect("didn't find a valid solution in the input");
    println!("{:?}", part2);
}
