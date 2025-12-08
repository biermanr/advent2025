use std::path::Path;
use std::collections::HashSet;
use std::collections::BinaryHeap;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
struct Box {
    x: u32,
    y: u32,
    z: u32,
}

impl Box {
    fn new(comma_s: &str) -> Self {
        let mut split_s = comma_s.split(',');
        Box {
            x: split_s.next().expect("Not enough nums in comma string for X").parse().expect("Unable to parse to num for X"),
            y: split_s.next().expect("Not enough nums in comma string for Y").parse().expect("Unable to parse to num for Y"),
            z: split_s.next().expect("Not enough nums in comma string for Z").parse().expect("Unable to parse to num for Z"),
        }
    }

    fn dist(&self, other: &Box) -> u32 {
        self.x.abs_diff(other.x)*self.x.abs_diff(other.x) +
        self.y.abs_diff(other.y)*self.y.abs_diff(other.y) +
        self.z.abs_diff(other.z)*self.z.abs_diff(other.z)
    }
}

#[derive(Debug, PartialEq)]
struct Circuit<'a> {
    boxes: HashSet<&'a Box>,
}

impl<'a> Circuit<'a> {
    fn new(b: &'a Box) -> Self {
        Circuit { boxes: HashSet::from([b]) }
    }
}

fn n_closest_pairs_of_boxes(boxes: &Vec<Box>, n: usize) -> Vec<(&Box, &Box)> {
    let mut closest_pairs = BinaryHeap::new();
    for (i1,b1) in boxes.iter().enumerate() {
        for (i2,b2) in boxes.iter().enumerate() {
            if i1 == i2 { continue }
            let d = b1.dist(b2);
            if closest_pairs.len() < n {
                closest_pairs.push((d, b1, b2));
            } else if let Some((largest_d,_,_)) = closest_pairs.peek() {
                if d < *largest_d {
                    closest_pairs.pop();
                    closest_pairs.push((b1.dist(b2), b1, b2));
                }
            }
        }
    }

    let mut pairs = vec![];
    while let Some((_,b1,b2)) = closest_pairs.pop() {
        pairs.push((b1, b2));
    }
    pairs
}

pub fn part1(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let boxes: Vec<Box> = text.lines().map(|l| Box::new(l)).collect();
    let mut circuits: Vec<Circuit> = boxes.iter().map(Circuit::new).collect();
    let pairs = n_closest_pairs_of_boxes(&boxes, 10);

    for (b1,b2) in pairs.iter().rev() {

        // Careful, this has an issue if both boxes are already in the same circuit
        let c1_idx = circuits.iter().position(|c| c.boxes.contains(b1)).unwrap();
        let c1 = circuits.remove(c1_idx);

        let c2_idx = circuits.iter().position(|c| c.boxes.contains(b2)).unwrap();
        let c2 = circuits.remove(c2_idx);

        if c1 != c2 {
            println!("{:?} is in Circuit {:?}", b1, c1);
            println!("{:?} is in Circuit {:?}", b2, c2);
            let c = Circuit { boxes: c1.boxes.union(c2.boxes).collect() };
            println!("Combined circuit is {:?}", c);
        }
        break;
    }
    0
}

pub fn part2(data_path: &Path) -> u32 {
    let text = std::fs::read_to_string(data_path).unwrap();
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
        write!(temp_file, "{}", test_input).unwrap();

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
        assert_eq!(result, 1);
    }
}