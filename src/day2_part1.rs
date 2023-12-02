use regex::Regex;

fn game_is_valid_by_rule(game: &str) -> u32 {
    let game_id_regex = Regex::new(r"Game\s+(\d+)").unwrap();
    let game_id = game_id_regex.captures(game).unwrap()[1].parse::<u32>().unwrap();

    let red_regex = Regex::new(r"(\d+)\s*red").unwrap();
    let green_regex = Regex::new(r"(\d+)\s*green").unwrap();
    let blue_regex = Regex::new(r"(\d+)\s*blue").unwrap();

    let red_invalid = red_regex.captures_iter(game).any(|cap| cap[1].parse::<u32>().unwrap() > 12);
    let green_invalid = green_regex.captures_iter(game).any(|cap| cap[1].parse::<u32>().unwrap() > 13);
    let blue_invalid = blue_regex.captures_iter(game).any(|cap| cap[1].parse::<u32>().unwrap() > 14);

    if !red_invalid && !green_invalid && !blue_invalid {
        game_id
    } else {
        0
    }
}

pub fn day2_part1() {
    let input = include_str!("/home/khozex/projects/aoc2023/src/inputs/day_2.txt");
    let input_lines = input.lines();
    println!("{}", input_lines.map(|line| game_is_valid_by_rule(line)).sum::<u32>());
        
}