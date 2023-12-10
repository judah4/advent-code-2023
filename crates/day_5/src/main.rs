use std::{fs, path::PathBuf, collections::HashSet};

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

struct ConverterRange {   
    source_num: u64,
    dest_num: u64,

    range: u64,
}

impl ConverterRange {
    fn is_in_range(&self, num: u64) -> bool {
        num >= self.source_num && num <= self.source_num + self.range
    }

    fn convert(&self, num: u64) -> Result<u64, String> {
        if !self.is_in_range(num) {
            return Err(format!("number {} is not in range", num));
        }

        let offset = num - self.source_num;
        return Ok(self.dest_num + offset);
    }
}

struct Converter {   
    converters: Vec<ConverterRange>,
}

impl Converter {

    fn convert(&self, num: u64) -> u64 {

        for ranger in self.converters.iter() {
            if ranger.is_in_range(num) {
                return ranger.convert(num).unwrap();
            }
        }
        //Not in range, so it's just a direct map.
        return num;
    }
}

fn main() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets\\input.txt");

    let file_path = d.display().to_string();

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let val = process(contents);
    println!("Lowest Location {}", val);
}

fn process(contents: String) -> u64 {

    let mut parsing_state = ParsingState::Seeds;

    let mut seeds = HashSet::new();
    let mut seed_to_soil = Converter {
        converters: Vec::<ConverterRange>::with_capacity(10),
    };
    let mut soil_to_fert = Converter {
        converters: Vec::<ConverterRange>::with_capacity(10),
    };
    let mut fert_to_water = Converter {
        converters: Vec::<ConverterRange>::with_capacity(10),
    }; 
    let mut water_to_light = Converter {
        converters: Vec::<ConverterRange>::with_capacity(10),
    };
    let mut light_to_temp = Converter {
        converters: Vec::<ConverterRange>::with_capacity(10),
    };
    let mut temp_to_humid = Converter {
        converters: Vec::<ConverterRange>::with_capacity(10),
    };
    let mut humid_to_loc = Converter {
        converters: Vec::<ConverterRange>::with_capacity(10),
    };

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

                seed_to_soil.converters.push(parse_conversion_map(line));

            },
            ParsingState::Soil2Fert => {             
                if line.starts_with("fertilizer-to-water") {
                    parsing_state = ParsingState::Fert2Water; //next
                    continue;
                }
                soil_to_fert.converters.push(parse_conversion_map(line));
            },
            ParsingState::Fert2Water => {                
                if line.starts_with("water-to-light") {
                    parsing_state = ParsingState::Water2Light; //next
                    continue;
                }
                fert_to_water.converters.push(parse_conversion_map(line));
            },
            ParsingState::Water2Light => {                
                if line.starts_with("light-to-temperature") {
                    parsing_state = ParsingState::Light2Temp; //next
                    continue;
                }
                water_to_light.converters.push(parse_conversion_map(line));
            },
            ParsingState::Light2Temp => {                
                if line.starts_with("temperature-to-humidity") {
                    parsing_state = ParsingState::Temp2Humid; //next
                    continue;
                }
                light_to_temp.converters.push(parse_conversion_map(line));
            },
            ParsingState::Temp2Humid => {                
                if line.starts_with("humidity-to-location") {
                    parsing_state = ParsingState::Humid2Loc; //next
                    continue;
                }
                temp_to_humid.converters.push(parse_conversion_map(line));
            },
            ParsingState::Humid2Loc => {
                humid_to_loc.converters.push(parse_conversion_map(line));

                //End State
            },
        }

        
    }

    //get the lowest location number
    let mut location_num = None;

    for seed in seeds {

        let soil = seed_to_soil.convert(seed);
        let fert = soil_to_fert.convert(soil);
        let water =  fert_to_water.convert(fert);
        let light  = water_to_light.convert(water);
        let tempurature = light_to_temp.convert(light);
        let humid = temp_to_humid.convert(tempurature);
        let location = humid_to_loc.convert(humid);

        println!("Seed:{seed}, Soil:{soil}, Fert:{fert}, Water:{water}, Light:{light}, Temp:{tempurature}, Humid:{humid}, Loc:{location}");

        if location_num.is_none() || location_num.unwrap() > location {
            location_num = Some(location);
        }
    }

    location_num.unwrap_or_default()
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
fn parse_conversion_map(number_line: &str) -> ConverterRange {

    println!("parsing line: {}", number_line);

    let split_nums: Vec<&str> = number_line.split(' ').collect();

    let dest_num = split_nums[0].parse::<u64>().unwrap();
    let source_num = split_nums[1].parse::<u64>().unwrap();
    let range = split_nums[2].parse::<u64>().unwrap();

    let converter_range = ConverterRange {
        source_num,
        dest_num,
        range: range,
    };

    converter_range
}

#[cfg(test)]
mod day_5_tests {
    use std::{fs, path::PathBuf};

    use crate::{process, ConverterRange};

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
        assert_eq!(382895070, val);
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

    #[test]
    fn converter_range_test() {
        let converter_range = ConverterRange {
            source_num: 50,
            dest_num: 52,
            range: 48,
        };
        let val = converter_range.convert(55).unwrap();
        assert_eq!(57, val);
    }
}
