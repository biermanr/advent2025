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

fn fill_grid(grid: &mut Vec<Vec<u8>>) {
    // Grid value key
    // - 0 means unsure
    // - 1 means wall
    // - 2 means inside
    // - 3 means outside
    for y in 0..grid.len() {
        for x in 0..grid[0].len() {
            if grid[y][x] != 0 { continue }

            if x > 0 && grid[y][x-1] != 1 { 
                // Check if the left neighbor is any help
                grid[y][x] = grid[y][x-1];
            } else if y > 0 && grid[y-1][x] != 1 { 
                // Check if the up neighbor is any help
                grid[y][x] = grid[y-1][x]; 
            } else {
                // Need to determine state ourselves
                // Walk from x=0 to current x and count how many walls we hit. If even then we're outside 
                let mut num_walls_hit_left = 0;
                let mut in_wall = false;
                for x_idx in 0..x {
                    if grid[y][x_idx] == 1 {
                        if in_wall { continue }
                        else { num_walls_hit_left += 1 }
                        in_wall = true;
                    } else {
                        in_wall = false;
                    }
                }

                // Walk from current x to max x and count how many walls we hit. If even then we're outside 
                let mut num_walls_hit_right = 0;
                let mut in_wall = false;
                for x_idx in x+1..grid[0].len() {
                    if grid[y][x_idx] == 1 {
                        if in_wall { continue }
                        else { num_walls_hit_right += 1 }
                        in_wall = true;
                    } else {
                        in_wall = false;
                    }
                }

                if num_walls_hit_left % 2 == 1 && num_walls_hit_right % 2 == 1 { 
                    grid[y][x] = 2;
                } else {
                    grid[y][x] = 3;
                }
            }
        }
    }
}

fn get_bad_regions(grid: &Vec<Vec<u8>>) -> Vec<(usize, usize, usize, usize)>{
    let mut bads: Vec<(usize, usize, usize, usize)> = vec![];

    for y in 0..grid.len() {
        let mut sx = 0;
        let mut opening = false;
        for x in 0..grid[0].len() {
            if grid[y][x] == 3 && opening { 
                continue;
            } else if grid[y][x] == 3 && !opening {
                sx = x;
                opening = true;
            } else if grid[y][x] != 3 && opening {
                bads.push((sx, y, x-1, y));
                opening = false;
            } else if grid[y][x] != 3 && !opening {
                continue;
            }
        }
        if opening {
            bads.push((sx, y, grid[0].len()-1, y));
        }
    } 
    bads
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
    let mut grid: Vec<Vec<u8>> = vec![vec![0; *max_x+1]; *max_y+1];

    let (mut last_x, mut last_y) = red_tiles[0];
    for i in 1..red_tiles.len() {
        let (curr_x,curr_y) = red_tiles[i];

        if curr_x == last_x {
            let (min_y, max_y) = (cmp::min(last_y, curr_y), cmp::max(last_y, curr_y));
            for y in min_y..=max_y {
                grid[y][curr_x] = 1;
            }
        } else if curr_y == last_y {
            let (min_x, max_x) = (cmp::min(last_x, curr_x), cmp::max(last_x, curr_x));
            for x in min_x..=max_x {
                grid[curr_y][x] = 1;
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
        for y in min_y..=max_y {
            grid[y][curr_x] = 1;
        }
    } else if curr_y == last_y {
        let (min_x, max_x) = (cmp::min(last_x, curr_x), cmp::max(last_x, curr_x));
        for x in min_x..=max_x {
            grid[curr_y][x] = 1;
        }
    } else {
        println!("ERROR: Consecutive points should have been on the same line");
    }

    fill_grid(&mut grid);

    let bad_regions = get_bad_regions(&grid);

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

            for (bad_min_x, bad_min_y, bad_max_x, _) in &bad_regions {
                let bad_y_contained = *bad_min_y >= min_y && *bad_min_y <= max_y;
                let bad_x_contained = (*bad_min_x >= min_x && *bad_min_x <= max_x) || (*bad_max_x >= min_x && *bad_max_x <= max_x);
                if bad_y_contained && bad_x_contained {
                    valid = false;
                    break;
                }
            }

            for x in min_x..max_x {
                for y in min_y..max_y {
                    if grid[y][x] == 3 {
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
        write!(temp_file, "{test_input}").unwrap();

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