fn read_codes(input: &str) -> Vec<i32> {
    let mut result = Vec::new();

    for number_str in input.trim().split(',') {
        match number_str.parse() {
            Ok(num) => result.push(num),
            Err(e) => {
                println!(
                    "Error parsing number, input was: {}, error was: {}",
                    number_str, e
                );
                panic!();
            }
        }
    }
    result
}

// returns (A, B, C, DE)
fn decode_op(code: i32) -> (i32, i32, i32, i32) {
    let de = code % 100;
    let c = code / 100 % 10;
    let b = code / 1000 % 10;
    let a = code / 10000 % 10;

    (a, b, c, de)
}

fn load_value(memory: &[i32], index: usize, mode: i32) -> i32 {
    if mode == 1 {
        memory[index]
    } else {
        memory[memory[index] as usize]
    }
}

fn save_value(memory: &mut [i32], index: usize, mode: i32, value: i32) {
    if mode == 1 {
        memory[index] = value;
    } else {
        memory[memory[index] as usize] = value;
    }
}

// returns (instruction ran, pointer)
fn run_instruction(mut memory: &mut [i32], pointer: usize) -> (i32, usize) {
    let code = memory[pointer];
    let (arg3_mode, arg2_mode, arg1_mode, op) = decode_op(code);
    let next_pointer = match op {
        1 | 2 => {
            let value1 = load_value(memory, pointer + 1, arg1_mode);
            let value2 = load_value(memory, pointer + 2, arg2_mode);
            let operation_result = match op {
                1 => value1 + value2,
                2 => value1 * value2,
                _ => 0,
            };
            save_value(&mut memory, pointer + 3, arg3_mode, operation_result);
            pointer + 4
        }
        3 => {
            // read number from stdin, hacked together to always set 1 or 5 since we're already reading everything from stdin
            // fixing this requires switching program to read input from file and read stdin here
            save_value(&mut memory, pointer + 1, arg1_mode, 5);
            pointer + 2
        }
        4 => {
            println!("{}", load_value(memory, pointer + 1, arg1_mode));
            pointer + 2
        }
        5 => {
            let par1 = load_value(memory, pointer + 1, arg1_mode);
            let par2 = load_value(memory, pointer + 2, arg2_mode) as usize;
            if par1 != 0 {
                par2
            } else {
                pointer + 3
            }
        }
        6 => {
            let par1 = load_value(memory, pointer + 1, arg1_mode);
            let par2 = load_value(memory, pointer + 2, arg2_mode) as usize;
            if par1 == 0 {
                par2
            } else {
                pointer + 3
            }
        }
        7 => {
            let par1 = load_value(memory, pointer + 1, arg1_mode);
            let par2 = load_value(memory, pointer + 2, arg2_mode);

            let store_value = if par1 < par2 { 1 } else { 0 };
            save_value(&mut memory, pointer + 3, arg3_mode, store_value);
            pointer + 4
        }
        8 => {
            let par1 = load_value(memory, pointer + 1, arg1_mode);
            let par2 = load_value(memory, pointer + 2, arg2_mode);

            let store_value = if par1 == par2 { 1 } else { 0 };
            save_value(&mut memory, pointer + 3, arg3_mode, store_value);
            pointer + 4
        }
        _ => 1,
    };

    (op, next_pointer)
}

fn intcode_computer(mut memory: Vec<i32>) {
    let mut instruction_pointer = 0;
    while instruction_pointer < memory.len() {
        let (op, next_pointer) = run_instruction(&mut memory, instruction_pointer);
        instruction_pointer = next_pointer;
        if op == 99 {
            break;
        }
    }
}

pub fn solve_first(input: &str) {
    let mem = read_codes(input);
    intcode_computer(mem);
}

pub fn solve_second(input: &str) {
    let mem = read_codes(input);
    intcode_computer(mem);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_decode1() {
        let code = 11102;
        let expected = (1, 1, 1, 02);
        let decoded = decode_op(code);
        assert_eq!(decoded, expected);
    }
    #[test]
    fn test_decode2() {
        let code = 10103;
        let expected = (1, 0, 1, 03);
        let decoded = decode_op(code);
        assert_eq!(decoded, expected);
    }

    #[test]
    fn test_load() {
        let memory = vec![4, 3, 2, 1, 0];
        let loaded_value = load_value(&memory, 4, 0);
        assert_eq!(loaded_value, 4);

        let loaded_value = load_value(&memory, 4, 1);
        assert_eq!(loaded_value, 0);
    }

    #[test]
    fn test_save() {
        let mut memory = vec![4, 3, 2, 1, 0];

        save_value(&mut memory, 0, 0, 2);
        assert_eq!(memory[4], 2);

        save_value(&mut memory, 0, 1, 3);
        assert_eq!(memory[0], 3);
    }

    #[test]
    fn test_instruction() {
        let mut memory = vec![1101, 100, -1, 4, 0];
        let (op, incr) = run_instruction(&mut memory, 0);
        assert!(op == 1);
        assert!(incr == 4);
        assert!(memory[4] == 99);
    }
}
