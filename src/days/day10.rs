use std::path::Path;
use std::collections::HashSet;

fn parse_comma_sep_nums(token: &str) -> Vec<u32> {
    let mut token_iter = token.chars();
    token_iter.next(); //remove start
    token_iter.next_back(); // remove end
    let stripped_t = token_iter.as_str();

    stripped_t.split(',').filter_map(|c| c.parse().ok()).collect()
}

fn parse_line(l: &str) -> (u32, Vec<u32>, Vec<u32>) {

    let mut tokens = l.trim().split(' ');
    let light_token = tokens.next().unwrap();
    let jolts_token = tokens.next_back().unwrap();

    // Parse the lights portion of the input
    let s_lights:String = light_token.chars()
                                     .filter(|c| *c == '.' || *c == '#')
                                     .map(|c| if c == '.' {'0'} else {'1'}).collect();

    let lights = u32::from_str_radix(&s_lights, 2).expect("Not a binary number!");

    // Parse the joltages
    let jolts = parse_comma_sep_nums(jolts_token);

    // Parse the buttons
    let mut buttons: Vec<u32> = vec![];
    let num_lights: u32 = s_lights.len().try_into().unwrap();
    for token in tokens {
        let toggled_light_idxs = parse_comma_sep_nums(token);
        let toggled:u32 = toggled_light_idxs.iter()
                                        .map(|idx| 2_u32.pow(num_lights.abs_diff(*idx+1)))
                                        .sum::<u32>();

        buttons.push(toggled);
    }

    (lights, buttons, jolts)
}

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();
    for line in text.lines() {
        let (target_state, buttons, _jolts) = parse_line(line);

        let mut light_state = 0;
        let mut prior_states: HashSet<u32> = HashSet::from([0]);
        let mut stack: Vec<(u32, u32)> = vec![(0,0)];

        while light_state != target_state {
            break;
        }

    }
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
[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";
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
        assert_eq!(result, 7);
    }
}