use std::{fs, path::PathBuf};

fn main() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets\\input.txt");

    let file_path = d.display().to_string();

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let val = process(contents);
    println!("Sum Total {}", val);
}

fn process(contents: String) -> u32 {
    todo!("{}", contents)
}

#[cfg(test)]
mod day_8_tests {
    use std::{fs, path::PathBuf};

    //use crate::process;

    #[test]
    fn input_test() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets\\input.txt");

        let file_path = d.display().to_string();

        let _contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        //let val = process(contents.clone());
        //assert_ne!(0, val, "Value should not be 0.");
        //assert!(13 < val, "Value is larger.");

    }

    #[test]
    fn example1_test() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets\\example.txt");

        let file_path = d.display().to_string();

        let _contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        //let val = process(contents.clone());
        //assert_eq!(0, val);
    }

}
