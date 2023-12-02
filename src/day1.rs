use std::collections::HashMap;
use std::fs;
use std::io::{self, BufRead};

/*
Day 1 Advent of Code: https://adventofcode.com/2023/day/1

## Part 1
Process a calibration document that consists of lines of text; each line originally
contained a specific calibration value that the Elves now need to recover. On each line,
the calibration value can be found by combining the first digit and the last digit
(in that order) to form a single two-digit number.

## Part 2
Process calbiration document but be able to handle written out words in the content,
for exampole 'one', 'two', 'three' ... 'nine' as well as values that are numeric.
A moving window algorithm can be used to compare number strings to mappings in a
hash map and translate them to their actual numeric values.

*/
pub fn day1_trebuchet(file_path: &str) -> Result<u32, std::io::Error> {
    let num_map = create_num_map();
    let file = fs::File::open(file_path)?;
    let reader = io::BufReader::new(file);
    let mut sum: u32 = 0;
    let max_word_len = 5;

    for line in reader.lines() {
        let line = line?;
        let mut first: Option<u32> = None;
        let mut last: Option<u32> = None;
        let mut win_start = 0;

        let chars: Vec<char> = line.chars().collect();
        let len = chars.len();

        while win_start < len {
            for win_size in 1..=max_word_len {
                let win_end = win_start + win_size;
                if win_end > len {
                    break;
                }

                let sub_str: String = chars[win_start..win_end].iter().collect();

                if let Some(&num) = num_map.get(&sub_str[..]) {
                    first.get_or_insert(num);
                    last = Some(num);
                } else if let Ok(num) = sub_str.parse::<u32>() {
                    first.get_or_insert(num);
                    last = Some(num);
                }
            }
            win_start += 1;
        }
        let num = 10 * first.unwrap_or(0) + last.unwrap_or(0);
        sum += num;
    }
    Ok(sum)
}

fn create_num_map() -> HashMap<&'static str, u32> {
    let num_map: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    num_map
}
