use std::{collections::HashSet, fs};

fn main() {
    let file_path = "joey_input.txt";
    let instructions = instructions_from_file(file_path);

    let result_p1 = play_simulation(&instructions, 2);
    let result_p2 = play_simulation(&instructions, 10);

    println!("Number of spaces p1: {result_p1} p2: {result_p2}");
}

fn play_simulation(instructions: &Vec<Instruction>, rope_length: i32) -> i32 {
    let mut uniq_pos: HashSet<(i32, i32)> = HashSet::new();

    let mut positions = Vec::new();
    for _i in 0..rope_length {
        positions.push((0, 0));
    }

    for ins in instructions {
        for _i in 0..ins.moves {
            match ins.dir {
                Dir::Up => positions[0].1 += 1,
                Dir::Down => positions[0].1 -= 1,
                Dir::Left => positions[0].0 -= 1,
                Dir::Right => positions[0].0 += 1,
            }
            for i in 1..positions.len() {
                positions[i] = new_tail_pos_from_head_pos(positions[i], positions[i - 1]);
            }
            uniq_pos.insert(positions[positions.len() - 1]);
        }
    }
    return i32::try_from(uniq_pos.len()).unwrap();
}

fn new_tail_pos_from_head_pos(current_pos: (i32, i32), head_pos: (i32, i32)) -> (i32, i32) {
    let x_away = head_pos.0 - current_pos.0;
    let y_away = head_pos.1 - current_pos.1;

    if x_away.abs() <= 1 && y_away.abs() <= 1 {
        return current_pos;
    }

    let x_move = if x_away != 0 {
        x_away / x_away.abs()
    } else {
        0
    };
    let y_move = if y_away != 0 {
        y_away / y_away.abs()
    } else {
        0
    };

    return (current_pos.0 + x_move, current_pos.1 + y_move);
}

fn instructions_from_file(file_path: &str) -> Vec<Instruction> {
    let contents = fs::read_to_string(file_path).expect("file open died");
    let lines = contents.lines();

    let mut instructions = Vec::new();
    for l in lines {
        let ins = instruction_from_line(l);
        instructions.push(ins);
    }

    return instructions;
}

fn instruction_from_line(line: &str) -> Instruction {
    let parts: Vec<&str> = line.split(" ").collect();

    let dir = dir_from_input(parts[0]).expect("invalid direction");
    let moves = parts[1].parse::<i32>().expect("number is bad");

    return Instruction {
        dir: dir,
        moves: moves,
    };
}

fn dir_from_input(input: &str) -> Option<Dir> {
    let d = match input {
        "R" => Dir::Right,
        "U" => Dir::Up,
        "L" => Dir::Left,
        "D" => Dir::Down,
        &_ => return None,
    };
    return Some(d);
}

struct Instruction {
    dir: Dir,
    moves: i32,
}

enum Dir {
    Left,
    Right,
    Down,
    Up,
}
