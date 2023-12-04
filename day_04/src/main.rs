use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let total_points: u32 = input
        .lines()
        .map(|ln| {
            let start = ln.find(":").unwrap();

            let (winning, my) = ln[start + 1..].split_once("|").unwrap();

            let winning: HashSet<&str> = winning.split_whitespace().collect();
            let my = my.split_whitespace();

            let mut exponent = None;

            my.for_each(|num| {
                if winning.contains(num) {
                    exponent = match exponent {
                        Some(exp) => Some(exp + 1),
                        None => Some(0),
                    }
                }
            });

            let base: u32 = 2;

            match exponent {
                Some(exp) => base.pow(exp),
                _ => 0,
            }
        })
        .sum();

    println!("1: {total_points}");

    // 2

    let mut total_cards: usize = input
        .lines()
        .enumerate()
        .map(|(ln_i, ln)| count_cards(ln_i, ln, &input.lines().collect::<Vec<&str>>()))
        .sum();

    total_cards += input.lines().count();

    println!("2: {total_cards}");
}

fn count_cards(ln_i: usize, ln: &str, cards: &Vec<&str>) -> usize {
    let start = ln.find(":").unwrap();

    let (winning, my) = ln[start + 1..].split_once("|").unwrap();

    let winning: HashSet<&str> = winning.split_whitespace().collect();
    let my = my.split_whitespace();

    let mut total = 0;
    let mut bonus = 0;

    my.for_each(|num| {
        if winning.contains(num) {
            total += 1;

            bonus += count_cards(ln_i + total, cards.get(ln_i + total).unwrap(), cards);
        }
    });

    total + bonus
}
