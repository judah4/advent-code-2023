use std::fs;

fn main() {
    println!("Hello, world!");

    let file_path = "crates/day_1/assets/example.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    let str_val = check_text("trebuch7et".to_string());
    let val = str_val.parse::<i32>().unwrap();
    println!("{}, {}", str_val, val);
}

fn check_text(input: String) -> String {
    let mut first_char: Option<char> = None;
    let mut last_char: Option<char> = None;
    for c in input.chars() {
        if c.is_numeric() {
            if first_char.is_none() {
                first_char = Some(c);
            }
            else {
                last_char = Some(c);
            }
        }
    }

    if first_char.is_none() {
        return "".to_string();
    }
    if last_char.is_none() {
        return first_char.unwrap().to_string();
    }

    return format!("{}{}", first_char.unwrap(), last_char.unwrap());
}