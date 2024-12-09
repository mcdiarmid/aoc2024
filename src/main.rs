use std::io::{self};
use std::error::Error;

fn strip_trailing_newline(input: &str) -> &str {
    input
        .strip_suffix("\r\n")
        .or(input.strip_suffix("\n"))
        .unwrap_or(input)
}


fn day1p1(lines: &Vec<String>, output: &mut String, a: bool) -> Result<(), Box<dyn Error>> {
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

    let summation: u32 = match a {
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

    *output = summation.to_string();
    Ok(())
}


fn main() {
    // TODO - Parse CLI args to determine what puzzle

    /* Read puzzle input:
    *   - Read lines into Vec<String>
    *   - Call relevant function based off of CLI args
    *     (Sinature of all fns are (input: &Vec<String>, output: &mut String) -> Result
    * 
    */
    let mut lines: Vec<String> = Vec::new();
    let mut output = String::new();
    let mut input = String::new();

    loop {
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(len) => {
                if len == 0 {
                    match day1p1(&lines, &mut output, false) {
                        Ok(_) => println!("{}", output),
                        Err(error) => eprintln!("Parsing Error: {}", error),
                    };
                    return;
                }
                lines.push(strip_trailing_newline(&input).to_string());
            },
            Err(error) => {
                eprintln!("STDIN Error: {}", error);
                return;
            },
        }
    }
}
