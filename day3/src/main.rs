use std::collections::HashMap;
use std::fs;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).expect("file open died");

    let lines: Vec<&str> = contents.lines().collect();

    let mut sum = 0;
    let mut badge_sum = 0;
    let mut counter = 0;
    for line in &lines {
        sum += char_priority(matched_char(line)) as i32;
        if (counter + 1) % 3 == 0 {
            badge_sum += char_priority(matched_char_three(
                lines[counter],
                lines[counter - 1],
                lines[counter - 2],
            )) as i32;
        }
        counter += 1;
    }

    println!("sum: {sum}");
    println!("badge: {badge_sum}");
}

fn char_priority(c: char) -> u8 {
    let p = c as u8;
    if p >= 97 {
        return p - 96;
    }
    return p - 38;
}

fn matched_char(sack: &str) -> char {
    let (left, right) = split_sack(sack);

    let mut char_check = HashMap::new();
    for c in left.chars() {
        char_check.insert(c, true);
    }

    for c in right.chars() {
        if char_check.contains_key(&c) {
            return c;
        }
    }

    println!("{left} - {right} of {sack}");
    return 'a';
}

fn split_sack(sack: &str) -> (&str, &str) {
    let length = sack.len();
    let left = &sack[0..length / 2];
    let right = &sack[length / 2..length];
    return (left, right);
}

fn matched_char_three(one: &str, two: &str, three: &str) -> char {
    let mut char_check_one = HashMap::new();
    for c in one.chars() {
        char_check_one.insert(c, true);
    }

    let mut char_check_two = HashMap::new();
    for c in two.chars() {
        char_check_two.insert(c, true);
    }

    for c in three.chars() {
        if char_check_one.contains_key(&c) && char_check_two.contains_key(&c) {
            return c;
        }
    }

    println!("no match in sacks {one}  {two}  {three}");
    return 'a';
}
