
fn get_number_by_line(line: &str) -> u32 {
    let char = line.chars();
    let mut all_numbers_in_line = char.filter(|c| c.is_digit(10));
    let first = all_numbers_in_line.next().unwrap_or('0');
    let last = all_numbers_in_line.last().unwrap_or(first);
    first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
}

pub fn day1_part1() {
    let input = include_str!("/home/khozex/projects/aoc2023/src/inputs/day_1.txt");
    let input_lines = input.lines();
    let input_lines = input_lines.map(|line| get_number_by_line(line));
    print!("{}", input_lines.sum::<u32>());
}
