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
    println!("raw left: {} raw right: {}", left, right);
    let left_parts = split_input(left);
    let right_parts = split_input(right);
    println!("spl left: {:?} spl right: {:?}", left_parts, right_parts);
    println!();
}

fn split_input(input: String) -> Vec<String> {
    let mut split_vals = Vec::new();

    // trim list parens
    let trimmed_input = input.get(1..input.len()-1).unwrap().to_string();
    if trimmed_input.len() == 0 {
        return Vec::new();
    }

    let mut split_postion = Vec::new();

    let mut open_paren_count = 0;
    for (i, c) in trimmed_input.char_indices() {
        if c == '[' {
            open_paren_count += 1;
        }
        if c == ']' {
            open_paren_count -= 1;
        }

        if c == ',' && open_paren_count == 0 {
            split_postion.push(i);
        }
    }

    if split_postion.len() == 0 {
        return  vec![trimmed_input];
    }

    split_postion.push(trimmed_input.len());

    let mut prev_split_postition = 0;
    for s in  split_postion {
        let ss = trimmed_input.get(prev_split_postition..s).unwrap().to_string();
        split_vals.push(ss);
        prev_split_postition = s+1;
    }

    return split_vals;
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
