use std::{fs, path::PathBuf, collections::{HashSet, HashMap}};

enum ParsingState {
    Seeds,
    Seed2Soil,
    Soil2Fert,
    Fert2Water,
    Water2Light,
    Light2Temp,
    Temp2Humid,
    Humid2Loc,
}

struct Converter {   
    start_from: u64,
    end_from: u64,

    start_to: u64,
}

impl Converter {
    fn is_in_range(&self, num: u64) -> bool {
        num >= self.start_from && num <= self.end_from
    }

    fn convert(&self, num: u64) -> u64 {
        if !self.is_in_range(num) {
            return num;
        }

        let offset = num - self.start_from;
        return self.start_to + offset;
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

    let mut parsing_state = ParsingState::Seeds;

    let mut seeds = HashSet::new();
    let mut soil_to_fert = HashMap::new();

    let lines = contents.lines();
    for line in lines {

        if line.len() < 1 {
            continue;
        }

        match parsing_state {
            ParsingState::Seeds => {
                parse_seeds(line, &mut seeds);

                parsing_state = ParsingState::Seed2Soil;
            },
            ParsingState::Seed2Soil => {
                if line.starts_with("seed-to-soil") {
                    continue;
                }
                if line.starts_with("soil-to-fertilizer") {
                    parsing_state = ParsingState::Soil2Fert; //next
                    continue;
                }

                parse_conversion_map(line, &mut soil_to_fert);

            },
            ParsingState::Soil2Fert => {
                return todo!();
            },
            ParsingState::Fert2Water => {
                
            },
            ParsingState::Water2Light => {
                
            },
            ParsingState::Light2Temp => {
                
            },
            ParsingState::Temp2Humid => {
                
            },
            ParsingState::Humid2Loc => {
                
            },
        }

        
    }

    todo!("{}", contents)
}

/// Parse => seeds: 79 14 55 13
fn parse_seeds(line: &str, seeds: &mut HashSet<u64>) {

    let split: Vec<&str> = line.split(':').collect();
    parse_numbers(split[1], seeds);
}

fn parse_numbers(number_line: &str, nums: &mut HashSet<u64>) {
    let nums_split: Vec<&str> = number_line.split(' ').collect();

    println!("Parsing number line {}", number_line);
    for raw_num in nums_split {
        if raw_num.len() < 1 {
            continue;
        }

        let num = raw_num.parse::<u64>().unwrap();
        nums.insert(num);
    }
}

/// Convert a line to parse 50 98 2 to 50 -> 98 and 51 -> 99
fn parse_conversion_map(number_line: &str, convesions: &mut HashMap<u64, u64>) {

    let split_nums: Vec<&str> = number_line.split(' ').collect();

    let from_num = split_nums[0].parse::<u64>().unwrap();
    let to_num = split_nums[1].parse::<u64>().unwrap();
    let range = split_nums[2].parse::<u64>().unwrap();

    for r in 0..range {
        convesions.insert(from_num + r, to_num + r);
    }
}

#[cfg(test)]
mod day_5_tests {
    use std::{fs, path::PathBuf};

    use crate::process;

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

    }

    #[test]
    fn example1_test() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets\\example.txt");

        let file_path = d.display().to_string();

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let val = process(contents.clone());
        assert_eq!(35, val);
    }

}
