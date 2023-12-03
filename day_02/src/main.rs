use std::cmp::Ordering;

use regex::Regex;

fn main() {
    // 1

    let input = include_str!("../input.txt");

    let allowed_values = (12, 13, 14);

    let game_id_sum: usize = input
        .lines()
        .enumerate()
        .map(|(ln_i, ln)| {
            let mut game_id = ln_i + 1;
            let color_start = ln.find(":").unwrap();

            let colors = ln[color_start..].split(";");

            colors.for_each(|color| {
                if parse_number(color, "red") > allowed_values.0
                    || parse_number(color, "green") > allowed_values.1
                    || parse_number(color, "blue") > allowed_values.2
                {
                    game_id = 0;
                }
            });

            game_id
        })
        .sum();

    println!("1: {game_id_sum}");

    // 2

    let power_sum: usize = input
        .lines()
        .map(|ln| {
            let color_start = ln.find(":").unwrap();

            let colors = ln[color_start..].split(";");

            let mut r = 0;
            let mut g = 0;
            let mut b = 0;

            colors.for_each(|color| {
                let p_r = parse_number(color, "red");
                if p_r > r {
                    r = p_r
                };

                let p_g = parse_number(color, "green");
                if p_g > g {
                    g = p_g
                };

                let p_b = parse_number(color, "blue");
                if p_b > b {
                    b = p_b
                };
            });

            r * g * b
        })
        .sum();

    println!("2: {power_sum}");
}

fn parse_number(ln: &str, name: &str) -> usize {
    let regex = format!("(?P<{name}>\\d+) {name}");
    let pattern = Regex::new(&regex).unwrap();

    if let Some(captures) = pattern.captures(ln) {
        let number: usize = captures.name(name).unwrap().as_str().parse().unwrap();

        return number;
    }

    0
}
