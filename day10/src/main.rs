use std::fs;

fn main() {
    let file_path = "input.txt";
    let instructions = load_instruction(file_path);

    let mut cycle_vals: Vec<i32> = Vec::new();
    let mut val_to_add = 0;
    let mut need_add_val = false;
    let mut current_val = 1;
    for ins in instructions {
        if need_add_val {
            current_val += val_to_add;
            need_add_val = false;
        }

        if matches!(ins.op, Operation::Add) {
            cycle_vals.push(current_val);
            cycle_vals.push(current_val);
            need_add_val = true;
            val_to_add = ins.val;
        } else {
            cycle_vals.push(current_val);
        }
    }

    println!("{:?}", cycle_vals);
    let mut i = 0;
    for c in &cycle_vals {
        if (c - i).abs() <= 1 {
            print!("#");
        } else {
            print!(".");
        }
        i += 1;
        if i % 40 == 0 {
            i = 0;
            print!("\n");
        }
    }

    let cycles_of_interest: Vec<i32> = vec![20, 60, 100, 140, 180, 220];
    let mut cycle_sum = 0;
    for c in cycles_of_interest {
        println!(
            "{} {} {}",
            c,
            cycle_vals[(c - 1) as usize],
            c * cycle_vals[(c - 1) as usize]
        );
        cycle_sum += c * cycle_vals[(c - 1) as usize];
    }

    println!("sum {}", cycle_sum);
}

fn load_instruction(file_path: &str) -> Vec<Instruction> {
    let f = fs::read_to_string(file_path).expect("file couldnt read in");

    let mut ins: Vec<Instruction> = Vec::new();
    for l in f.lines() {
        let ps: Vec<&str> = l.split(" ").collect();

        let i = match ps[0] {
            "addx" => Operation::Add,
            _ => Operation::NoOp,
        };

        let n;
        if matches!(i, Operation::Add) {
            n = ps[1].parse::<i32>().expect("bad number");
        } else {
            n = 0;
        }

        ins.push(Instruction { op: i, val: n })
    }

    return ins;
}

struct Instruction {
    op: Operation,
    val: i32,
}

enum Operation {
    Add,
    NoOp,
}
