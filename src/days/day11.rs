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
    current: &'a str,
    target: &'a str,
    connections: &HashMap<&'a str, HashSet<&'a str>>, 
    memo: &mut HashMap<&'a str, u128>, 
) -> u128 {
    if current == target {
        // We've found a path!
        1
    } 
    
    match memo.get(&current) {
        Some(num_paths) => { num_paths },
        None => {
            // Recurse by trying all the downstream connections
            let mut num_paths = 0;
            if let Some(downstreams) = connections.get(&current) {
                for downstream in downstreams {
                    num_paths += count_paths(downstream, target, connections, memo);
                }
            }

            memo.insert(current_state, num_paths);
            num_paths
        }
    }
}

pub fn part1(data_path: &Path) -> u128 {
    let data = std::fs::read_to_string(data_path).unwrap();
    let connections = parse_connections(&data);
    
    let mut memo: HashMap<&str, u128> = HashMap::new();
    let score = count_paths("you", "out", &connections, &mut memo);
    score
}

pub fn part2(data_path: &Path) -> u128 {
    let data = std::fs::read_to_string(data_path).unwrap();
    let connections = parse_connections(&data);

    // svr --> dac
    let mut memo: HashMap<&str, u128> = HashMap::new();
    let num_svr_to_dac = count_paths("svr", "dac", &connections, &mut memo);

    // dac --> fft
    let mut memo: HashMap<&str, u128> = HashMap::new();
    let num_dac_to_fft = count_paths("dac", "fft", &connections, &mut memo);

    // fft --> out
    let mut memo: HashMap<&str, u128> = HashMap::new();
    let num_fft_to_out = count_paths("fft", "out", &connections, &mut memo);

    score += num_svr_to_dac * num_dac_to_fft * num_fft_to_out;

    // svr --> fft
    let mut memo: HashMap<&str, u128> = HashMap::new();
    let num_svr_to_fft = count_paths("svr", "fft", &connections, &mut memo);

    // fft --> dac
    let mut memo: HashMap<&str, u128> = HashMap::new();
    let num_fft_to_dac = count_paths("fft", "dac", &connections, &mut memo);

    // dac --> out
    let mut memo: HashMap<&str, u128> = HashMap::new();
    let num_dac_to_out = count_paths("dac", "out", &connections, &mut memo);

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
}
