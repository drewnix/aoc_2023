use std::fs;
use std::io::{self, BufRead};

/*
Day 1 Advent of Code: https://adventofcode.com/2023/day/1

## Part 1
Process a calibration document that consists of lines of text; each line originally 
contained a specific calibration value that the Elves now need to recover. On each line, 
the calibration value can be found by combining the first digit and the last digit 
(in that order) to form a single two-digit number.

*/
pub fn day1_trebuchet(file_path: &str) -> Result<u32, std::io::Error> {
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut sum: u32 = 0;

    for line in reader.lines() {
        let line = line?;
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;

        for c in line.chars() {
            if let Some(digit) = c.to_digit(10) {
                first.get_or_insert(digit);
                last = Some(digit);
            }
        }
        let num = 10 * first.unwrap_or(0) + last.unwrap_or(0);
        sum += num;
    }
    Ok(sum)
}