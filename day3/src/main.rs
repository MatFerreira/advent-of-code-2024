use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");
    dbg!(compute_sum_of_products(input));

    dbg!(compute_all_instructions(input));
}

// Part One
fn compute_sum_of_products(input: &str) -> i32 {
    let instruction_pat = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let valid_instructions = instruction_pat
        .captures_iter(input)
        .map(|c| (c[1].parse::<i32>().unwrap(), c[2].parse::<i32>().unwrap()))
        .collect::<Vec<_>>();

    valid_instructions.iter().map(|(x, y)| x * y).sum()
}

// Part Two
fn compute_all_instructions(input: &str) -> i32 {
    let instruction_pat = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();
    let mut sum = 0;
    let mut mul_enabled = true;

    for cap in instruction_pat.captures_iter(input) {
        if cap[0].starts_with("don't") {
            mul_enabled = false;
        } else if cap[0].starts_with("do") {
            mul_enabled = true;
        } else if cap[0].starts_with("mul") && mul_enabled {
            let left_number: i32 = cap[1].parse().unwrap();
            let right_number: i32 = cap[2].parse().unwrap();
            sum += left_number * right_number;
        }
    }

    sum
}
