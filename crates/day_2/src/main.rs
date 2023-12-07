use std::{path::PathBuf, fs};


const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

#[derive(Debug)]
struct GameData {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

fn main() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets\\input.txt");

    let file_path = d.display().to_string();

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let val = process_games(contents);
    println!("{}", val);
}

/// Iterate though the lines, format the games, and check if the game fits the expected numbers.
fn process_games(games: String) -> u32 {

    let mut game_ids_total: u32 = 0;
    let mut valid_games = vec![];
    for line in games.lines().into_iter() {
        let game = process_game(line);

        let qualifies = game_qualifies(&game);
        #[cfg(test)]
        println!("{} => {:?}, Qualifies: {}", line, game, qualifies);
        if qualifies {
            game_ids_total += game.id;
            valid_games.push(game);
        }
    }

    game_ids_total
}

/// Take the game string and format into structured game data.
fn process_game(game: &str) -> GameData {

    let mut game_data = GameData {
        id: 0,
        red: 0,
        green: 0,
        blue: 0,
    };

    let game_split: Vec<&str> = game.split(':').collect();

    // Get game id
    for (i, c) in game_split[0].char_indices() {
        if c.is_numeric() {
            let substring = &game_split[0][i..];
            #[cfg(test)]
            println!("{} -> number index {}, {}", game_split[0], i, substring);
            game_data.id = substring.parse::<u32>().unwrap();
            break;
        }
    }

    let game_grabs: Vec<&str> = game_split[1].split(';').collect();

    for grab in game_grabs.iter() {
        let colors_with_counts: Vec<&str> = grab.split(',').collect();
        for color_data in colors_with_counts.iter() {
            let color_data_split: Vec<&str> = color_data.trim().split(' ').collect();
            let num = color_data_split[0].parse::<u32>().unwrap();
            match color_data_split[1] {
                "red" => {
                    if num > game_data.red {
                        game_data.red = num;
                    }
                }
                "green" => {
                    if num > game_data.green {
                        game_data.green = num;
                    }
                }
                "blue" => {
                    if num > game_data.blue {
                        game_data.blue = num;
                    }
                }
                _ => panic!("Value could not be parsed properly: {}", grab)
            }
        }
    }

    game_data
}

fn game_qualifies(game: &GameData) -> bool {
    if game.red > MAX_RED {
        return false;
    }
    if game.green > MAX_GREEN {
        return false;
    }
    if game.blue > MAX_BLUE {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, fs};

    use crate::{process_games, process_game};

    #[test]
    fn input_test() {

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets\\input.txt");
    
        let file_path = d.display().to_string();
    
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        
        let val = process_games(contents.clone());
        assert_eq!(true, val > 272, "Value should be greater than 272");
    }

    #[test]
    fn example1_test() {

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets\\example.txt");
    
        let file_path = d.display().to_string();
    
        let contents = fs::read_to_string(file_path)
            .expect("Should have been able to read the file");
        
        let val = process_games(contents.clone());
        assert_eq!(8, val);
    }

    #[test]
    fn progress_game_test() {

        let contents = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        
        let val = process_game(contents);
        assert_eq!(3, val.id);
        assert_eq!(20, val.red);
        assert_eq!(13, val.green);
        assert_eq!(6, val.blue);
    }

    #[test]
    fn progress_game_test2() {

        let contents = "Game 13: 8 green, 7 blue, 12 red; 8 blue, 5 red, 15 green; 5 green, 1 red";
        
        let val = process_game(contents);
        assert_eq!(13, val.id);
        assert_eq!(12, val.red);
        assert_eq!(15, val.green);
        assert_eq!(8, val.blue);
    }
}
