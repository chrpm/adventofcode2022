use std::{collections::HashSet, fs};

fn main() {
    let file_path = "input.txt";
    let mut height_map = load_input(file_path);

    let final_value = 99;
    let starting_value = 98;
    let mut starting_postition = (0, 0);
    let mut alt_starts = Vec::new();
    let mut final_postition = (0, 0);
    for i in 0..height_map.len() {
        for j in 0..height_map[0].len() {
            if height_map[i][j] == starting_value {
                starting_postition = (i as i32, j as i32);
                height_map[i][j] = 1;
            }
            if height_map[i][j] == final_value {
                final_postition = (i as i32, j as i32);
                height_map[i][j] = 26;
            }
            if height_map[i][j] == 1 {
                alt_starts.push((i as i32, j as i32));
            }
        }
    }

    let depth = find_shortest_route_len(&height_map, starting_postition, final_postition);
    println!("p1d: {}", depth);

    let mut min_d = depth;
    for start in alt_starts {
        let depth = find_shortest_route_len(&height_map, start, final_postition);
        if depth < min_d {
            min_d = depth;
        }
    }
    println!("p2d: {}", min_d);
}

fn find_shortest_route_len(
    height_map: &Vec<Vec<u8>>,
    starting_postition: (i32, i32),
    final_postition: (i32, i32),
) -> i32 {
    let mut visited = HashSet::new();
    let mut depth = 0;
    let mut check_queue = Vec::new();
    let x_lim = height_map.len();
    let y_lim = height_map[0].len();
    check_queue.push(starting_postition);
    visited.insert(starting_postition);
    'super_looper: for _i in 0..1000 {
        let depth_queue = check_queue.clone();
        check_queue = vec![];

        for point in depth_queue {
            for ptc in adjacent_points(point) {
                let (x, y) = ptc;
                if x >= x_lim as i32 || x < 0 || y >= y_lim as i32 || y < 0 {
                    continue;
                }

                if (height_map[x as usize][y as usize] as i32
                    - height_map[point.0 as usize][point.1 as usize] as i32)
                    > 1
                {
                    continue;
                }

                if visited.insert(ptc) {
                    check_queue.push(ptc);
                } 
                if ptc == final_postition {
                    depth += 1;
                    break 'super_looper;
                }
            }
        }
        depth += 1;
    }
    return depth;
}

fn adjacent_points(point: (i32, i32)) -> Vec<(i32, i32)> {
    let (x, y) = point;
    return vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];
}

fn load_input(file_path: &str) -> Vec<Vec<u8>> {
    let contents = fs::read_to_string(file_path).expect("file open died");
    let lines = contents.lines();

    let mut heights: Vec<Vec<u8>> = Vec::new();
    for l in lines {
        let mut line = Vec::new();
        for c in l.chars() {
            let val = char_height(c);
            line.push(val);
        }

        heights.push(line);
    }
    return heights;
}

fn char_height(c: char) -> u8 {
    return match c {
        'S' => 98,
        'E' => 99,
        _ => c as u8 - 96,
    };
}
