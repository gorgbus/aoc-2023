use std::collections::HashMap;

use regex::Regex;

fn main() {
    let input = include_str!("../input.txt");

    let pattern = Regex::new(r"(\d+)").unwrap();

    let mut gears: HashMap<usize, Vec<u32>> = HashMap::new();

    let engine_part_sum: u32 = input
        .lines()
        .enumerate()
        .map(|(ln_i, ln)| {
            let mut nums = vec![];

            pattern.find_iter(ln).for_each(|cap| {
                let mut num = 0;
                let number: u32 = cap.as_str().parse().unwrap();

                for index in cap.start()..cap.end() {
                    if is_engine_part(input, ln_i, ln.len(), index, &mut gears, number) {
                        num = number;
                    }
                }

                nums.push(num);
            });

            nums.iter().sum::<u32>()
        })
        .sum();

    println!("1: {engine_part_sum}");

    // 2

    let gear_ratios_sum: u32 = gears
        .iter_mut()
        .map(|(_, nums)| {
            nums.dedup();

            if nums.len() == 2 {
                nums[0] * nums[1]
            } else {
                0
            }
        })
        .sum();

    println!("2: {gear_ratios_sum}");
}

fn is_engine_part(
    input: &str,
    line: usize,
    line_len: usize,
    index: usize,
    map: &mut HashMap<usize, Vec<u32>>,
    number: u32,
) -> bool {
    let mut is = false;

    let line_count = input.lines().count() - 1;
    let input = &input.replace("\r\n", "");

    let actual_index = index + line_len * line;

    if index > 0 {
        if line > 0 {
            is = is_part(input, actual_index - line_len - 1, is, map, number);
            is = is_part(input, actual_index - line_len, is, map, number);
            is = is_part(input, actual_index - line_len + 1, is, map, number);
        }

        is = is_part(input, actual_index - 1, is, map, number);
    }

    if index < line_len - 1 {
        is = is_part(input, actual_index + 1, is, map, number);

        if line < line_count {
            is = is_part(input, actual_index + line_len - 1, is, map, number);
            is = is_part(input, actual_index + line_len, is, map, number);
            is = is_part(input, actual_index + line_len + 1, is, map, number);
        }
    }

    is
}

fn is_part(
    input: &str,
    index: usize,
    mut is: bool,
    map: &mut HashMap<usize, Vec<u32>>,
    number: u32,
) -> bool {
    let character = &input[index..=index];

    if character == "*" {
        match map.get_mut(&index) {
            Some(nums) => {
                nums.push(number);
            }
            None => {
                map.insert(index, vec![number]);
            }
        };
    }

    if character != "." && character.parse::<u32>().is_err() {
        is = true;
    }

    is
}
