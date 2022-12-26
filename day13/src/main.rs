use std::fs;

fn test() {
    assert!(is_val_integer(&String::from("5")));
    assert!(!is_val_integer(&String::from("[5]")));
    assert_eq!(int_val_to_list(String::from("5")), "[5]");
    assert!(matches!( val_int_comparson(String::from("5"), String::from("4")), PairEval::Right));
    assert!(matches!( val_int_comparson(String::from("5"), String::from("6")), PairEval::Left));
    assert!(matches!( val_int_comparson(String::from("6"), String::from("6")), PairEval::Same));
}

fn main() {
    test();

    let file_path = "test_input.txt";
    let packet_pairs = load_input(file_path);

    let mut pairs_in_order:Vec<i32> = Vec::new();
    for (i, pair) in packet_pairs.iter().enumerate() {
        let (l, r) = pair;
        if is_pair_in_right_order(l.to_string(), r.to_string()) {
            pairs_in_order.push(i as i32 + 1);
        }
    }

    let sum: i32 = pairs_in_order.iter().sum();
    println!("sum: {}", sum);
}

fn is_pair_in_right_order(left: String, right: String) -> bool {
    println!("left: {}, right: {}", left, right);
    let eval = evaluate_pair(left, right);
    println!("eval: {:?}, decision: {}", eval, !matches!(eval, PairEval::Right));
    println!();
    return !matches!(eval, PairEval::Right);
}

fn evaluate_pair(left: String, right: String) -> PairEval {
    // println!("raw left: {} raw right: {}", left, right);
    let left_parts = split_input(left);
    let right_parts = split_input(right);
    // println!("spl left: {:?} spl right: {:?}", left_parts, right_parts);
    if left_parts.len() == 0 && right_parts.len() == 0 {
        return PairEval::Same;
    } else if left_parts.len() == 0 {
        return PairEval::Left;
    } else if right_parts.len() == 0 {
        return PairEval::Right;
    }


    for i in 0..left_parts.len() {
        if i >= right_parts.len() {
            return PairEval::Right;
        }

        let left_comparison:String = left_parts[i].to_string();
        let right_comparison:String = right_parts[i].to_string();
        let eval = match determine_comparsion_type(&left_comparison, &right_comparison) {
            ComparisonType::LeftToList =>  evaluate_pair(int_val_to_list(left_comparison), right_comparison),
            ComparisonType::RightToList => evaluate_pair(left_comparison, int_val_to_list(right_comparison)),
            ComparisonType::BothList => evaluate_pair(left_comparison, right_comparison),
            ComparisonType::BothInt => val_int_comparson(left_comparison, right_comparison),
        };

        if !matches!(eval, PairEval::Same) {
            return eval;
        }
    }

    // checked all left parts without decision but there's still right parts so left wins
    if right_parts.len() > left_parts.len() {
        return PairEval::Left;
    }

    println!();
    return PairEval::Same;
}

#[derive(Debug)]
enum ComparisonType {
    BothInt,
    LeftToList,
    RightToList,
    BothList
}

fn determine_comparsion_type(left: &String, right: &String) -> ComparisonType {
    let is_left_int = is_val_integer(left);
    let is_right_int = is_val_integer(right);

    if is_left_int && is_right_int {
        return  ComparisonType::BothInt;
    } else if is_left_int {
        return ComparisonType::LeftToList;
    } else if is_right_int {
        return ComparisonType::RightToList;
    } 

    return ComparisonType::BothList;
}

fn is_val_integer(val: &String) -> bool {
    let parse_result = val.parse::<i32>();
    return parse_result.is_ok();
}

fn int_val_to_list(val: String) -> String {
    return format!("[{}]", val);
}

fn val_int_comparson(left:String, right:String) -> PairEval {
    let left_parse= left.parse::<i32>().unwrap();
    let right_parse = right.parse::<i32>().unwrap();
    if left_parse < right_parse {
        return PairEval::Left;
    }
    if right_parse < left_parse {
        return PairEval::Right;
    }
    return PairEval::Same;
}


#[derive(Debug)]
enum PairEval {
    Left,
    Right,
    Same
}

fn split_input(input: String) -> Vec<String> {
    let mut split_vals = Vec::new();

    // trim list parens
    let trimmed_input = input.get(1..input.len()-1).unwrap().to_string();
    if trimmed_input.len() == 0 {
        return Vec::new();
    }

    let mut split_postions = Vec::new();

    let mut open_paren_count = 0;
    for (i, c) in trimmed_input.char_indices() {
        if c == '[' {
            open_paren_count += 1;
        }
        if c == ']' {
            open_paren_count -= 1;
        }

        if c == ',' && open_paren_count == 0 {
            split_postions.push(i);
        }
    }

    if split_postions.len() == 0 {
        return  vec![trimmed_input];
    }

    split_postions.push(trimmed_input.len());

    let mut prev_split_postition = 0;
    for s in  split_postions {
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
