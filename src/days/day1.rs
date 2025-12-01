use std::path::Path;

pub fn part1(data_path: &Path) -> u32 {

    let text = std::fs::read_to_string(data_path).unwrap();

    let mut dial = 50;
    let dial_min = 0;
    let dial_max = 99;
    let dial_span = dial_max-dial_min+1;
    let mut num_zeros = 0;

    for line in text.lines() {
        let mut sign = 1;
        if let Some(direction) = line.chars().nth(0) {
            if direction == 'L' {
                sign = -1;
            }
        }

        let magnitude: i32 = line.chars().skip(1).collect::<String>().parse().unwrap();
        let offset = sign*magnitude;

        dial = (dial+offset) % dial_span + dial_min; // TODO this is not general. Fails if dial_min != 0
        if dial < 0 {
            dial += dial_span;
        }

        if dial == 0 {
            num_zeros += 1;
        }
    }

    num_zeros
}

pub fn part2(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();

    let mut dial = 50;
    let dial_min = 0;
    let dial_max = 99;
    let dial_span = (dial_max-dial_min+1) as u32;
    let mut score: u32 = 0;

    for line in text.lines() {
        let mut sign: i32 = 1;
        if let Some(direction) = line.chars().nth(0) {
            if direction == 'L' {
                sign = -1;
            }
        }

        let mut magnitude: u32 = line.chars().skip(1).collect::<String>().parse().unwrap();


        let full_spins = magnitude / dial_span;
        score += full_spins;

        magnitude = magnitude-full_spins*dial_span;
        let offset = sign*(magnitude as i32);
        

        let mut new_dial = dial+offset;
        if new_dial == 0 {
            score += 1;
        }
        else if new_dial < 0 {
            new_dial += dial_span as i32;
            if dial > 0 {
                score += 1;
            }
        }
        else if new_dial >= dial_span as i32 {
            score += 1;
            new_dial = new_dial % (dial_span as i32);
        }
        dial = new_dial;
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
L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

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
        assert_eq!(result, 6);
    }
}
