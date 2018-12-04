use std::collections::HashMap;

type Counts = HashMap<char, u32>;
type CountList = Vec<Counts>;

fn build_freqs(text: &str) -> CountList {
    let mut hmap_list: CountList = Vec::new();
    for line in text.lines() {
        let mut hmap: Counts = HashMap::new();
        for ch in line.chars() {
            *hmap.entry(ch).or_insert(0) += 1
        }
        hmap_list.push(hmap);
    }
    hmap_list
}

fn string_diff(string_a: &str, string_b: &str) -> (String, usize) {
    let common = string_a.chars().zip(string_b.chars()).filter_map(|(a, b)| {
        if a == b { Some(a) } else { None }
    }).collect::<String>();
    let diff_count = string_a.len() - common.len();
    (common, diff_count)
}

fn calc_checksum(text: &str) -> usize {
    let freqs  = build_freqs(text);
    let twos   = freqs.iter().filter(|m| m.values().any(|v| *v == 2)).count();
    let threes = freqs.iter().filter(|m| m.values().any(|v| *v == 3)).count();
    twos * threes
}

fn find_id(text: &str) -> Option<String> {
    text.lines().find_map(|line_a| {
        text.lines().find_map(|line_b| {
            match string_diff(line_a, line_b) {
                (string, 1) => Some(string),
                _           => None
            }
        })
    })
}

pub fn main() {
    let text = include_str!("../resources/day2.txt");
    println!("Checksum: {}", calc_checksum(text));
    find_id(text).map(|i| println!("Common ID Letters: {}", i));
}
