use std::path::Path;

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let mut score = 0;

    for line in text.lines() {
        // Get the left-most index of the largest value excluding the end
        let line = line.trim();
        let mut max_ind = 0;
        let mut max_val = 0;
        for (index, c) in line.chars().enumerate() {
            // Skip the last index
            if index == line.len()-1 {
                break
            }
            let numeric_c = c.to_digit(10).unwrap();
            if numeric_c > max_val {
                max_ind = index;
                max_val = numeric_c;
            }
        }

        // Starting at +1 of the max_ind, find the next largest value
        let mut next_largest = 0;
        for (index, c) in line.chars().enumerate() {
            if index <= max_ind {
                continue
            }
            let numeric_c = c.to_digit(10).unwrap();
            if numeric_c > next_largest {
                next_largest = numeric_c;
            }
        }

        score += 10*max_val + next_largest;
    }

    score
}

pub fn part2(data_path: &Path) -> u128 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let battery_banks: Vec<Vec<u8>> = text.lines().map(|line| line.trim().chars().map(|c| c.to_digit(10).unwrap() as u8).collect()).collect();
    let mut score: u128 = 0;

    for batteries_vec in battery_banks {
        let mut batteries = &batteries_vec[..]; // ugly hack
        let mut max_val = 0;
        for nth in (1..=12).rev() {
            (max_val, batteries) = pop_largest(&batteries, nth);
            score += (max_val as u128)*10_u128.pow((nth-1).try_into().unwrap());
        }
    }
    score
}

fn pop_largest(batteries: &[u8], nth: u8) -> (u8, &[u8]) {
    // Get the left-most index of the largest value excluding the last n-1 regions
    // Return the largest value and the remaining bank of batteries
    let mut max_ind = 0;
    let mut max_val = 0;
    let check_till = batteries.len()-(nth as usize)+1;

    for ind in 0..check_till {
        if batteries[ind] > max_val {
            max_ind = ind;
            max_val = batteries[ind];
        }
    }

    (max_val, &batteries[max_ind+1..])
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
987654321111111
811111111111119
234234234234278
818181911112111";
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
        assert_eq!(result, 357);
    }

    #[test]
    fn test_part2() {
        let (_d, _f, test_path) = create_test_file();
        let result = part2(&test_path);
        assert_eq!(result, 3121910778619);
    }
    
}
