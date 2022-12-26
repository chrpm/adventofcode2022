use std::fs;

fn main() {
    let file_path = "input.txt";

    let input = load_input(file_path);
    let mut max_y = 0;
    let mut grid = vec![vec!['.'; 200]; 1000];

    for path in input {
        let elements_to_put_rock_in = elements_to_rock_in_path(path);
        for e in elements_to_put_rock_in {
            if e.1 > max_y as i32 {
                max_y = e.1;
            }
            grid[e.0 as usize][e.1 as usize] = '#';
        }
    }

    for i in 0..grid.len() {
        grid[i][(max_y+2) as usize] = '#';
    }

    let sand_entry = (500, 0);
    let mut current_sand_point = sand_entry;
    let mut sand_at_rest = 0;
    let mut part_1_sand_count = 0;
    let part_2_sand_count;

    loop {
        if current_sand_point.1 > max_y && part_1_sand_count == 0 {
            part_1_sand_count = sand_at_rest;
        }

        if grid[current_sand_point.0 as usize][(current_sand_point.1+1) as usize] == '.' {
            current_sand_point = (current_sand_point.0, current_sand_point.1+1);
        } else if grid[current_sand_point.0-1][(current_sand_point.1+1) as usize] == '.' {
            current_sand_point = (current_sand_point.0-1, current_sand_point.1+1);
        } else if grid[current_sand_point.0+1][(current_sand_point.1+1) as usize] == '.' {
            current_sand_point = (current_sand_point.0+1, current_sand_point.1+1);
        } else {
            sand_at_rest += 1;
            grid[current_sand_point.0][current_sand_point.1 as usize] = 'o';
            if current_sand_point == sand_entry {
                part_2_sand_count = sand_at_rest;
                break;
            }
            current_sand_point = sand_entry;
        }
    }

    println!("successfully place sand p1:{} p2:{}", part_1_sand_count, part_2_sand_count);
}

fn elements_to_rock_in_path(path: Vec<(i32, i32)>) -> Vec<(i32, i32)> {
    let mut elements_to_rock = Vec::new();

    for i in 1..path.len() { 
        let p1 = path[i-1];
        let p2 = path[i];
        let mut points_rocks = elements_to_rock_between_points(p1, p2);
        elements_to_rock.append(&mut points_rocks);
    }

    return elements_to_rock;
}

fn elements_to_rock_between_points(p1: (i32, i32), p2:(i32, i32)) -> Vec<(i32, i32)> {
    let mut rocks = Vec::new();
    if p1.0 > p2.0 {
        for i in 0..(p1.0 - p2.0)+1 {
            rocks.push((p2.0+i, p2.1))
        }
    }

    if p1.0 < p2.0 {
        for i in 0..(p2.0 - p1.0)+1 {
            rocks.push((p1.0+i, p1.1))
        }
    }

    if p1.1 > p2.1 {
        for i in 0..(p1.1 - p2.1)+1 {
            rocks.push((p2.0, p2.1+i))
        }
    }

    if p1.1 < p2.1 {
        for i in 0..(p2.1 - p1.1)+1 {
            rocks.push((p1.0, p1.1+i))
        }
    }

    return rocks;
}

fn load_input(file_path: &str) -> Vec<Vec<(i32, i32)>> {
    let contents = fs::read_to_string(file_path).unwrap();
    let lines:Vec<&str> = contents.lines().collect();

    let mut paths = Vec::new();
    for l in lines {
        let positions:Vec<&str> = l.split(" -> ").collect();
        let mut parsed_positions = Vec::new();
        for p in positions {
            let vals:Vec<&str> = p.split(",").collect();
            let a = vals[0].parse::<i32>().unwrap();
            let b = vals[1].parse::<i32>().unwrap();
            parsed_positions.push((a, b));
        }
        paths.push(parsed_positions);
    }

    return paths;
}
