use std::path::Path;
use std::collections::HashMap;

fn find_start(grid: &Vec<Vec<char>>) -> Option<(usize, usize)> {
    for x in 0..grid[0].len() {
        for y in 0..grid.len() {
            if grid[y][x] == 'S' {
                return Some((x, y));
            }
        }
    }
    None
}

fn traverse(grid: &Vec<Vec<char>>, memo: &mut HashMap<(usize, usize), u128>, x: usize, y: usize) -> u128 {
    if  y >= grid.len()-1 {
        // Reached the bottom, this is one path
        1
    } else if grid[y+1][x] == '^' {
        // Hit a splitter, return num paths left plus num paths right
        if let Some(n_paths) = memo.get(&(x, y)) {
            *n_paths
        } else {
            let left = if x > 0 { traverse(grid, memo, x-1, y+1) } else { 0 };
            let right = if x < grid[0].len()-1 { traverse(grid, memo, x+1, y+1) } else { 0 };
            memo.insert((x,y), left+right);
            left+right
        }
    } else {
        // Didn't hit splitter, return num paths after travelling downward
        traverse(grid, memo, x, y+1)
    }
}

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let grid: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();
    let (sx, sy) = find_start(&grid).unwrap();
    let mut memo: HashMap<(usize, usize), u128> = HashMap::new();

    traverse(&grid, &mut memo, sx, sy);
    memo.len().try_into().unwrap()
}

pub fn part2(data_path: &Path) -> u128 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let grid: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();
    let (sx, sy) = find_start(&grid).unwrap();
    let mut memo: HashMap<(usize, usize), u128> = HashMap::new();

    traverse(&grid, &mut memo, sx, sy)
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
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";
        let temp_dir = tempdir().unwrap();
        let f_path = temp_dir.path().join("test_input.txt");
        let mut temp_file = File::create(f_path.clone()).unwrap();
        write!(temp_file, "{test_input}").unwrap();

        // have to return dir and file so they don't go out of scope
        (temp_dir, temp_file, f_path)
    }

    #[test]
    fn test_part1() {
        let (_d, _f, test_path) = create_test_file();
        let result = part1(&test_path);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_part2() {
        let (_d, _f, test_path) = create_test_file();
        let result = part2(&test_path);
        assert_eq!(result, 40);
    }

    fn custom_test(test_input: &str) -> (tempfile::TempDir, File, PathBuf) {
        let temp_dir = tempdir().unwrap();
        let f_path = temp_dir.path().join("test_input.txt");
        let mut temp_file = File::create(f_path.clone()).unwrap();
        write!(temp_file, "{test_input}").unwrap();

        // have to return dir and file so they don't go out of scope
        (temp_dir, temp_file, f_path)
    }

    #[test]
    fn test_part2_p1() {
        let test_input = "\
.......S.......
...............
.......^.......
...............";
        let (_d, _f, test_path) = custom_test(test_input);
        let result = part2(&test_path);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2_p2() {
        let test_input = "\
.......S.......
...............
.......^.......
...............
.......^.......
...............";
        let (_d, _f, test_path) = custom_test(test_input);
        let result = part2(&test_path);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_part2_p3() {
        let test_input = "\
.......S.......
...............
.......^.......
...............
......^........
...............";
        let (_d, _f, test_path) = custom_test(test_input);
        let result = part2(&test_path);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part2_p4() {
        let test_input = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............";
        let (_d, _f, test_path) = custom_test(test_input);
        let result = part2(&test_path);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part2_p5() {
        let test_input = "\
.......S.......
...............
.......^.......
...............
......^........
...............
.......^.......
...............";
        let (_d, _f, test_path) = custom_test(test_input);
        let result = part2(&test_path);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_part2_p6() {
        let test_input = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.......^.......
...............";
        let (_d, _f, test_path) = custom_test(test_input);
        let result = part2(&test_path);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part2_p7() {
        let test_input = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^...^.....
...............";
        let (_d, _f, test_path) = custom_test(test_input);
        let result = part2(&test_path);
        assert_eq!(result, 6);
    }

    #[test]
    fn test_part2_p8() {
        let test_input = "\
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^...^.....
...............
....^.^.^.^....
...............";
        let (_d, _f, test_path) = custom_test(test_input);
        let result = part2(&test_path);
        assert_eq!(result, 10);
    }
}
