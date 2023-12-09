use std::{fs, path::PathBuf, collections::HashSet};

/// Scratchcard
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    nums_have: HashSet<u32>,
}

impl Card {
    /// which of the numbers you have appear in the list of winning numbers. 
    /// The first match makes the card worth one point and each match after the first doubles the point value of that card.
    fn get_points(&self) -> u32 {
        let mut total = 0;

        for num in self.nums_have.iter() {
            if self.winning_numbers.contains(num) {
                if total == 0 {
                    total = 1;
                }
                else {
                    total *= 2;
                }
            }
        }

        total
    }
}

fn main() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets\\input.txt");

    let file_path = d.display().to_string();

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let val = process(contents);
    println!("Sum Total {}", val);
}

fn process(contents: String) -> u32 {

    let mut sum_total = 0;
    let lines = contents.lines();
    for line in lines.into_iter() {
        let card = process_line_to_card(line);

        let points = card.get_points();

        if points > 0 {
            println!("Winner card! id:{}, {}", card.id, points);
        }
        sum_total += points;
    }

    sum_total
}

fn process_line_to_card(line: &str) -> Card {

    let card_title_split: Vec<&str> = line.split(':').collect();
    let mut id: u32 = 0;
    // Get id
    for (i, c) in card_title_split[0].char_indices() {
        if c.is_numeric() {
            let substring = &card_title_split[0][i..];
            #[cfg(test)]
            println!("{} -> number index {}, {}", card_title_split[0], i, substring);
            id = substring.parse::<u32>().unwrap();
            break;
        }
    }    
    let numbers_split: Vec<&str> = card_title_split[1].split('|').collect();

    let winning_numbers = parse_numbers(numbers_split[0]);
    let nums_have = parse_numbers(numbers_split[1]);

    let card = Card {
        id: id,
        winning_numbers: winning_numbers,
        nums_have: nums_have,
    };

    card
}

fn parse_numbers(number_line: &str) -> HashSet<u32> {
    
    let split_nums : Vec<&str> = number_line.split(' ').collect();

    let mut num_set = HashSet::<u32>::with_capacity(10);

    for raw_num in split_nums {

        if raw_num.len() < 1 {
            continue; //There are some extra spaces for formatting single digit numbers. Skip them.
        }

        let num = raw_num.parse::<u32>().unwrap();
        num_set.insert(num);
    }

    num_set
}

#[cfg(test)]
mod day_4_tests {
    use std::{fs, path::PathBuf};

    use crate::{process, process_line_to_card};

    #[test]
    fn input_test() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets\\input.txt");

        let file_path = d.display().to_string();

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let val = process(contents.clone());
        assert_ne!(0, val, "Value should not be 0.");
        assert!(13 < val, "Value is larger.");
        assert_eq!(27454, val);

    }

    #[test]
    fn example1_test() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets\\example.txt");

        let file_path = d.display().to_string();

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let val = process(contents.clone());
        assert_eq!(13, val);
    }

    #[test]
    fn process_line_to_card_test() {
        let line = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = process_line_to_card(line);

        assert_eq!(1, card.id);
        assert_eq!(5, card.winning_numbers.len());
        assert_eq!(8, card.nums_have.len());
        assert_eq!(8, card.get_points());
    }

}
