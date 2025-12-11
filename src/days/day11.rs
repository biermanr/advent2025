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
    avoids: &HashSet<&'a str>,
    connections: &HashMap<&'a str, HashSet<&'a str>>, 
    memo: &mut HashMap<&'a str, u32>, 
    prior_visits: HashSet<&'a str>,
) -> u32 {
    if current == target {
        // We've found a path!
        1
    } else if avoids.contains(&current) {
        // Stop early on paths that hit a device we want to avoid
        0
    } else if prior_visits.contains(&current) {
        // This path has hit the same node again so there's a loop no solution(?)
        0
    } else if !connections.contains_key(&current) {
        // We've hit a dead end, no downstreams to try
        0
    } else {
        // Add the current node to the list of prior visited (ugly implementation)
        let mut updated_prior_visits: HashSet<&str> = HashSet::from([current]);
        for prior_device in prior_visits {
            updated_prior_visits.insert(prior_device);
        }

        match memo.get(&current) {
            Some(num_paths) => {
                // Haven't been to this device on this walk, but have been here on a prior walk, we know how many downstream paths there are
                *num_paths
            },
            None => {
                // Recur by trying all the downstream connections
                let mut num_paths = 0;
                if let Some(downstreams) = connections.get(&current) {
                    for downstream in downstreams {
                        num_paths += count_paths(downstream, target, avoids, connections, memo, updated_prior_visits.clone()); //wasteful to clone here
                    }
                }
                memo.insert(current, num_paths);
                num_paths
            }
        }
    }
}

pub fn part1(data_path: &Path) -> u32 {
    let data = std::fs::read_to_string(data_path).unwrap();
    let connections = parse_connections(&data);
    let must_visits:HashSet<&str> = HashSet::new();
    let mut memo: HashMap<&str, u32> = HashMap::new();
    let priors: HashSet<&str> = HashSet::new();
    let avoids: HashSet<&str> = HashSet::new();
    let score = count_paths("you", "out", &avoids, &connections, &mut memo, priors);
    score
}

pub fn part2(data_path: &Path) -> u32 {
    let data = std::fs::read_to_string(data_path).unwrap();
    let connections = parse_connections(&data);
    let mut score = 0;

    // svr --> dac
    let mut memo: HashMap<&str, u32> = HashMap::new();
    let priors: HashSet<&str> = HashSet::new();
    let avoids: HashSet<&str> = HashSet::from(["fft", "out"]);
    let num_svr_to_dac = count_paths("svr", "dac", &avoids, &connections, &mut memo, priors);

    // dac --> fft
    let mut memo: HashMap<&str, u32> = HashMap::new();
    let priors: HashSet<&str> = HashSet::new();
    let avoids: HashSet<&str> = HashSet::from(["svr", "out"]);
    let num_dac_to_fft = count_paths("dac", "fft", &avoids, &connections, &mut memo, priors);

    // fft --> out
    let mut memo: HashMap<&str, u32> = HashMap::new();
    let priors: HashSet<&str> = HashSet::new();
    let avoids: HashSet<&str> = HashSet::from(["svr", "dac"]);
    let num_fft_to_out = count_paths("fft", "out", &avoids, &connections, &mut memo, priors);

    score += num_svr_to_dac * num_dac_to_fft * num_fft_to_out;

    // svr --> fft
    let mut memo: HashMap<&str, u32> = HashMap::new();
    let priors: HashSet<&str> = HashSet::new();
    let avoids: HashSet<&str> = HashSet::from(["dac", "out"]);
    let num_svr_to_fft = count_paths("svr", "fft", &avoids, &connections, &mut memo, priors);

    // fft --> dac
    let mut memo: HashMap<&str, u32> = HashMap::new();
    let priors: HashSet<&str> = HashSet::new();
    let avoids: HashSet<&str> = HashSet::from(["svr", "out"]);
    let num_fft_to_dac = count_paths("fft", "dac", &avoids, &connections, &mut memo, priors);

    // dac --> out
    let mut memo: HashMap<&str, u32> = HashMap::new();
    let priors: HashSet<&str> = HashSet::new();
    let avoids: HashSet<&str> = HashSet::from(["svr", "fft"]);
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
}
