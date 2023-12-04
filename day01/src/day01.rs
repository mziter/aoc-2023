use rayon::prelude::*;

pub fn solve_part_one(input: &str) -> u32 {
    input.par_lines().map(calibration_value).sum()
}

pub fn solve_part_two(input: &str) -> u32 {
    input.par_lines().map(calibration_value_detect_str).sum()
}

fn calibration_value(line: &str) -> u32 {
    let mut answer = "".to_string();
    for c in line.chars() {
        if c.is_ascii_digit() {
            answer.push(c);
            break;
        }
    }
    for c in line.chars().rev() {
        if c.is_ascii_digit() {
            answer.push(c);
            break;
        }
    }
    answer.parse::<u32>().unwrap()
}

fn calibration_value_detect_str(line: &str) -> u32 {
    let mut answer = "".to_string();
    let chars: Vec<char> = line.chars().collect();
    for (i, &c) in chars.iter().enumerate() {
        if c.is_ascii_digit() {
            answer.push(c);
            break;
        }
        let digit_at = at_digit(i, &c, line);
        match digit_at {
            Some(digit) => {
                answer.push(digit);
                break;
            }
            None => continue,
        }
    }
    for (i, &c) in chars.iter().enumerate().rev() {
        if c.is_ascii_digit() {
            answer.push(c);
            break;
        }
        let digit_at = at_digit_rev(i, &c, line);
        match digit_at {
            Some(digit) => {
                answer.push(digit);
                break;
            }
            None => continue,
        }
    }
    answer.parse::<u32>().unwrap()
}

fn at_digit(i: usize, c: &char, line: &str) -> Option<char> {
    let possible_digits = starts_with(c);
    if !possible_digits.is_empty() {
        for possible_digit in possible_digits {
            let end_idx = i + possible_digit.len();
            if end_idx < line.len() && possible_digit == &line[i..end_idx] {
                return what_digit(possible_digit);
            }
        }
    }
    None
}

fn at_digit_rev(i: usize, c: &char, line: &str) -> Option<char> {
    let possible_digits = ends_with(c);
    if !possible_digits.is_empty() {
        for possible_digit in possible_digits {
            if possible_digit.len() > (i + 1) {
                continue;
            }
            let start_idx = (i + 1) - possible_digit.len();
            if possible_digit == &line[start_idx..(i + 1)] {
                return what_digit(possible_digit);
            }
        }
    }
    None
}

fn what_digit(name: &str) -> Option<char> {
    match name {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}

fn starts_with(c: &char) -> Vec<&'static str> {
    match c {
        'o' => vec!["one"],
        't' => vec!["two", "three"],
        'f' => vec!["four", "five"],
        's' => vec!["six", "seven"],
        'e' => vec!["eight"],
        'n' => vec!["nine"],
        _ => vec![],
    }
}

fn ends_with(c: &char) -> Vec<&'static str> {
    match c {
        'e' => vec!["one", "three", "five", "nine"],
        'o' => vec!["two"],
        'r' => vec!["four"],
        'x' => vec!["six"],
        'n' => vec!["seven"],
        't' => vec!["eight"],
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn examples_with_str() {
        assert_eq!(calibration_value_detect_str("two1nine"), 29);
        assert_eq!(calibration_value_detect_str("eighttwothree"), 83);
        assert_eq!(calibration_value_detect_str("abcone2threexyz"), 13);
        assert_eq!(calibration_value_detect_str("xtwoone3four"), 24);
        assert_eq!(calibration_value_detect_str("4nineeightseven2"), 42);
        assert_eq!(calibration_value_detect_str("zoneight234"), 14);
        assert_eq!(calibration_value_detect_str("7pqrstsixteen"), 76);
    }
}
