use std::{collections::HashSet, fs};

fn main() {
    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("file open died");
    let lines: Vec<&str> = contents.lines().collect();

    let mut trees: Vec<Vec<i32>> = Vec::new();
    for line in lines {
        let ts = line.trim().split("");
        let mut tree_line = Vec::new();
        for t in ts {
            let val = t.parse::<i32>();
            if val.is_ok() {
                tree_line.push(val.unwrap());
            }
        }
        trees.push(tree_line);
    }

    let cols_len = trees[0].len();
    let rows_len = trees.len();

    let mut found_trees: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..rows_len {
        let mut left_max = -1;
        let mut right_max = -1;
        for j in 0..cols_len {
            let r_j = cols_len - (j + 1);
            let left_val = trees[i][j];
            let right_val = trees[i][r_j];

            if left_val > left_max {
                found_trees.insert((i, j));
                left_max = left_val;
            }

            if right_val > right_max {
                found_trees.insert((i, r_j));
                right_max = right_val;
            }
        }
    }

    for i in 0..cols_len {
        let mut left_max = -1;
        let mut right_max = -1;
        for j in 0..rows_len {
            let r_j = rows_len - (j + 1);
            let left_val = trees[j][i];
            let right_val = trees[r_j][i];

            if left_val > left_max {
                found_trees.insert((j, i));
                left_max = left_val;
            }

            if right_val > right_max {
                found_trees.insert((r_j, i));
                right_max = right_val;
            }
        }
    }

    let mut max_scenic = 0;
    for i in 0..rows_len {
        for j in 0..cols_len {
            let score = calc_score(&trees, i, j, rows_len, cols_len);
            if score > max_scenic {
                max_scenic = score;
            }
        }
    }

    println!("result: {}", calc_score(&trees, 50, 50, rows_len, cols_len));

    println!("p1: {}", found_trees.len());
    println!("p2: {}", max_scenic);
}
fn calc_score(trees: &Vec<Vec<i32>>, i: usize, j: usize, i_size: usize, j_size: usize) -> i32 {
    let main_val = trees[i][j];

    let mut a = 0;
    for ii in 0..i {
        a += 1;
        let val = trees[i - (ii + 1)][j];
        if val >= main_val {
            break;
        }
    }

    let mut b = 0;
    for ii in i + 1..i_size {
        b += 1;
        let val = trees[ii][j];
        if val >= main_val {
            break;
        }
    }

    let mut c = 0;
    for ii in 0..j {
        c += 1;
        let val = trees[i][j - (ii + 1)];
        if val >= main_val {
            break;
        }
    }

    let mut d = 0;
    for ii in j + 1..j_size {
        d += 1;
        let val = trees[i][ii];
        if val >= main_val {
            break;
        }
    }

    return a * b * d * c;
}
