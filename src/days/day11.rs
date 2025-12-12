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

fn count_paths<'a>(
    current_state: (&'a str, Vec<bool>),
    must_visits: &Vec<&'a str>,
    connections: &HashMap<&'a str, HashSet<&'a str>>, 
    memo: &mut HashMap<(&'a str, Vec<bool>), u32>, 
    prior_states: HashSet<(&'a str, Vec<bool>)>,
) -> u32 {
    if current_state.0 == "out" && current_state.1.iter().all(|&v| v) {
        // We've found "out" AND we've hit all must visits so we've found a path
        1
    } else if !connections.contains_key(&current_state.0) {
        // We've hit a dead end, no downstreams to try
        0
    } else if memo.contains_key(&current_state) {
        // We already know how many paths stem from this state, no need to recurse
        *memo.get(&current_state).unwrap()
    } else {
        // Add the current state to the list of prior visited (ugly implementation)
        let mut updated_visit_info = current_state.1.clone();
        for (i, must_visit) in must_visits.iter().enumerate() {
            if current_state.0 == *must_visit {
                updated_visit_info[i] = true;
            }
        }

        let mut updated_current_state = current_state.clone();
        updated_current_state.1 = updated_visit_info.clone();

        let mut updated_prior_states = prior_states.clone();
        updated_prior_states.insert(updated_current_state.clone());

        // Recurse by trying all the downstream connections
        let mut num_paths = 0;
        if let Some(downstreams) = connections.get(&current_state.0) {
            for downstream in downstreams {
                let mut next_state = current_state.clone();
                next_state.0 = downstream;
                next_state.1 = updated_visit_info.clone();

                // Don't revisit prior states we've tried to avoid infinit loops
                if !updated_prior_states.contains(&next_state) {
                    num_paths += count_paths(next_state, must_visits, connections, memo, updated_prior_states.clone());
                }
            }
        }
        memo.insert(updated_current_state, num_paths);
        num_paths
    }
}

pub fn part1(data_path: &Path) -> u32 {
    let data = std::fs::read_to_string(data_path).unwrap();
    let connections = parse_connections(&data);

    let mut memo: HashMap<(&str, Vec<bool>), u32> = HashMap::new();
    let priors: HashSet<(&str, Vec<bool>)> = HashSet::new();

    let current_state: (&str, Vec<bool>) = ("you", vec![true]);
    let must_visits: Vec<&str> = vec![];

    count_paths(current_state, &must_visits, &connections, &mut memo, priors)
}

pub fn part2(data_path: &Path) -> u32 {
    let data = std::fs::read_to_string(data_path).unwrap();
    let connections = parse_connections(&data);

    let mut memo: HashMap<(&str, Vec<bool>), u32> = HashMap::new();
    let priors: HashSet<(&str, Vec<bool>)> = HashSet::new();

    let must_visits: HashMap<&str, bool> = HashMap::new();

    let current_state: (&str, Vec<bool>) = ("svr", vec![false, false]);
    let must_visits: Vec<&str> = vec!["dac", "fft"];

    count_paths(current_state, &must_visits, &connections, &mut memo, priors)
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

    fn create_test_file2() -> (tempfile::TempDir, File, PathBuf) {
        let test_input = "\
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";
        let temp_dir = tempdir().unwrap();
        let f_path = temp_dir.path().join("test_input.txt");
        let mut temp_file = File::create(f_path.clone()).unwrap();
        write!(temp_file, "{}", test_input).unwrap();

        // have to return dir and file so they don't go out of scope
        (temp_dir, temp_file, f_path)
    }

    #[test]
    fn test_part2() {
        let (_d, _f, test_path) = create_test_file2();
        let result = part2(&test_path);
        assert_eq!(result, 2);
    }

    fn create_test_file2a() -> (tempfile::TempDir, File, PathBuf) {
        /* 
        This was supposed to be the tricky situation that I was failing where
        we pass by "out" once to collect "dac" and "fft" but then hit out again:
            svr -> out ->  fft -> dac -> out

        And made the code more complicated to handle this but it turns out the problem
        input was nice and we never have "out" as an upstream such as "out: aaa bbb"
        so I don't actually have to worry about this situation

        Also similarly "svr" is never downstream such as "aaa: svr" so also don't have
        to worry about whether or not we hit "svr" multiple times
        */
        let test_input = "\
svr: out
out: fft
fft: dac
dac: out";
        let temp_dir = tempdir().unwrap();
        let f_path = temp_dir.path().join("test_input.txt");
        let mut temp_file = File::create(f_path.clone()).unwrap();
        write!(temp_file, "{}", test_input).unwrap();

        // have to return dir and file so they don't go out of scope
        (temp_dir, temp_file, f_path)
    }

    #[test]
    fn test_part2a() {
        let (_d, _f, test_path) = create_test_file2a();
        let result = part2(&test_path);
        assert_eq!(result, 1);
    }

    fn create_test_file2b() -> (tempfile::TempDir, File, PathBuf) {
        /* 
        */
        let test_input = "\
svr: aaa bbb ccc
aaa: fft
bbb: fft
ccc: fft
fft: dac
dac: out";
        let temp_dir = tempdir().unwrap();
        let f_path = temp_dir.path().join("test_input.txt");
        let mut temp_file = File::create(f_path.clone()).unwrap();
        write!(temp_file, "{}", test_input).unwrap();

        // have to return dir and file so they don't go out of scope
        (temp_dir, temp_file, f_path)
    }

    #[test]
    fn test_part2b() {
        let (_d, _f, test_path) = create_test_file2b();
        let result = part2(&test_path);
        assert_eq!(result, 3);
    }

    fn create_test_file2c() -> (tempfile::TempDir, File, PathBuf) {
        /* 
        */
        let test_input = "\
svr: aaa bbb ccc
aaa: fft
bbb: fft
ccc: fft
aaa: bbb
fft: dac
dac: out";
        let temp_dir = tempdir().unwrap();
        let f_path = temp_dir.path().join("test_input.txt");
        let mut temp_file = File::create(f_path.clone()).unwrap();
        write!(temp_file, "{}", test_input).unwrap();

        // have to return dir and file so they don't go out of scope
        (temp_dir, temp_file, f_path)
    }

    #[test]
    fn test_part2c() {
        let (_d, _f, test_path) = create_test_file2c();
        let result = part2(&test_path);
        assert_eq!(result, 4);
    }
}
