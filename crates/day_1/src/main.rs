use std::fs;

use regex::Regex;

fn main() {
    println!("Hello, world!");

    let file_path = "crates/day_1/assets/example2.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    let val = check_text_part_1(contents.clone());
    println!("Computed Value: {}", val);

    let part2 = check_text_part_2(contents.clone());
    println!("Part 2 Value: {}", part2);
}

fn check_text_part_1(input: String) -> i32 {
    let mut total_value: i32 = 0;
    let mut first_char: Option<char> = None;
    let mut last_char: Option<char> = None;
    for c in input.chars() {
        if c.is_numeric() {
            if first_char.is_none() {
                // Set first and last
                first_char = Some(c);
                last_char = Some(c);
            }
            else {
                // Set the last
                last_char = Some(c);
            }
        }
        else if c == '\n' {
            //compute!
            let val = compute_chars_to_i32(first_char, last_char);
            println!("Val: {}", val);
            total_value += val;
            //reset
            first_char = None;
            last_char = None;
        }
    }

    if first_char.is_some() {
        //final row
        let val = compute_chars_to_i32(first_char, last_char);
        total_value += val;
    }

    total_value
}

fn compute_chars_to_i32(first_char: Option<char>, last_char: Option<char>) -> i32 {
    if first_char.is_none() {
        // Nothing set, default to 0.
        return 0;
    }

    let str_val = format!("{}{}", first_char.unwrap(), last_char.unwrap());
    let val = str_val.parse::<i32>().unwrap();
    return val;
}

fn check_text_part_2(input: String) -> i32 {

    let re = Regex::new(r"(?m)^([^:]+):([0-9]+):(.+)$").unwrap();

    let mut results = vec![];
    for (_, [path, lineno, line]) in re.captures_iter(&input).map(|c| c.extract()) {
        results.push((path, lineno.parse::<i32>(), line));
    }

    let mut total_value: i32 = 0;

    total_value
}