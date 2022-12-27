use std::{fs, collections::HashSet};
use regex::Regex;

fn test() {
    assert_eq!(manhattan_dist(0, 0, 1, 1), 2);
    assert_eq!(manhattan_dist(0, 0, -1, 1), 2);
    assert_eq!(manhattan_dist(0, 0, 2, 3), 5);
    assert_eq!(manhattan_dist(0, 0, 0, 0), 0);
    assert_eq!(beacon_area_around_sensor(0,0,0,2,10).len(), 0);
    assert_eq!(beacon_area_around_sensor(0,0,0,2,1).len(), 3);
}

fn main() {
    test();

    // let file_path = "test_input.txt";
    // let line_to_check = 10;
    // let max_bound = 20;

    let file_path = "input.txt";
    let line_to_check = 2000000;
    let max_bound = 4000000;

    let sensors = load_input(file_path);


    let ans_p1 = part_1(&sensors, line_to_check);
    println!("ans 1: {ans_p1}");

    let ans_p2 = part_2(&sensors, max_bound);
    println!("ans 2: {ans_p2}");
}

fn part_1(sensors: &Vec<Sensor>, line_to_check: i32) -> i32 {
    let mut postitions:HashSet<(i32, i32)>= HashSet::new();
    for s in sensors {
        let bas = beacon_area_around_sensor(s.coordinate_x, s.coordinate_y, s.beacon_x, s.beacon_y, line_to_check);

        for ba in bas {
            postitions.insert(ba);
        }
    }

    for s in sensors {
        postitions.remove(&(s.beacon_x, s.beacon_y));
    }

    let pos_count = postitions.iter().filter(|x| x.1 == line_to_check).count();
    return pos_count as i32;
}

fn part_2(sensors: &Vec<Sensor>, max_bound: i32) -> i64 {
    let mut answer_point: (i64, i64) = (0,0);

    'outer: for s in sensors {
        let md_to_check = s.becon_manhatthan + 1;

        let mut x_dir = 1;
        let mut y_dir = 1;
        let start_point = (s.coordinate_x - md_to_check, s.coordinate_y);
        let mut current_point = start_point;

        for i in 1..4*md_to_check+1 {
            let was_answer = is_answer(current_point.0, current_point.1, &sensors, max_bound);
            if was_answer {
                answer_point = (current_point.0 as i64, current_point.1 as i64);
                break 'outer;
            }
            current_point.0 += x_dir;
            current_point.1 += y_dir;
            if i == md_to_check {
                y_dir = -1;
            }
            if i == md_to_check*2 {
                x_dir = -1
            }
            if i == md_to_check*3 {
                y_dir = 1
            }
        }
    }

    let tuning_freq = (4000000 * answer_point.0) + answer_point.1;
    return tuning_freq;
}

fn is_answer(x: i32, y: i32, sensors: &Vec<Sensor>, max_bound: i32) -> bool {
    if x > max_bound || y > max_bound || x < 0 || y < 0 {
        return false;
    }

    for s in sensors {
        if manhattan_dist(x, y, s.coordinate_x, s.coordinate_y) <= s.becon_manhatthan {
            return false
        }
    }

    return true;
}


fn beacon_area_around_sensor(s_x: i32, s_y: i32, b_x: i32, b_y: i32, y_to_check: i32) -> Vec<(i32, i32)> {
    let mut points = Vec::new();

    let md = manhattan_dist(s_x, s_y, b_x, b_y);

    for i  in s_x-md..s_x+md+1 {
        if manhattan_dist(s_x, s_y, i, y_to_check) <= md {
            points.push((i, y_to_check));
        }
    }

    return points;
}

fn manhattan_dist(a_x: i32, a_y: i32, b_x: i32, b_y: i32) -> i32 {
    return (a_x-b_x).abs() + (a_y-b_y).abs()
}

fn load_input(file_path: &str) -> Vec<Sensor> {
    let contents = fs::read_to_string(file_path).unwrap();

    let mut sensors = Vec::new(); 

    let re = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
    for cap in re.captures_iter(&contents) {

        let coordinate_x = cap[1].parse::<i32>().unwrap();
        let coordinate_y = cap[2].parse::<i32>().unwrap();
        let beacon_x =  cap[3].parse::<i32>().unwrap();
        let beacon_y = cap[4].parse::<i32>().unwrap();
        let s = Sensor {
            coordinate_x,
            coordinate_y,
            beacon_x,
            beacon_y,
            becon_manhatthan: manhattan_dist(coordinate_x,coordinate_y, beacon_x, beacon_y)
        };
        sensors.push(s);
    }

    return sensors;
}


#[derive(Debug)]
struct Sensor {
    coordinate_x: i32,
    coordinate_y: i32,
    beacon_x: i32,
    beacon_y: i32,
    becon_manhatthan: i32,
}
