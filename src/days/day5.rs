use std::path::Path;
use std::cmp;

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let mut score = 0;

    let mut fresh_ranges: Vec<(u128, u128)> = vec![];

    for line in text.split('\n') {
        let line = line.trim();
        if line.len() == 0 { continue }

        if line.contains('-') {
            let mut split_range = line.split('-');
            let start = split_range.next().unwrap().parse().unwrap();
            let end = split_range.next().unwrap().parse().unwrap();
            fresh_ranges.push((start, end));
        } else {
            let n: u128 = line.parse().unwrap();
            for (start,end) in &fresh_ranges {
                if *start <= n && n <= *end {
                    score += 1;
                    break;
                }
            }
        }
    }

    score
}

pub fn part2(data_path: &Path) -> u128 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let mut score = 0;

    // Collect the ranges of fresh IDs
    let mut fresh_ranges: Vec<(u128, u128)> = vec![];
    for line in text.split('\n') {
        let line = line.trim();

        if !line.contains('-') {
            break;
        }
        let mut split_range = line.split('-');
        let start = split_range.next().unwrap().parse().unwrap();
        let end = split_range.next().unwrap().parse().unwrap();
        fresh_ranges.push((start, end));
    }

    // "Merge" the fresh-id ranges for example [(1,10), (5, 15)] --> [(1,15)]
    // 1. Sort the ranges by start positions
    // 2. Loop through, extending the regions and there are two situations to handle
    //    a. The current range overlaps with the prior range: Extend the end 
    //    b. The current range does NOT overlap prior range:  Add the prior range span to score, reset prior range
    // 3. At the end of this loop, we'll always have to add the final prior range span to the score
    fresh_ranges.sort_by(|(s1,_e1), (s2,_e2)| s1.cmp(s2));

    let (mut last_s, mut last_e) = fresh_ranges.remove(0);
    for (s,e) in fresh_ranges {
        if s <= last_e {
            last_e = cmp::max(last_e, e);
        } else {
            score += last_e-last_s+1;
            last_s = s;
            last_e = e;
        }
    }
    score += last_e-last_s+1;
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
3-5
10-14
16-20
12-18

1
5
8
11
17
32";
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
        assert_eq!(result, 3);
    }

    #[test]
    fn test_part2() {
        let (_d, _f, test_path) = create_test_file();
        let result = part2(&test_path);
        assert_eq!(result, 14);
    }
}
