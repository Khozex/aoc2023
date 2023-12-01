use regex::Regex;
use maplit::hashmap;
use std::collections::HashMap;

fn transform_word_numbers(line: &str) -> String {
    let number_regex = Regex::new(r"oneight|twone|threeight|fiveight|sevenine|eighthree|eightwo|one|two|three|four|five|six|seven|eight|nine").unwrap();
    let word_to_number: HashMap<&str, &str> = hashmap!{
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
		"oneight" => "18",
		"twone" => "21",
		"threeight" => "38",
		"fiveight" => "58",
		"sevenine" => "79",
		"eighthree" => "83",
		"eightwo" => "82",
    };
    number_regex.replace_all(line, |caps: &regex::Captures| {
        word_to_number.get(caps.get(0).unwrap().as_str()).unwrap_or(&"")
    }).to_string()
}

fn get_value_by_char(chars: std::str::Chars) -> u32 {
    let digits: Vec<u32> = chars.filter_map(|c| c.to_digit(10)).collect();
    if let (Some(&first), Some(&last)) = (digits.first(), digits.last()) {
        first * 10 + last
    } else {
        0
    }
}

pub fn day1_part2() {
	let input = include_str!("/home/khozex/projects/aoc2023/src/inputs/day_1.txt");
    let input_lines = input.lines();
    let transformed_lines = input_lines.map(transform_word_numbers);
    let sum: u32 = transformed_lines.map(|line| get_value_by_char(line.chars())).sum();
    println!("{}", sum);
}

