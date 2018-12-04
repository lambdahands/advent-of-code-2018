mod day1;
mod day2;
mod day3;

use std::env;

fn main() {
    let arg = env::args().nth(1);
    let prog = arg.as_ref().map(String::as_str);
    match prog {
        Some("day1") => day1::main(),
        Some("day2") => day2::main(),
        Some("day3") => day3::main(),
        Some(_) => println!("No days matched!"),
        _ => println!("Please enter day")
    }
}


