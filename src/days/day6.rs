use std::path::Path;

pub fn part1(data_path: &Path) -> u128 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let mut grid: Vec<Vec<&str>> = text.lines().map(|l| l.split_whitespace().collect()).collect();
    let ops = grid.pop().unwrap();
    let mut score = 0;

    for (problem,op) in ops.iter().enumerate() {
        let mut solution = u128::from(*op == "*");

        for row in &grid {
            if *op == "*" {
                solution *= row[problem].parse::<u128>().unwrap();
            } else {
                solution += row[problem].parse::<u128>().unwrap();
            }
        }
        score += solution;
    }
    score
}

pub fn part2(data_path: &Path) -> u128 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let mut grid: Vec<&str> = text.lines().collect();
    let ops_str = grid.pop().unwrap();

    let mut ops: Vec<char> = vec![];
    let mut starts: Vec<usize> = vec![];
    let mut ends: Vec<usize> = vec![];

    let mut last_start = 0;
    ops.push(ops_str.chars().next().unwrap()); //First is always an op
    for i in 1..ops_str.len() {
        match ops_str.chars().nth(i) {
            None => {},
            Some(' ') => {},
            Some(c) => {
                ops.push(c);
                starts.push(last_start);
                ends.push(i-1);
                last_start = i;
            }
        }
    }
    starts.push(last_start);
    ends.push(ops_str.len());

    let mut problems: Vec<Vec<u128>> = vec![];
    for (start, end) in starts.iter().zip(ends.iter()) {
        let mut problem: Vec<u128> = vec![];
        for i in *start..*end {
            let mut str_num: Vec<char> = vec![];
            for row in &grid {
                match row.chars().nth(i) {
                    None => {},
                    Some(' ') => {},
                    Some(c) => { str_num.push(c) },
                }
            }
            problem.push(str_num.iter().collect::<String>().parse().unwrap()); //SO UGLY, FIX THIS!
        }
        problems.push(problem);
    }

    let mut score = 0;
    for (op,problem) in ops.iter().zip(problems) {
        let mut solution = u128::from(*op == '*');
        for n in problem {
            if *op == '*' {
                solution *= n;
            } else {
                solution += n;
            }
        }
        score += solution;
    }

    score
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
123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";
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
        assert_eq!(result, 4_277_556);
    }

    #[test]
    fn test_part2() {
        let (_d, _f, test_path) = create_test_file();
        let result = part2(&test_path);
        assert_eq!(result, 3_263_827);
    }
}
