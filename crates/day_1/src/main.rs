use std::fs;

fn main() {
    println!("Hello, world!");

    let file_path = "crates/day_1/assets/puzzleinput.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    let val = check_text(contents);
    println!("Computed Value: {}", val);
}

fn check_text(input: String) -> i32 {
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