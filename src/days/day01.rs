use crate::{Solution, SolutionPair};
use std::fs::read_to_string;

///////////////////////////////////////////////////////////////////////////////

pub fn solve() -> SolutionPair {
    let input: Vec<String> = read_to_string("input/day01.txt").unwrap()
        .lines()
        .map(str::to_string)
        .collect();

    let mut sum = 0;
    for line in input {
        let digits: Vec<char> = line.chars().filter(|x| x.is_digit(10)).collect();
        let first = digits.first().unwrap();
        let last = digits.last().unwrap();
        let mut str_number = String::from("");
        str_number.push(*first);
        str_number.push(*last);
        let number = str_number.parse::<i32>().unwrap();
        sum += number;
    }

    let sol1: i32 = sum;

    let number_strings = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let sol2: u64 = 0;

    (Solution::from(sol1), Solution::from(sol2))
}
