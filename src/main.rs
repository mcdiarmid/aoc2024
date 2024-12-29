use std::io::{self};
use clap::Parser;

use aoc2024::*;

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
