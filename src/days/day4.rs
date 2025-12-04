use std::path::Path;

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let diagram: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();
    let mut score = 0;

    for y in 0..diagram.len() {
        for x in 0..diagram[0].len() {
            if diagram[y][x] != '@' {
                continue;
            }
            let num_adj = count_adjacent(&diagram, x, y);
            if num_adj < 4 {
                score += 1;
            }
        }
    }
    score
}

pub fn part2(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let mut diagram: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();
    let mut score = 0;

    loop {
        let mut removable_positions: Vec<(usize, usize)> =  vec![];

        for y in 0..diagram.len() {
            for x in 0..diagram[0].len() {
                if diagram[y][x] != '@' {
                    continue;
                }
                let num_adj = count_adjacent(&diagram, x, y);
                if num_adj < 4 {
                    removable_positions.push((x,y));
                    score += 1;
                }
            }
        }

        if removable_positions.len() == 0 {
            break
        }

        for (x,y) in removable_positions {
            diagram[y][x] = '.';
        }
    }
    score
}

fn count_adjacent(diagram: &Vec<Vec<char>>, x: usize, y: usize) -> u8 {
    let mut num_adj = 0;
    let max_y = diagram.len() - 1;
    let max_x = diagram[0].len() - 1;

    // Check up-left
    if x > 0 && y > 0 && diagram[y-1][x-1] == '@' { num_adj += 1 }

    // Check directly up
    if y > 0 && diagram[y-1][x] == '@' { num_adj += 1 }

    // Check up-right
    if x < max_x && y > 0 && diagram[y-1][x+1] == '@' { num_adj += 1}

    // Check left
    if x > 0 && diagram[y][x-1] == '@' { num_adj += 1 }

    // Check right
    if x < max_x && diagram[y][x+1] == '@' { num_adj += 1 }

    // Check down-left
    if x > 0 && y < max_y && diagram[y+1][x-1] == '@' { num_adj += 1 }

    // Check directly down
    if y < max_y && diagram[y+1][x] == '@' { num_adj += 1 }

    // Check down-right
    if x < max_x && y < max_y && diagram[y+1][x+1] == '@' { num_adj += 1}

    num_adj
}

// Test the run function
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn create_test_file() -> (tempfile::TempDir, File, PathBuf) {
        let test_input = "\
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";
        let temp_dir = tempdir().unwrap();
        let f_path = temp_dir.path().join("test_input.txt");
        let mut temp_file = File::create(f_path.clone()).unwrap();
        write!(temp_file, "{}", test_input).unwrap();

        // have to return dir and file so they don't go out of scope
        (temp_dir, temp_file, f_path)
    }

    #[test]
    fn test_part1() {
        let (_d, _f, test_path) = create_test_file();
        let result = part1(&test_path);
        assert_eq!(result, 13);
    }

    #[test]
    fn test_part2() {
        let (_d, _f, test_path) = create_test_file();
        let result = part2(&test_path);
        assert_eq!(result, 43);
    }
}
