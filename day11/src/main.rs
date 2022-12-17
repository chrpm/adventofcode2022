fn main() {
    let mut monkeys = get_monkeys();
    // let mut monkeys = get_test_monkeys();

    let mut counts = vec![0, 0, 0, 0, 0, 0, 0, 0];
    for _round in 0..10000 {
        for monkey in 0..monkeys.len() {
            money_round(&mut monkeys, monkey, &mut counts);
        }
    }

    for monkey in monkeys {
        println!("monkey inspection {:?}", monkey)
    }
    println!("monkey inspections {:?}", counts)
}

fn money_round(monkeys: &mut Vec<Monkey>, monkey: usize, counts: &mut Vec<i32>) {
    let item_list = monkeys[monkey].starting_items.clone();
    monkeys[monkey].starting_items = vec![];
    for item in item_list {
        counts[monkey] += 1;
        let new_val = (monkeys[monkey].operation)(item);
        let new_val_post_worry = new_val % 9699690;
        let next_monkey = if (monkeys[monkey].test)(new_val_post_worry) {
            monkeys[monkey].test_pass_next
        } else {
            monkeys[monkey].test_fail_next
        };

        monkeys[next_monkey].starting_items.push(new_val_post_worry);
    }
}

#[derive(Debug)]
struct Monkey {
    starting_items: Vec<i64>,
    operation: fn(i64) -> i64,
    test: fn(i64) -> bool,
    test_pass_next: usize,
    test_fail_next: usize,
}

fn get_monkeys() -> Vec<Monkey> {
    return vec![
        Monkey {
            starting_items: vec![80],
            operation: |x| {
                return x * 5;
            },
            test: |x| {
                return x % 2 == 0;
            },
            test_pass_next: 4,
            test_fail_next: 3,
        },
        Monkey {
            starting_items: vec![75, 83, 74],
            operation: |x| {
                return x + 7;
            },
            test: |x| {
                return x % 7 == 0;
            },
            test_pass_next: 5,
            test_fail_next: 6,
        },
        Monkey {
            starting_items: vec![86, 67, 61, 96, 52, 63, 73],
            operation: |x| {
                return x + 5;
            },
            test: |x| {
                return x % 3 == 0;
            },
            test_pass_next: 7,
            test_fail_next: 0,
        },
        Monkey {
            starting_items: vec![85, 83, 55, 85, 57, 70, 85, 52],
            operation: |x| {
                return x + 8;
            },
            test: |x| {
                return x % 17 == 0;
            },
            test_pass_next: 1,
            test_fail_next: 5,
        },
        Monkey {
            starting_items: vec![67, 75, 91, 72, 89],
            operation: |x| {
                return x + 4;
            },
            test: |x| {
                return x % 11 == 0;
            },
            test_pass_next: 3,
            test_fail_next: 1,
        },
        Monkey {
            starting_items: vec![66, 64, 68, 92, 68, 77],
            operation: |x| {
                return x * 2;
            },
            test: |x| {
                return x % 19 == 0;
            },
            test_pass_next: 6,
            test_fail_next: 2,
        },
        Monkey {
            starting_items: vec![97, 94, 79, 88],
            operation: |x| {
                return x * x;
            },
            test: |x| {
                return x % 5 == 0;
            },
            test_pass_next: 2,
            test_fail_next: 7,
        },
        Monkey {
            starting_items: vec![77, 85],
            operation: |x| {
                return x + 6;
            },
            test: |x| {
                return x % 13 == 0;
            },
            test_pass_next: 4,
            test_fail_next: 0,
        },
    ];
}

fn get_test_monkeys() -> Vec<Monkey> {
    return vec![
        Monkey {
            starting_items: vec![79, 98],
            operation: |x| {
                return x * 19;
            },
            test: |x| {
                return x % 23 == 0;
            },
            test_pass_next: 2,
            test_fail_next: 3,
        },
        Monkey {
            starting_items: vec![54, 65, 75, 74],
            operation: |x| {
                return x + 6;
            },
            test: |x| {
                return x % 19 == 0;
            },
            test_pass_next: 2,
            test_fail_next: 0,
        },
        Monkey {
            starting_items: vec![79, 60, 97],
            operation: |x| {
                return x * x;
            },
            test: |x| {
                return x % 13 == 0;
            },
            test_pass_next: 1,
            test_fail_next: 3,
        },
        Monkey {
            starting_items: vec![74],
            operation: |x| {
                return x + 3;
            },
            test: |x| {
                return x % 17 == 0;
            },
            test_pass_next: 0,
            test_fail_next: 1,
        },
    ];
}

