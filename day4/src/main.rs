use std::fs;

fn main() {
    let file_path = "input.txt";

    let contents = fs::read_to_string(file_path).expect("file open died");

    let lines: Vec<&str> = contents.lines().collect();

    let mut sum = 0;
    let mut sum_2 = 0;
    for line in &lines {
        let p: Vec<&str> = line.split(",").collect();
        let left: Vec<&str> = p[0].split("-").collect();
        let right: Vec<&str> = p[1].split("-").collect();
        let ll = left[0].parse::<i32>().unwrap();
        let lr = left[1].parse::<i32>().unwrap();
        let rl = right[0].parse::<i32>().unwrap(); 
        let rr = right[1].parse::<i32>().unwrap();

        if (ll <= rl && lr >= rr) || (rl <= ll && rr >= lr) {
            sum += 1;
        }

        if (ll >= rl && ll <= rr) || (rl >= ll && rl <= lr) {
            println!("{ll} {lr} {rl} {rr}");
            sum_2 += 1;
        }
    }

    println!("sum: {sum}");
    println!("sum 2: {sum_2}");
}
