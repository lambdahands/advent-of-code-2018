/// Advent of Code Day 1
/// https://adventofcode.com/2018/day/1
use std::collections::HashSet;

fn parse_line_num(line: &str) -> Option<i32> {
    line.get(1..).and_then(|s| s.parse::<i32>().ok())
}

fn update_total(op: &str, total: &mut i32, num: &i32) {
    match op {
        "+" => *total += num,
        "-" => *total -= num,
        _   => ()
    }
}

fn parse_and_update(line: &str, total: &mut i32) {
    match (line.get(0..1), parse_line_num(line)) {
        (Some(op), Some(num)) => update_total(op, total, &num),
        (_, _)                => ()
    }
}

fn run_adjustment(text: &str) -> i32 {
    let mut total: i32 = 0;
    for line in text.lines() {
        parse_and_update(line, &mut total);
    }
    total
}

fn find_duplicate(text: &str) -> i32 {
    let mut total: i32 = 0;
    let mut duplicates = HashSet::new();
    for line in text.lines().cycle() {
        parse_and_update(line, &mut total);
        if !duplicates.insert(total) { break; }
    }
    total
}

pub fn main() {
    let text = include_str!("../resources/day1.txt");
    println!("Total: {}", run_adjustment(text));
    println!("Duplicate: {}", find_duplicate(text));
}
