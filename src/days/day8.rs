use std::path::Path;
use std::collections::HashSet;

fn n_closest_pairs_of_boxes(boxes: &Vec<(u64, u64, u64)>, n: usize) -> Vec<(u64, usize, usize)> {
    let mut closest_pairs = vec![];
    for i1 in 0..boxes.len()-1 {
        let (x1,y1,z1) = boxes[i1];
        for (i2, b2) in boxes.iter().enumerate().skip(i1+1) {
            let (x2,y2,z2) = b2;
            let x_diff = x1.abs_diff(*x2);
            let y_diff = y1.abs_diff(*y2); 
            let z_diff = z1.abs_diff(*z2);
            let d = x_diff.pow(2) + y_diff.pow(2) + z_diff.pow(2);
            closest_pairs.push((d, i1, i2));
        }
    }

    closest_pairs.sort_unstable();

    let mut pairs = vec![];

    if n == 0 {
        for (d,b1,b2) in closest_pairs {
            pairs.push((d, b1, b2));
        }
    } else {
        for (d,b1,b2) in closest_pairs {
            if pairs.len() < n {
                pairs.push((d, b1, b2));
            }
        }
    }
    pairs
}

pub fn part1(data_path: &Path) -> u64 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let mut boxes: Vec<(u64, u64, u64)> = vec![];
    for line in text.lines() {
        let mut s = line.trim().split(',');
        let x = s.next().unwrap().parse().unwrap();
        let y = s.next().unwrap().parse().unwrap();
        let z = s.next().unwrap().parse().unwrap();
        boxes.push((x,y,z));
    }
    let n = if boxes.len() <= 20 { 10 } else { 1000 };
    let pairs = n_closest_pairs_of_boxes(&boxes, n);
    let mut circuits: Vec<HashSet<usize>> = (0..boxes.len()).map(|i| HashSet::from([i])).collect();

    for (_,i1,i2) in &pairs {

        let c1_idx = circuits.iter().position(|c| c.contains(i1)).unwrap();


        if circuits[c1_idx].contains(i2) { continue } // already in the same circuit, nothing to be done

        let c1 = circuits.remove(c1_idx);
        let c2_idx = circuits.iter().position(|c| c.contains(i2)).unwrap();
        let c2 = circuits.remove(c2_idx);

        let mut combined_boxes = HashSet::new(); // NOTE this is ugly, but I'm having trouble with .union()
        for b in c1 { combined_boxes.insert(b); }
        for b in c2 { combined_boxes.insert(b); }
        circuits.push(combined_boxes);
    }

    let mut circuit_sizes: Vec<usize> = circuits.iter().map(std::collections::HashSet::len).collect();
    circuit_sizes.sort_unstable();

    let mut score = 1;
    score *= circuit_sizes.pop().unwrap();
    score *= circuit_sizes.pop().unwrap();
    score *= circuit_sizes.pop().unwrap();

    score.try_into().unwrap()
}

pub fn part2(data_path: &Path) -> u64 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let mut boxes: Vec<(u64, u64, u64)> = vec![];
    for line in text.lines() {
        let mut s = line.trim().split(',');
        let x = s.next().unwrap().parse().unwrap();
        let y = s.next().unwrap().parse().unwrap();
        let z = s.next().unwrap().parse().unwrap();
        boxes.push((x,y,z));
    }
    let pairs = n_closest_pairs_of_boxes(&boxes, 0);
    let mut circuits: Vec<HashSet<usize>> = (0..boxes.len()).map(|i| HashSet::from([i])).collect();

    for (_,i1,i2) in &pairs {

        let c1_idx = circuits.iter().position(|c| c.contains(i1)).unwrap();


        if circuits[c1_idx].contains(i2) { continue } // already in the same circuit, nothing to be done

        let c1 = circuits.remove(c1_idx);
        let c2_idx = circuits.iter().position(|c| c.contains(i2)).unwrap();
        let c2 = circuits.remove(c2_idx);

        let mut combined_boxes = HashSet::new(); // NOTE this is ugly, but I'm having trouble with .union()
        for b in c1 { combined_boxes.insert(b); }
        for b in c2 { combined_boxes.insert(b); }
        circuits.push(combined_boxes);

        if circuits.len() == 1 {
            return (boxes[*i1].0 * boxes[*i2].0).try_into().unwrap();
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
162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";
        let temp_dir = tempdir().unwrap();
        let f_path = temp_dir.path().join("test_input.txt");
        let mut temp_file = File::create(f_path.clone()).unwrap();
        write!(temp_file, "{test_input}").unwrap();

        // have to return dir and file so they don't go out of scope
        (temp_dir, temp_file, f_path)
    }

    #[test]
    fn test_part1() {
        let (_d, _f, test_path) = create_test_file();
        let result = part1(&test_path);
        assert_eq!(result, 40);
    }

    #[test]
    fn test_part2() {
        let (_d, _f, test_path) = create_test_file();
        let result = part2(&test_path);
        assert_eq!(result, 25272);
    }
}