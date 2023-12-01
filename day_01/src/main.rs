use std::{cmp::Ordering, collections::HashMap};

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    // 1

    let input = include_str!("../input.txt");

    let calibration = input
        .lines()
        .map(|ln| {
            let first_i = ln.find(|c: char| c.is_digit(10)).unwrap();
            let last_i = ln.rfind(|c: char| c.is_digit(10)).unwrap();

            let first = ln.chars().nth(first_i).unwrap().to_digit(10).unwrap() * 10;
            let last = ln.chars().nth(last_i).unwrap().to_digit(10).unwrap();

            first + last
        })
        .sum::<u32>();

    println!("part 1: {calibration}");

    // 2

    let input = include_str!("../input2.txt");

    let map: HashMap<&str, u32> = NUMBERS
        .into_iter()
        .enumerate()
        .map(|(i, num)| (num, i as u32 + 1))
        .collect();

    let calibration = input
        .lines()
        .map(|ln| {
            let first_i = ln.find(|c: char| c.is_digit(10));
            let last_i = ln.rfind(|c: char| c.is_digit(10));

            let number_pos1 = find_number_pos(ln, false).unwrap();
            let number_pos2 = find_number_pos(ln, true).unwrap();

            let first = get_number_by_pos(&map, ln, number_pos1, first_i, false) * 10;
            let last = get_number_by_pos(&map, ln, number_pos2, last_i, true);

            first + last
        })
        .sum::<u32>();

    println!("part 2: {calibration}");
}

fn find_number_pos(str: &str, rev: bool) -> Option<(&str, Option<usize>)> {
    NUMBERS
        .iter()
        .map(|num| {
            let pos = if rev { str.rfind(num) } else { str.find(num) };
            (*num, pos)
        })
        .max_by(|(_, x_pos), (_, y_pos)| match (x_pos, y_pos) {
            (Some(x), Some(y)) => {
                if rev {
                    x.cmp(&y)
                } else {
                    y.cmp(&x)
                }
            }
            (None, None) => Ordering::Equal,
            (None, _) => Ordering::Less,
            (_, None) => Ordering::Greater,
        })
}

fn get_number_by_pos(
    map: &HashMap<&str, u32>,
    str: &str,
    (num, num_pos): (&str, Option<usize>),
    digit_pos: Option<usize>,
    rev: bool,
) -> u32 {
    if (rev && num_pos.unwrap_or(usize::MIN) > digit_pos.unwrap_or(usize::MIN))
        || (!rev && num_pos.unwrap_or(usize::MAX) < digit_pos.unwrap_or(usize::MAX))
    {
        return *map.get(num).unwrap();
    }

    str.chars()
        .nth(digit_pos.unwrap())
        .unwrap()
        .to_digit(10)
        .unwrap()
}
