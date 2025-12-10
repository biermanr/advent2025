use std::path::Path;
use std::collections::{VecDeque, HashSet};

fn parse_comma_sep_nums(token: &str) -> Vec<usize> {
    let mut token_iter = token.chars();
    token_iter.next(); //remove start
    token_iter.next_back(); // remove end
    let stripped_t = token_iter.as_str();

    stripped_t.split(',').filter_map(|c| c.parse().ok()).collect()
}

fn parse_line(l: &str) -> (usize, Vec<usize>, Vec<Vec<usize>>, Vec<usize>) {

    let mut tokens = l.trim().split(' ');
    let light_token = tokens.next().unwrap();
    let jolts_token = tokens.next_back().unwrap();

    // Parse the lights portion of the input
    let s_lights:String = light_token.chars()
                                     .filter(|c| *c == '.' || *c == '#')
                                     .map(|c| if c == '.' {'0'} else {'1'}).collect();

    let lights = usize::from_str_radix(&s_lights, 2).expect("Not a binary number!");

    // Parse the joltages
    let jolts = parse_comma_sep_nums(jolts_token);

    // Parse the buttons
    let mut buttons: Vec<Vec<usize>> = vec![];
    let mut bit_buttons: Vec<usize> = vec![];
    let num_lights: usize = s_lights.len().try_into().unwrap();
    for token in tokens {
        let toggled_light_idxs = parse_comma_sep_nums(token);

        let bit_toggled:usize = toggled_light_idxs.iter()
                                        .map(|idx| 2_usize.pow(num_lights.abs_diff(*idx+1).try_into().unwrap()))
                                        .sum::<usize>();

        buttons.push(toggled_light_idxs);
        bit_buttons.push(bit_toggled);
    }

    (lights, bit_buttons, buttons, jolts)
}

pub fn part1(data_path: &Path) -> usize {
    let text = std::fs::read_to_string(data_path).unwrap();
    let mut score = 0;

    for line in text.lines() {
        let (target_state, buttons, _vec_buttons, _jolts) = parse_line(line);

        let mut queue: VecDeque<(usize, usize)> = VecDeque::from([(0, 0)]);
        let mut prior_states: HashSet<usize> = HashSet::new();

        while let Some((current_state, num_presses)) = queue.pop_back() {
            if current_state == target_state {
                score += num_presses;
                break;
            } else {
                for button in &buttons {
                    let new_state = current_state^button;
                    if ! prior_states.contains(&new_state){
                        queue.push_front((new_state, num_presses+1));
                        prior_states.insert(new_state);
                    }
                }
            }
        }
    }
    score
}


pub fn part2(data_path: &Path) -> usize {
    let text = std::fs::read_to_string(data_path).unwrap();
    let mut score = 0;

    for line in text.lines() {
        println!("Working on {}", line);
        let (_target_state, _bit_buttons, buttons, target_jolts) = parse_line(line);

        let mut queue: VecDeque<(Vec<usize>, usize)> = VecDeque::new();
        queue.push_front((vec![0; target_jolts.len()], 0));

        let mut prior_states: HashSet<Vec<usize>> = HashSet::new();

        while let Some((current_jolts, num_presses)) = queue.pop_back() {
            println!("{:?} vs {:?}", current_jolts, target_jolts);
            if current_jolts == target_jolts {
                score += num_presses;
                break;
            } else {
                for button in &buttons {
                    let mut too_large = false;
                    let mut new_jolts = vec![];
                    for (i,current_jolt) in current_jolts.iter().enumerate() {
                        if button.contains(&i) {
                            let new_jolt = current_jolt+1;
                            if new_jolt > target_jolts[i] {
                                too_large = true;
                                break
                            }
                            new_jolts.push(new_jolt);
                        } else {
                            new_jolts.push(*current_jolt);
                        }
                    }

                    if !too_large && !prior_states.contains(&new_jolts){
                        queue.push_front((new_jolts.clone(), num_presses+1));
                        prior_states.insert(new_jolts);
                    }
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

    fn custom_create_test_file() -> (tempfile::TempDir, File, PathBuf) {
        let test_input = "\
[..#.##] (0,1,3,4) (0,3,4) (0,5) (0,1,2) (3,5) (0,2,3,4) (2,3) {58,27,37,57,37,24}";
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

    //#[test]
    //fn test_part2() {
    //    let (_d, _f, test_path) = create_test_file();
    //    let result = part2(&test_path);
    //    assert_eq!(result, 33);
    //}

    #[test]
    fn test_part2_custom() {
        let (_d, _f, test_path) = custom_create_test_file();
        let result = part2(&test_path);
        assert_eq!(result, 33);
    }
}