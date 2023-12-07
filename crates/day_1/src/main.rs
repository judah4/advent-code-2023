use std::fs;

use regex::Regex;

fn main() {

    let file_path = "crates/day_1/assets/puzzleinput.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    println!("With text:\n{contents}");

    let val = check_text_part_1(contents.clone());

    let part2 = check_text_part_2(contents.clone());

    println!("Part 1 Value: {}", val);
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

    let mut total_value: i32 = 0;

    let re = Regex::new(r"^(\d|one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let lines = input.lines();

    for line in lines {
        let mut results = vec![];

        // This for loop is so bad but it makes it so overlaps work properly. See example2-extended.txt
        for cap in line.char_indices().filter_map(|(i,_)| re.captures(&line[i..]))  {
            let num_str = cap[0].to_string();
            let val = string_to_num_char(&num_str);
            if val.is_some() { 
                results.push(val.unwrap());
            }
            #[cfg(test)]
            println!("{} => {}", num_str, results.last().unwrap())
        }

        let first_char = results.first();
        let last_char = results.last();
        if first_char.is_some() {
            let char1 = *first_char.unwrap();
            let char2 = *last_char.unwrap();
            //rewrap
            let val = compute_chars_to_i32(Some(char1), Some(char2));
            #[cfg(test)]
            println!("line: {}, vals: {},{} => {}", line, char1, char2, val);

            total_value += val;
        }
    }

    total_value
}

fn string_to_num_char(num_str: &str) -> Option<char> {

    if num_str.len() == 1 {
        //numeric
        let chars = num_str.chars().into_iter();
        for c in chars {
            if c.is_numeric() {
                return Some(c);
            }
        }
    }

    match num_str {
        "zero" => Some('0'),
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use crate::{check_text_part_1, check_text_part_2};

    #[test]
    fn check_text_part_1_test() {

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets\\example.txt");
    
        let file_path = d.display().to_string();
    
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        
        let val = check_text_part_1(contents.clone());
        assert_eq!(142, val);
    }

    #[test]
    fn check_text_part_2_test() {

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets\\example2.txt");

        let file_path = d.display().to_string();
    
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
      
        let val = check_text_part_2(contents.clone());
        assert_eq!(281, val);
    }

    #[test]
    fn check_text_part_2_extended_test() {

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets\\example2-extended.txt");

        let file_path = d.display().to_string();
    
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
      
        let val = check_text_part_2(contents.clone());
        assert_eq!(285, val);
    }

    #[test]
    fn check_text_part_2_full_test() {

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets\\puzzleinput.txt");

        let file_path = d.display().to_string();
    
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
      
        let val = check_text_part_2(contents.clone());
        assert_ne!(54506, val);
        assert_ne!(54704, val);
    }

}
