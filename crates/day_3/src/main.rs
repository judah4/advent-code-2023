use std::{fs, path::PathBuf, vec};

#[derive(Debug)]
enum CellType {
    None,
    Symbol,
    Number,
}

/// Formatted data
struct Cell {
    index: usize,
    id: u32,
    character: char,
    num_value: u32,
    cell_type: CellType,
    adjacent_symbol_indices: Vec<usize>,
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
    let mut cells: Vec<Cell> = Vec::<Cell>::with_capacity(contents.len());
    let mut current_id = 0;
    let lines = contents.lines();
    let mut line_length: usize = 0;
    for (index, line) in lines.enumerate() {
        if line_length == 0 {
            line_length = line.len();
        }

        current_id = create_cells(current_id, &mut cells, line, index);
    }

    check_cell_adjacents(&mut cells, line_length);

    let mut sum_total = 0;

    for cell in cells {
        match cell.cell_type {
            CellType::Number => {}
            _ => continue,
        }

        if cell.adjacent_symbol_indices.len() > 0 {
            sum_total += cell.num_value;
        }
    }

    sum_total
}

fn check_cell_adjacents(cells: &mut Vec<Cell>, line_length: usize) {
    let mut line_offset: usize = 0;
    let mut line_index = 0;
    let max_line = cells.len() / line_length;
    let mut cell_index = 0;
    while cell_index < cells.len() {
        let cell = cells.get(cell_index).unwrap();
        #[cfg(test)]
        println!("{cell_index}, {line_offset}, {line_length}, {line_index}, {:?}", cell.cell_type);

        match cell.cell_type {
            CellType::Number => {

                // Row before
                if line_offset > 0 {
                    if line_index > 0 {
                        check_adjacent(
                            (line_offset - 1) * line_length + line_index - 1,
                            cell_index,
                            cells,
                        );
                    }

                    if line_index + 1 < line_length {
                        check_adjacent(
                            (line_offset - 1) * line_length + line_index + 1,
                            cell_index,
                            cells,
                        );
                    }
                }

                // Curernt row
                if line_index > 0 {
                    check_adjacent(
                        line_offset * line_length + line_index - 1,
                        cell_index,
                        cells,
                    );
                }

                if line_index + 1 < line_length {
                    check_adjacent(
                        line_offset * line_length + line_index + 1,
                        cell_index,
                        cells,
                    );
                }

                // Row after
                if line_offset+1 < max_line {
                    if line_index > 0 {
                        check_adjacent(
                            (line_offset + 1) * line_length + line_index - 1,
                            cell_index,
                            cells,
                        );
                    }

                    if line_index + 1 < line_length {
                        check_adjacent(
                            (line_offset + 1) * line_length + line_index + 1,
                            cell_index,
                            cells,
                        );
                    }
                }
            }
            _ => {}
        }

        line_index += 1;

        if line_index >= line_length {
            line_index = 0;
            line_offset += 1;
        }
        cell_index = line_offset * line_length + line_index;
    }
}

fn check_adjacent(check_cell_index: usize, index: usize, cells: &mut Vec<Cell>) {
    match cells[check_cell_index].cell_type {
        CellType::Symbol => {
            let cell_mut = cells.get_mut(index).unwrap();
            cell_mut.adjacent_symbol_indices.push(check_cell_index);
        }
        _ => {}
    }
}

fn create_cells(mut current_id: u32, cells: &mut Vec<Cell>, line: &str, line_offset: usize) -> u32 {
    let mut current_number: Option<u32> = None;

    for (i, c) in line.chars().enumerate() {
        let mut cell_type = CellType::None;
        if c == '.' {
            current_number = None;
            current_id += 1;
        } else if c.is_numeric() {
            cell_type = CellType::Number;
            if current_number.is_none() {
                current_id += 1;
                // Look ahead for the full number
                let mut num_index_end = i;
                let mut end_char: char = line.chars().nth(num_index_end).unwrap();
                while end_char.is_numeric() && num_index_end + 1 < line.len() {
                    num_index_end += 1;
                    end_char = line.chars().nth(num_index_end).unwrap();
                }
                let sub_num: &str = &line[i..num_index_end];
                current_number = Some(sub_num.parse::<u32>().unwrap());
            }
        } else if !c.is_alphanumeric() {
            cell_type = CellType::Symbol;
        }

        let cell = Cell {
            index: line_offset + i,
            id: current_id,
            character: c,
            cell_type: cell_type,
            adjacent_symbol_indices: vec![],
            num_value: current_number.unwrap_or_default(),
        };
        cells.push(cell);
    }

    current_id
}

#[cfg(test)]
mod tests {
    use std::{fs, path::PathBuf};

    use crate::{create_cells, process, Cell};

    #[test]
    fn input_test() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets\\input.txt");

        let file_path = d.display().to_string();

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let val = process(contents.clone());
        assert_ne!(0, val, "Value should not be 0.");
        assert!(604692 > val, "Value is less than 604692.");
    }

    #[test]
    fn example1_test() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets\\example.txt");

        let file_path = d.display().to_string();

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let val = process(contents.clone());
        assert_eq!(4361, val);
    }

    #[test]
    fn create_cells_test() {
        let line = "467..114..";

        let line_offset = 0;
        let mut cells: Vec<Cell> = Vec::<Cell>::with_capacity(line.len());
        let mut current_id = 0;

        current_id = create_cells(current_id, &mut cells, line, line_offset);

        assert_eq!(line.len(), cells.len());
        assert_ne!(0, current_id);

        let cell = cells.get(0).unwrap();
        assert_eq!(1, cell.id);
        assert_eq!(467, cell.num_value);
        let cell = cells.get(1).unwrap();
        assert_eq!(1, cell.id);
        assert_eq!(467, cell.num_value);
        let cell = cells.get(2).unwrap();
        assert_eq!(1, cell.id);
        assert_eq!(467, cell.num_value);
        let cell = cells.get(3).unwrap();
        assert_eq!(2, cell.id);
        let cell = cells.get(4).unwrap();
        assert_eq!(3, cell.id);
        let cell = cells.get(5).unwrap();
        assert_eq!(4, cell.id);
        assert_eq!(114, cell.num_value);
        let cell = cells.get(6).unwrap();
        assert_eq!(4, cell.id);
        assert_eq!(114, cell.num_value);
        let cell = cells.get(7).unwrap();
        assert_eq!(4, cell.id);
        assert_eq!(114, cell.num_value);
        let cell = cells.get(8).unwrap();
        assert_eq!(5, cell.id);
        let cell = cells.get(9).unwrap();
        assert_eq!(6, cell.id);
    }
}
