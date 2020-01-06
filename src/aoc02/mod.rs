fn read_codes() -> Vec<i32> {
    let mut input = String::new();
    let mut result = Vec::new();
    if std::io::stdin().read_line(&mut input).is_err() {
        return result;
    }
    for number_str in input.split(',') {
        result.push(number_str.parse().unwrap())
    }
    result
}

pub fn naive_run(mut memory: Vec<i32>, noun: i32, verb: i32) -> i32 {
    memory[1] = noun;
    memory[2] = verb;

    let mut position = 0;
    loop {
        if position >= memory.len() {
            break;
        }
        match memory[position] {
            1 => {
                let value1 = memory[memory[position + 1] as usize];
                let value2 = memory[memory[position + 2] as usize];
                let result_pos = memory[position + 3] as usize;
                memory[result_pos] = value1 + value2;
            }
            2 => {
                let value1 = memory[memory[position + 1] as usize];
                let value2 = memory[memory[position + 2] as usize];
                let result_pos = memory[position + 3] as usize;
                memory[result_pos] = value1 * value2;
            }
            99 | _ => {
                break;
            }
        }
        position += 4;
    }

    memory[0]
}

pub fn solve_first() -> i32 {
    let memory = read_codes();
    naive_run(memory, 12, 2)
}

pub fn solve_second() -> i32 {
    let memory = read_codes();
    for noun in 0..memory.len() {
        for verb in 0..memory.len() {
            if naive_run(memory.clone(), noun as i32, verb as i32) == 19690720 {
                return (noun * 100 + verb) as i32;
            }
        }
    }
    -1
}
