
use regex::Regex;
fn min_games_to_playabe(game: &str) ->  u32{

    let red_regex = Regex::new(r"(\d+)\s*red").unwrap();
    let green_regex = Regex::new(r"(\d+)\s*green").unwrap();
    let blue_regex = Regex::new(r"(\d+)\s*blue").unwrap();

    let max_red = red_regex.captures_iter(game).map(|cap| cap[1].parse::<u32>().unwrap()).max().unwrap_or(0);
    let max_green = green_regex.captures_iter(game).map(|cap| cap[1].parse::<u32>().unwrap()).max().unwrap_or(0);
    let max_blue = blue_regex.captures_iter(game).map(|cap| cap[1].parse::<u32>().unwrap()).max().unwrap_or(0);

    max_red * max_green * max_blue 
}



pub fn day2_part2() {
    let input = include_str!("/home/khozex/projects/aoc2023/src/inputs/day_2.txt");
    let input_lines = input.lines();
    print!("{}", input_lines.map(|line| min_games_to_playabe(line)).sum::<u32>());
}