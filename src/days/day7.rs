use std::path::Path;
use std::collections::{HashSet, HashMap};

fn find_start(grid: &Vec<Vec<char>>) -> (usize, usize) {
    let nrows = grid.len();
    let ncols = grid[0].len();

    // Find (x,y) of start 'S'
    let mut sx = 0;
    let mut sy = 0;
    let mut found_s = false;
    for x in 0..ncols {
        for y in 0..nrows {
            if grid[y][x] == 'S' {
                found_s = true;
                sx = x;
                sy = y;
            }
        }
    }

    if !found_s {
        panic!("Didn't find S starting point!");
    }
    (sx, sy)
}

fn find_splitters(grid: &Vec<Vec<char>>, splitters: &mut HashSet<(usize, usize)>, x: usize, y: usize) {
    if y >= grid.len()-1 { return }

    if grid[y+1][x] == '.' {
        find_splitters(grid, splitters, x, y+1);
    } 
    
    if grid[y+1][x] == '^' && !splitters.contains(&(x,y)) {
        splitters.insert((x,y));
        if x > 0 {
            find_splitters(grid, splitters, x-1, y+1);
        }

        if x <= grid[0].len()-1 {
            find_splitters(grid, splitters, x+1, y+1);
        }
    }
}

fn count_paths(grid: &Vec<Vec<char>>, memo: &mut HashMap<(usize, usize), u128>, x: usize, y: usize) -> u128 {
    if  y >= grid.len()-1 {
        // Reached the bottom, this is one path
        1
    } else if grid[y+1][x] == '^' {
        // Hit a splitter, return num paths left plus num paths right
        if let Some(n_paths) = memo.get(&(x, y)) {
            *n_paths
        } else {
            let left = if x > 0 { count_paths(grid, memo, x-1, y+1) } else { 0 };
            let right = if x < grid[0].len()-1 { count_paths(grid, memo, x+1, y+1) } else { 0 };
            memo.insert((x,y), left+right);
            left+right
        }
    } else {
        // Didn't hit splitter, return num paths after travelling downward
        count_paths(grid, memo, x, y+1)
    }
}

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let grid: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();
    let mut splitters: HashSet<(usize, usize)> = HashSet::new();
    let (sx, sy) = find_start(&grid);
    find_splitters(&grid, &mut splitters, sx, sy);

    splitters.len().try_into().unwrap()
}

pub fn part2(data_path: &Path) -> u128 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let grid: Vec<Vec<char>> = text.lines().map(|l| l.chars().collect()).collect();
    let (sx, sy) = find_start(&grid);

    let mut memo: HashMap<(usize, usize), u128> = HashMap::new();
    count_paths(&grid, &mut memo, sx, sy)
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
        write!(temp_file, "{}", test_input).unwrap();

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
        write!(temp_file, "{}", test_input).unwrap();

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
