use std::error;

pub type Result<T> = std::result::Result<T, Box<dyn error::Error>>;
pub type AocFn = fn(&Vec<String>, bool) -> Result<String>;


/* Day 1 */
pub fn day1(lines: &Vec<String>, a: bool) -> Result<String> {
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

/* Day 2 */
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

fn first_intolerable(report: &Vec<i32>, ascending: bool) -> Option<usize> {
    (0..report.len() - 1)
        .filter(|&i| { !is_safe_pair(&report[i], &report[i+1], ascending) })
        .map(|i| {
            let first = i == 0 || is_safe_pair(&report[i-1], &report[i+1], ascending);
            let second = i < report.len() - 2 && is_safe_pair(&report[i+1], &report[i+2], ascending);
            if first && second { i } else { i+1 }
        })
        .next()
}

pub fn day2(lines: &Vec<String>, a: bool) -> Result<String> {
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
