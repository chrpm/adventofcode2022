use std::fs;

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    let lines = contents.lines();

    let mut current_elf_cals = 0;
    let mut max_elf_cals = 0;
    let mut max_elf_cals_2 = 0;
    let mut max_elf_cals_3 = 0;
    for line in lines {
        if line == "" {
            if current_elf_cals > max_elf_cals {
                max_elf_cals_3 = max_elf_cals_2;
                max_elf_cals_2 = max_elf_cals;
                max_elf_cals = current_elf_cals
            } else if current_elf_cals > max_elf_cals_2 {
                max_elf_cals_3 = max_elf_cals_2;
                max_elf_cals_2 = current_elf_cals;
            } else if current_elf_cals > max_elf_cals_3 {
                max_elf_cals_3 = current_elf_cals;
            }
            current_elf_cals = 0;
            continue
        }
        let cals = line.parse::<i32>().unwrap();
        current_elf_cals += cals
    }

    println!("{}", max_elf_cals+max_elf_cals_2+max_elf_cals_3)
}