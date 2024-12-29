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


fn day2(lines: &Vec<String>, a: bool) -> Result<String> {
    fn is_generally_ascending(v: &Vec<i32>) -> bool {
        let integral = v.windows(2)
            .map(|w| (w[1] > w[0]) as i32 * 2 - 1)
            .into_iter()
            .reduce(|a, b| a + b);
        return integral.unwrap() > 0;
    }

    fn is_safe_pair(a: &i32, b: &i32, ascending: bool) -> bool {
        let delta = if ascending {*b - *a} else {*a - *b};
        (1..=3).contains(&delta)
    }

    fn first_intolerable(v: &Vec<i32>, ascending: bool) -> Option<usize> {
        for i in 0..v.len() - 1 {
            if !is_safe_pair(&v[i], &v[i+1], ascending) {
                let first_pair_safe = i == 0 || is_safe_pair(&v[i-1], &v[i+1], ascending);
                let second_pair_safe = i < v.len() - 2 && is_safe_pair(&v[i+1], &v[i+2], ascending);
                return if first_pair_safe && second_pair_safe { Some(i) } else { Some(i+1) };
            }
        }
        None
    }

    let mut num_safe = 0;

    for i in 0..lines.len() {
        let mut parsed: Vec<i32> = lines[i]
            .split_ascii_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // Determine if we're generally ascending or descending
        let ascending = is_generally_ascending(&parsed);

        // Remove the first intolerable level (if one exists)
        if !a {
            if let Some(index) = first_intolerable(&parsed, ascending) {
                parsed.remove(index);
            }
        }

        // Check if line is safe
        if parsed.windows(2).all(|w| is_safe_pair(&w[0], &w[1], ascending))
        {
            num_safe += 1;
        }
    }

    Ok(num_safe.to_string())
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

    #[test]
    fn test_day2() {
        let input_str = r"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9";
        let input = input_str_to_vec_of_strings(&input_str);
        assert_eq!(day2(&input, true).unwrap(), "2");
        assert_eq!(day2(&input, false).unwrap(), "4");
    }
}

fn main() -> Result<()> {
    // Parse CLI args to determine what puzzle
    let cli = Cli::parse();

    // Get thr function or raise before any reading or parsing
    let aoc_fn: AocFn = match cli.day {
        1 => day1,
        2 => day2,
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
