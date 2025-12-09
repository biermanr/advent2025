use std::path::Path;
use std::cmp;

pub fn part1(data_path: &Path) -> u64 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let mut red_tiles: Vec<(usize, usize)> = vec![];
    for line in text.lines() {
        let mut s = line.trim().split(',');
        let x = s.next().unwrap().parse().unwrap();
        let y = s.next().unwrap().parse().unwrap();
        red_tiles.push((x,y));
    }

    let mut max_area = 0;
    for i in 0..red_tiles.len()-1 {
        let (x1,y1) = red_tiles[i];
        for j in i+1..red_tiles.len() {
            let (x2,y2) = red_tiles[j];
            let area = (x1.abs_diff(x2)+1)*(y1.abs_diff(y2)+1);
            if area > max_area {
                max_area = area;
            }

        }
    }
    max_area.try_into().unwrap()
}

fn ok_tile(grid: &Vec<Vec<bool>>, x: usize, y: usize) -> bool {
    if grid[y][x] {
        return true;
    }

    // Walk from x=0 to current x and count how many walls we hit. If even then we're outside 
    let mut num_walls_hit = 0;
    let mut in_wall = false;
    for x_idx in 0..x {
        if grid[y][x_idx] {
            if in_wall { continue }
            else { num_walls_hit += 1 }
            in_wall = true;
        } else {
            in_wall = false;
        }
    }
    if num_walls_hit % 2 == 0 { return false }

    // Walk from current x to max x and count how many walls we hit. If even then we're outside 
    let mut num_walls_hit = 0;
    let mut in_wall = false;
    for x_idx in x+1..grid[0].len() {
        if grid[y][x_idx] {
            if in_wall { continue }
            else { num_walls_hit += 1 }
            in_wall = true;
        } else {
            in_wall = false;
        }
    }
    if num_walls_hit % 2 == 0 { return false }

    return true;
}

fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid {
        let s: String = row.iter().map(|v| if *v {"#"} else {"."}).collect();
        println!("{:?}",s);
    }
}

pub fn part2(data_path: &Path) -> u64 {
    let text = std::fs::read_to_string(data_path).unwrap();
    let mut red_tiles: Vec<(usize, usize)> = vec![];
    for line in text.lines() {
        let mut s = line.trim().split(',');
        let x = s.next().unwrap().parse().unwrap();
        let y = s.next().unwrap().parse().unwrap();
        red_tiles.push((x,y));
    }

    let max_x = red_tiles.iter().map(|(x,_)| x).max().unwrap();
    let max_y = red_tiles.iter().map(|(_,y)| y).max().unwrap();
    let mut grid: Vec<Vec<bool>> = vec![vec![false; *max_x+1]; *max_y+1];

    let (mut last_x, mut last_y) = red_tiles[0];
    for i in 1..red_tiles.len() {
        let (curr_x,curr_y) = red_tiles[i];

        if curr_x == last_x {
            let (min_y, max_y) = (cmp::min(last_y, curr_y), cmp::max(last_y, curr_y));
            for y in min_y..max_y+1 {
                grid[y][curr_x] = true;
            }
        } else if curr_y == last_y {
            let (min_x, max_x) = (cmp::min(last_x, curr_x), cmp::max(last_x, curr_x));
            for x in min_x..max_x+1 {
                grid[curr_y][x] = true;
            }
        } else {
            println!("ERROR: Consecutive points should have been on the same line");
        }
        last_x = curr_x;
        last_y = curr_y;
    }

    // Repeat for the wrap-around of the last to first item
    let (last_x, last_y) = red_tiles[red_tiles.len()-1];
    let (curr_x, curr_y) = red_tiles[0];
    if curr_x == last_x {
        let (min_y, max_y) = (cmp::min(last_y, curr_y), cmp::max(last_y, curr_y));
        for y in min_y..max_y+1 {
            grid[y][curr_x] = true;
        }
    } else if curr_y == last_y {
        let (min_x, max_x) = (cmp::min(last_x, curr_x), cmp::max(last_x, curr_x));
        for x in min_x..max_x+1 {
            grid[curr_y][x] = true;
        }
    } else {
        println!("ERROR: Consecutive points should have been on the same line");
    }

    let mut max_area = 0;
    for i in 0..red_tiles.len()-1 {
        let (x1,y1) = red_tiles[i];
        for j in i+1..red_tiles.len() {
            let (x2,y2) = red_tiles[j];

            let area = (x1.abs_diff(x2)+1)*(y1.abs_diff(y2)+1);
            if area <= max_area { continue }

            let (min_x, max_x) = (cmp::min(x1, x2), cmp::max(x1, x2));
            let (min_y, max_y) = (cmp::min(y1, y2), cmp::max(y1, y2));
            let mut valid = true;
            for x in min_x..max_x {
                for y in min_y..max_y {
                    if !ok_tile(&grid, x, y){
                        valid = false;
                        break;
                    }
                }
                if !valid { break };
            }

            if valid {
                max_area = area;
            }
        }
    }
    max_area.try_into().unwrap()
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
7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";
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
        assert_eq!(result, 50);
    }

    #[test]
    fn test_part2() {
        let (_d, _f, test_path) = create_test_file();
        let result = part2(&test_path);
        assert_eq!(result, 24);
    }
}