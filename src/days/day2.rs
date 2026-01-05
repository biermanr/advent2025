use std::path::Path;

pub fn part1(data_path: &Path) -> u128 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let text = text.trim();
    let mut score: u128 = 0;

    for range in text.split(','){
        let mut start_end = range.split('-');
        let start: u128 = start_end.next().unwrap().parse().unwrap();
        let end: u128 = start_end.next().unwrap().parse().unwrap();

        for n in start..=end {
            let n_string = n.to_string();
            let num_digits = n_string.len();
            if num_digits % 2 == 1 {
                continue
            }

            if n_string[..num_digits/2] == n_string[num_digits/2..] {
                score += n;
            }
        }
    }
    score
}

pub fn part2(data_path: &Path) -> u128 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let text = text.trim();
    let mut score: u128 = 0;

    for range in text.split(','){
        let mut start_end = range.split('-');
        let start: u128 = start_end.next().unwrap().parse().unwrap();
        let end: u128 = start_end.next().unwrap().parse().unwrap();

        // Check if each substring can be repeated to match the full string?
        for n in start..=end {
            let n_string = n.to_string();

            let mut passes = false;
            for substring_len in 1..=(n_string.len() / 2) {
                let max_repeats = n_string.len()/substring_len;
                for num_repeats in 1..=max_repeats {
                    let substring = n_string[..substring_len].repeat(num_repeats);
                    if substring == n_string {
                        score += n;
                        passes = true;
                        break;
                    }
                }

                if passes {
                    break
                }
            }
        }
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
        let test_input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
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
        assert_eq!(result, 1_227_775_554);
    }

    #[test]
    fn test_part2() {
        let (_d, _f, test_path) = create_test_file();
        let result = part2(&test_path);
        assert_eq!(result, 4_174_379_265);
    }
}
