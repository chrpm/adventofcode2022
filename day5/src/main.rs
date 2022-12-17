use std::fs;

struct Instruction {
    num_to_move: i32,
    from_index: i32,
    to_index: i32,
}

/*
                    [Q]     [P] [P]
                [G] [V] [S] [Z] [F]
            [W] [V] [F] [Z] [W] [Q]
        [V] [T] [N] [J] [W] [B] [W]
    [Z] [L] [V] [B] [C] [R] [N] [M]
[C] [W] [R] [H] [H] [P] [T] [M] [B]
[Q] [Q] [M] [Z] [Z] [N] [G] [G] [J]
[B] [R] [B] [C] [D] [H] [D] [C] [N]
 1   2   3   4   5   6   7   8   9
*/

fn main() {
    let mut stacks: Vec<Vec<char>> = vec![
        vec!['B', 'Q', 'C'],
        vec!['R', 'Q', 'W', 'Z'],
        vec!['B', 'M', 'R', 'L', 'V'],
        vec!['C', 'Z', 'H', 'V', 'T', 'W'],
        vec!['D', 'Z', 'H', 'B', 'N', 'V', 'G'],
        vec!['H', 'N', 'P', 'C', 'J', 'F', 'V', 'Q'],
        vec!['D', 'G', 'T', 'R', 'W', 'Z', 'S'],
        vec!['C', 'G', 'M', 'N', 'B', 'W', 'Z', 'P'],
        vec!['N', 'J', 'B', 'M', 'W', 'Q', 'F', 'P'],
    ];

    let file_path = "input.txt";
    let contents = fs::read_to_string(file_path).expect("file open died");
    let lines: Vec<&str> = contents.lines().collect();
    let mut insts: Vec<Instruction> = Vec::new();

    for line in &lines {
        let parts: Vec<&str> = line.split(" ").collect();

        insts.push(Instruction {
            num_to_move: parts[1].parse::<i32>().unwrap(),
            from_index: parts[3].parse::<i32>().unwrap() - 1,
            to_index: parts[5].parse::<i32>().unwrap() - 1,
        });
    }

    for is in insts {
        // problem 1 loop
        // for _i in 0..is.num_to_move {
        // let p = stacks[is.from_index as usize].pop().unwrap();
        // stacks[is.to_index as usize].push(p);
        // }

        // problem 2 loop
        let mut temp: Vec<char> = Vec::new();
        for _i in 0..is.num_to_move {
            let p = stacks[is.from_index as usize].pop().unwrap();
            temp.push(p);
        }

        for _i in 0..is.num_to_move {
            let p = temp.pop().unwrap();
            stacks[is.to_index as usize].push(p);
        }
    }

    for s in stacks {
        println!("{}", s.last().unwrap());
    }
}
