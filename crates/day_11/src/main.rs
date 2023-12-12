use std::{fs, path::PathBuf, collections::{HashSet, HashMap}};
use pathfinding::prelude::astar;

struct UnexpandedSpace {
    space: Vec<char>,
    width: usize,
    height: usize,
    rows_to_expand: HashSet<usize>,
    columns_to_expand: HashSet<usize>,
}

struct ExpandedSpace {
    galaxies: HashMap<Vector2, Galaxy>,
}
impl ExpandedSpace {
    /// What moves are valid fom this position
    fn get_successors(&self, p: &Vector2) -> Vec<(Vector2, u32)> {

        //cardinal values
        let mut successors = vec![];
        if p.x > 0 {
            successors.push(Vector2 {x: p.x - 1, y: p.y});
        }
        if p.y > 0 {
            successors.push(Vector2 {x: p.x, y: p.y - 1});
        }

        successors.push(Vector2 {x: p.x + 1, y: p.y});
        successors.push(Vector2 {x: p.x, y: p.y + 1});

        successors.into_iter().map(|p| {
            //let mut bias = 1;
            //if self.galaxies.contains_key(&p) {
            //    bias = 100;
            //}
            (p, 1)
        }).collect()
    }
}

#[derive(Debug)]
struct Galaxy {
    position: Vector2,
    _id: u32,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Vector2 {
    x: u32,
    y: u32,
}

impl Vector2 {
    fn distance(&self, other: &Vector2) -> u32 {
        (self.x.abs_diff(other.x) + self.y.abs_diff(other.y)) as u32
      }
    
}

fn main() {
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push("assets\\input.txt");

    let file_path = d.display().to_string();

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let expansion_size = 1_000_000;

    println!("Expanding the universe at {} rate", expansion_size);

    let val = process(contents, expansion_size);
    println!("Sum Total {}", val);
}

fn process(contents: String, expansion_size: u32) -> u32 {

    // Do an expansion pass and then look up pass.
    
    let unexpanded_space = parse_contents(&contents);

    let expanded_space = expand_space(&unexpanded_space, expansion_size);

    let shortest_paths = get_shortest_paths(&expanded_space);

    let mut total = 0;

    for path in shortest_paths {
        total += path;
    }

    total
}

fn parse_contents(contents: &String) -> UnexpandedSpace {

    let mut space_lines = Vec::with_capacity(contents.len());
    let mut columns_expand = HashSet::new();
    let mut rows_expand = HashSet::new();
    let lines = contents.lines();
    let mut add_columns = true;
    let mut width = 0;
    let mut height = 0;
    for (row, line) in lines.enumerate() {
        let mut row_empty_space = true;
        height += 1; // Naive height, just set each time we iterate
        for (column, c) in line.chars().enumerate() {
            if column >= width {
                width = column + 1;
            }

            space_lines.push(c);
            if c != '.' {
                row_empty_space = false;
                columns_expand.remove(&column);
            }
            else if add_columns {
                //empty column, add for the first time.
                columns_expand.insert(column);
            }
        }
        if row_empty_space {
            rows_expand.insert(row);
        }
        //only add columns for the first row
        add_columns = false;
    }

    UnexpandedSpace {
        space: space_lines,
        width: width,
        height: height,
        rows_to_expand: rows_expand,
        columns_to_expand: columns_expand,
    }
}

fn expand_space(unexpanded_space: &UnexpandedSpace, expansion_size: u32) -> ExpandedSpace {
    
    let mut galaxies =  HashMap::new();
    let mut expanded_y: u32 = 0;
    let mut galaxy_id = 1;

    for y in 0..unexpanded_space.height {
        let mut expanded_x: u32 = 0;
        for x in 0..unexpanded_space.width {
            let space_tile = unexpanded_space.space[y * unexpanded_space.height + x];

            if space_tile == '#' {
                let pos = Vector2 { x: expanded_x, y: expanded_y};
                galaxies.insert(pos.clone(), Galaxy {
                    _id: galaxy_id,
                    position: pos,
                });
                galaxy_id += 1;
            }

            let mut size_x = 1;
            if unexpanded_space.columns_to_expand.contains(&x) {
                size_x *= expansion_size;
            }

            expanded_x += size_x;
        }

        let mut size_y = 1;
        if unexpanded_space.rows_to_expand.contains(&y) {
            size_y *= expansion_size;
        }

        expanded_y += size_y;
    }

    ExpandedSpace { galaxies }
}

fn get_shortest_paths(expanded_space: &ExpandedSpace) -> Vec<u32> {

    let mut shortest_paths = vec![];

    let galaxies: Vec<&Galaxy> = expanded_space.galaxies.values().collect();
    for g1 in 0..galaxies.len() {
        let galaxy1 = galaxies[g1];

        // Go through each galaxy pair until none are left to be paired
        for g2 in g1+1..galaxies.len() {
            let galaxy2 = galaxies[g2];

            let result = astar(
                &galaxy1.position, 
                |p| expanded_space.get_successors(&p),
                |p| p.distance(&galaxy2.position),
                |p| *p == galaxy2.position);
                let nodes: u32 = result.expect("no path found.").0.len().try_into().unwrap();
                //let nodes: u32 = result.expect("no path found.").1;
                //remove the begining node count.
                //#[cfg(test)]
                println!("Shortest path between: {:?} and {:?} is {}", galaxy1, galaxy2, nodes - 1);
                shortest_paths.push(nodes - 1)
        }
    }

    shortest_paths
}

#[cfg(test)]
mod day_11_tests {
    use std::{fs, path::PathBuf};

    use crate::process;

    #[test]
    fn example1_test() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets\\example.txt");

        let file_path = d.display().to_string();

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let val = process(contents.clone(), 2);
        assert_eq!(374, val);
    }

    #[test]
    fn example1_10_test() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets\\example.txt");

        let file_path = d.display().to_string();

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let val = process(contents.clone(), 10);
        assert_eq!(1030, val);
    }

    #[test]
    fn example1_1000_test() {
        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("assets\\example.txt");

        let file_path = d.display().to_string();

        let contents =
            fs::read_to_string(file_path).expect("Should have been able to read the file");

        let val = process(contents.clone(), 100);
        assert_eq!(8410, val);
    }

}
