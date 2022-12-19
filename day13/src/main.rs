use std::fs;

fn main() {
    let file_path = "test_input.txt";
    let packet_pairs = load_input(file_path);

    for pair in packet_pairs {
        compare_values(pair.0, pair.1);
            break;
    }
}

fn compare_values(left: String, right: String) {
   println!("{} {}", left, right);
}


fn load_input(file_path: &str) -> Vec<(String, String)> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let lines:Vec<&str> = contents.lines().collect();
    let mut items: Vec<(String, String)> = Vec::new();
    for i in  0..lines.len() {
        if i % 3 != 0 {
            continue;
        }
        items.push((String::from(lines[i]), String::from(lines[i+1])));
    }

    return items;
}
