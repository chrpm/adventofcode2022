use std::{collections::HashMap, fs};

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("file open died");
    let lines: Vec<&str> = contents.lines().collect();

    let mut current_path: Vec<&str> = Vec::new();
    let mut dir_sizes = HashMap::new();
    let mut total_size = 0;
    for l in lines {
        let parts: Vec<&str> = l.split(" ").collect();
        if parts[0] == "$" {
            if parts[1] == "cd" {
                if parts[2] == ".." {
                    current_path.pop();
                } else {
                    current_path.push(parts[2]);
                }
            }
        } else {
            if parts[0] != "dir" {
                let size = parts[0].parse::<i32>().unwrap();
                total_size += size;

                let mut current_path_copy = current_path.clone();
                while current_path_copy.len() > 0 {
                    let d = current_path_copy.join("/");
                    let current_size = dir_sizes.get(d.as_str());
                    if current_size.is_some() {
                        dir_sizes.insert(d, size + current_size.unwrap());
                    } else {
                        dir_sizes.insert(d, size);
                    }
                    current_path_copy.pop();
                }
            }
        }
    }

    let mut sum = 0;
    let max = 100_000;
    let smallest_delete = 358913;
    let mut current_smallest = 999_999_999;
    for d in dir_sizes.keys() {
        let size = dir_sizes.get(d).unwrap();
        if size <= &max {
            sum += size;
        }
        if size >= &smallest_delete && size < &current_smallest {
            current_smallest = *size;
        }
    }
    println!("{} - {}", dir_sizes.get("/").unwrap(), total_size);
    println!("{}", sum);
    println!("{}", current_smallest);
}
