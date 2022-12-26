use std::fs;

fn main() {
    let file_path = "test_input.txt";
    // let file_path = "input.txt";

    let input = load_input(file_path);

    println!("{:?}", input);
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
