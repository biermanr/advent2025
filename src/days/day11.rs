use std::path::Path;
use std::collections::{HashMap, HashSet};

fn parse_connections(data: &str) -> HashMap<&str, HashSet<&str>> {
    let mut connections: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in data.lines() {
        match line.trim().split_once(": ") {
            Some((upstream, downstreams)) => { 
                connections.insert(upstream, downstreams.split(' ').collect());
            },
            None => { println!("Unable to parse into upstream and downstreams {}", line) },
        }
    }
    connections
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
    let data = std::fs::read_to_string(data_path).unwrap();
    let connections = parse_connections(&data);
    println!("{:?}", connections);
    0
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
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";
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
        assert_eq!(result, 5);
    }
}
