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
    connections: &HashMap<&'a str, HashSet<&'a str>>, 
    memo: &mut HashMap<&'a str, u32>, 
    prior_visits: HashSet<&'a str>,
) -> u32 {
    if current == "out" {
        // We've found a path!
        1
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

        // Recur by trying all the downstream connections
        let mut num_paths = 0;
        if let Some(downstreams) = connections.get(&current) {
            for downstream in downstreams {
                num_paths += count_paths(downstream, connections, memo, updated_prior_visits.clone()); //wasteful to clone here
            }
        }
        memo.insert(current, num_paths);
        num_paths
    }
}

pub fn part1(data_path: &Path) -> u32 {
    let data = std::fs::read_to_string(data_path).unwrap();
    let connections = parse_connections(&data);
    let mut memo: HashMap<&str, u32> = HashMap::new();
    let priors: HashSet<&str> = HashSet::new();
    let score = count_paths("you", &connections, &mut memo, priors);
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
}
