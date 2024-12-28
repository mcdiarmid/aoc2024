use std::io::{self};
use std::error;

use clap::Parser;

type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
type AocFn = fn(&Vec<String>, bool) -> Result<String>;

/// Read some lines of a file
#[derive(Debug, Parser)]
struct Cli {
    /// Advent of Code 2024 Day
    #[structopt(short = 'd')]
    day: i32,

    /// Part 1 or Part 2
    #[structopt(short = 'p')]
    part: i32,

    /// Input file to read
    input: Option<String>,
}

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}


fn day1(lines: &Vec<String>, a: bool) -> Result<String> {
    // Populate left and right vectors with values in chronological order
    let mut left = vec![0u32; lines.len()];
    let mut right = vec![0u32; lines.len()];

    for i in 0..lines.len() {
        let parsed: Vec<u32> = lines[i]
            .split_ascii_whitespace()
            .map(|s| s.parse::<u32>().unwrap())
            .collect();

        if parsed.len() != 2 {
            Err("Expected exactly two inputs on line")?
        }
        left[i] = parsed[0];
        right[i] = parsed[1];
    }

    // Sort by ascending order, calculate result
    left.sort();
    right.sort();

    let result = match a {
        true =>
            left
                .iter()
                .zip(right)
                .fold(0, |acc, (l, r)| acc + l.abs_diff(r)),
        false =>
            left
                .iter()
                .fold(0, |acc, v| acc + *v * right.iter().filter(|x| **x == *v).count() as u32)
    };
    Ok(result.to_string())
}

#[cfg(test)]
mod test {
    use super::*;

    fn input_str_to_vec_of_strings(input_str: &str) -> Vec<String> {
        input_str
            .lines()
            .into_iter()
            .filter(|&x| !x.is_empty())
            .map(|s| s.to_string())
            .collect()
    }

    #[test]
    fn d1p1() {
        let input_str = r"
        3   4
        4   3
        2   5
        1   3
        3   9
        3   3";
        let input = input_str_to_vec_of_strings(&input_str);
        assert_eq!(day1(&input, true).unwrap(), "11");
        assert_eq!(day1(&input, false).unwrap(), "31");
    }
}

fn main() -> Result<()> {
    // Parse CLI args to determine what puzzle
    let cli = Cli::parse();

    // Get thr function or raise before any reading or parsing
    let aoc_fn: AocFn = match cli.day {
        1 => day1,
        _ => Err("Not Implemented.")?
    };

    // Read all lines of txt file input
    let mut lines: Vec<String> = Vec::new();
    let mut input = String::new();

    // TODO - read from file instead of stdin if provided with CLI args
    loop {
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(len) => {
                if len == 0 {
                    break;
                }
                lines.push(strip_trailing_newline(&input).to_string());
            },
            Err(error) => {
                eprintln!("STDIN Error: {}", error);
            },
        }
    }

    // Call relevant AOC function, return result
    match aoc_fn(&lines, cli.part == 1) {
        Ok(result) => {
            println!("{}", result);
            Ok(())
        },
        Err(error) => {
            eprintln!("STDIN Error: {}", error);
            Err(error)
        },
    }
}
