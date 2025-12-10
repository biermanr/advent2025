use std::path::Path;

fn parse_line(l: &str) {

    let mut lights = 0;
    let mut num_lights = 0;
    //let mut buttons = vec![];
    //let mut joltages = vec![];

    let mut tokens = l.trim().split(' ');
    let light_token = tokens.next().unwrap();
    let jolts_token = tokens.next_back().unwrap();

    for token in l.trim().split(' ') {
        let mut token_iter = token.chars();
        let start = token_iter.next(); //remove start
        token_iter.next_back(); // remove end
        let stripped_t = token_iter.as_str();

        match start {
            Some('[') => {
                let s_lights:String = stripped_t.chars()
                                                .filter(|c| *c == '.' || *c == '#')
                                                .map(|c| if c == '.' {'0'} else {'1'}).collect();

                num_lights = s_lights.len();
                lights = u16::from_str_radix(&s_lights, 2).expect("Not a binary number!");
            },
            Some('(') => {
                let s_buttons:Vec<u8> = stripped_t.split(',')
                                                   .filter_map(|c| c.parse().ok())
                                                   .collect();

                println!("Button {} has nums {:?}", token, s_buttons);

            },
            Some('{') => {
                //[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
                println!("Token {} is JOLTAGE", token);
            },
            Some(_) => println!("ERROR, UNEXPECTED start for token {}", token),
            None => println!("ERROR, NO TOKEN!")
        }
    }
}

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();
    for line in text.lines() {
        parse_line(line);
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