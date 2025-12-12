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
    memo: &mut HashMap<&'a str, u128>, 
    prior_visits: HashSet<&'a str>,
) -> u128 {
    if current == target {
        // We've found a path!
        1
    } else if !connections.contains_key(&current_state.0) {
        // We've hit a dead end, no downstreams to try
        0
    } else if memo.contains_key(&current_state) {
        // We already know how many paths stem from this state, no need to recurse
        *memo.get(&current_state).unwrap()
    } else {

        let mut updated_prior_states = prior_states.clone();
        updated_prior_states.insert(current_state.clone());

        // Recurse by trying all the downstream connections
        let mut num_paths = 0;
        if let Some(downstreams) = connections.get(&current_state.0) {
            for downstream in downstreams {
                let mut next_state = current_state.clone();
                next_state.0 = downstream;
                next_state.1 = updated_visit_info.clone();

                // If there's a memo value then just use it
                if let Some(remembered_steps) = memo.get(&next_state) {
                    num_paths += remembered_steps;
                    continue;
                }

                // Don't revisit prior states we've tried. avoid infinite loops
                println!("Deciding whether or not to recurse through {:?} given priors {:?}", next_state, updated_prior_states);
                if !updated_prior_states.contains(&next_state) {
                    num_paths += count_paths(next_state, must_visits, connections, memo, updated_prior_states.clone());
                }
            }
        }

        if num_paths > 0 {
            memo.insert(current_state, num_paths);
        }
        num_paths
    }
}

pub fn part1(data_path: &Path) -> u128 {
    let data = std::fs::read_to_string(data_path).unwrap();
    let connections = parse_connections(&data);
    let must_visits:HashSet<&str> = HashSet::new();
    let mut memo: HashMap<&str, u128> = HashMap::new();
    let priors: HashSet<&str> = HashSet::new();
    let avoids: HashSet<&str> = HashSet::new();
    let score = count_paths("you", "out", &avoids, &connections, &mut memo, priors);
    score
}

pub fn part2(data_path: &Path) -> u128 {
    let data = std::fs::read_to_string(data_path).unwrap();
    let connections = parse_connections(&data);

    // svr --> dac
    let mut memo: HashMap<&str, u128> = HashMap::new();
    let priors: HashSet<&str> = HashSet::new();
    //let avoids: HashSet<&str> = HashSet::from(["fft", "out"]);
    let avoids: HashSet<&str> = HashSet::new();
    let num_svr_to_dac = count_paths("svr", "dac", &avoids, &connections, &mut memo, priors);

    // dac --> fft
    let mut memo: HashMap<&str, u128> = HashMap::new();
    let priors: HashSet<&str> = HashSet::new();
    //let avoids: HashSet<&str> = HashSet::from(["svr", "out"]);
    let avoids: HashSet<&str> = HashSet::new();
    let num_dac_to_fft = count_paths("dac", "fft", &avoids, &connections, &mut memo, priors);

    // fft --> out
    let mut memo: HashMap<&str, u128> = HashMap::new();
    let priors: HashSet<&str> = HashSet::new();
    //let avoids: HashSet<&str> = HashSet::from(["svr", "dac"]);
    let avoids: HashSet<&str> = HashSet::new();
    let num_fft_to_out = count_paths("fft", "out", &avoids, &connections, &mut memo, priors);

    score += num_svr_to_dac * num_dac_to_fft * num_fft_to_out;

    // svr --> fft
    let mut memo: HashMap<&str, u128> = HashMap::new();
    let priors: HashSet<&str> = HashSet::new();
    //let avoids: HashSet<&str> = HashSet::from(["dac", "out"]);
    let avoids: HashSet<&str> = HashSet::new();
    let num_svr_to_fft = count_paths("svr", "fft", &avoids, &connections, &mut memo, priors);

    // fft --> dac
    let mut memo: HashMap<&str, u128> = HashMap::new();
    let priors: HashSet<&str> = HashSet::new();
    //let avoids: HashSet<&str> = HashSet::from(["svr", "out"]);
    let avoids: HashSet<&str> = HashSet::new();
    let num_fft_to_dac = count_paths("fft", "dac", &avoids, &connections, &mut memo, priors);

    // dac --> out
    let mut memo: HashMap<&str, u128> = HashMap::new();
    let priors: HashSet<&str> = HashSet::new();
    //let avoids: HashSet<&str> = HashSet::from(["svr", "fft"]);
    let avoids: HashSet<&str> = HashSet::new();
    let num_dac_to_out = count_paths("dac", "out", &avoids, &connections, &mut memo, priors);

    score += num_svr_to_fft * num_fft_to_dac * num_dac_to_out;

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
        Should be 5 paths

            -> aaa ----|
            |   ^      v
        svr -> bbb -> fft -> dac -> out
            |   v      ^
            -> ccc ----|
        */
        let test_input = "\
svr: aaa bbb ccc
aaa: fft
bbb: fft aaa ccc
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
    fn test_part2c() {
        let (_d, _f, test_path) = create_test_file2c();
        let result = part2(&test_path);
        assert_eq!(result, 5);
    }

    fn create_test_file2d() -> (tempfile::TempDir, File, PathBuf) {
        /* 
        Should be 6 paths:
        * svr -> aaa -> fft -> dac -> out
        * svr -> bbb -> fft -> dac -> out
        * svr -> ccc -> fft -> dac -> out

        * svr -> bbb -> aaa -> fft -> dac -> out
        * svr -> ccc -> bbb -> fft -> dac -> out
        * svr -> ccc -> bbb -> aaa -> fft -> dac -> out

            -> aaa ----|
            |   ^      v
        svr -> bbb -> fft -> dac -> out
            |   ^      ^      
            -> ccc ----|      

        */
        let test_input = "\
svr: aaa bbb ccc
aaa: fft
bbb: fft aaa
ccc: bbb fft
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
    fn test_part2d() {
        let (_d, _f, test_path) = create_test_file2d();
        let result = part2(&test_path);
        assert_eq!(result, 6);
    }

    fn create_test_file2e() -> (tempfile::TempDir, File, PathBuf) {
        /* 
        Should be 12 paths
        * svr -> aaa -> fft -> dac -> out
        * svr -> bbb -> fft -> dac -> out
        * svr -> ccc -> fft -> dac -> out

        * svr -> bbb -> aaa -> fft -> dac -> out
        * svr -> ccc -> bbb -> fft -> dac -> out
        * svr -> ccc -> bbb -> aaa -> fft -> dac -> out

        * svr -> aaa -> fft -> dac -> aaa -> fft -> dac -> out
        * svr -> bbb -> fft -> dac -> aaa -> fft -> dac -> out
        * svr -> ccc -> fft -> dac -> aaa -> fft -> dac -> out

        * svr -> bbb -> aaa -> fft -> dac -> aaa -> fft -> dac -> out
        * svr -> ccc -> bbb -> fft -> dac -> aaa -> fft -> dac -> out
        * svr -> ccc -> bbb -> aaa -> fft -> dac -> aaa -> fft -> dac -> out

                |--------------
                v             |
            -> aaa ----|      |
            |   ^      v      |
        svr -> bbb -> fft -> dac -> out
            |   ^      ^       
            -> ccc ----|       

        */
        let test_input = "\
svr: aaa bbb ccc
aaa: fft
bbb: fft aaa
ccc: bbb fft
fft: dac
dac: aaa out";
        let temp_dir = tempdir().unwrap();
        let f_path = temp_dir.path().join("test_input.txt");
        let mut temp_file = File::create(f_path.clone()).unwrap();
        write!(temp_file, "{}", test_input).unwrap();

        // have to return dir and file so they don't go out of scope
        (temp_dir, temp_file, f_path)
    }

    #[test]
    fn test_part2e() {
        let (_d, _f, test_path) = create_test_file2e();
        let result = part2(&test_path);
        assert_eq!(result, 12); // Seems like the current issue is filling the memo before counting all paths?
    }

    fn create_test_file2f() -> (tempfile::TempDir, File, PathBuf) {
        /* 
        Should be 2 paths
        * svr -> aaa -> bbb -> fft -> dac -> out
        * svr -> aaa -> bbb -> aaa -> bbb -> fft -> dac -> out

                |------|       
                v      |       
        svr -> aaa -> bbb -> fft -> dac -> out

        */
        let test_input = "\
svr: aaa
aaa: bbb
bbb: fft aaa
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
    fn test_part2f() {
        let (_d, _f, test_path) = create_test_file2f();
        let result = part2(&test_path);
        assert_eq!(result, 2); 
        // Problem is when I go aaa -> bbb -> aaa I stop because
        // I've already been in this same state before
    }
}
