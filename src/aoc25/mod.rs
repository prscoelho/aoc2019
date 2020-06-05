use std::io;

use regex::Regex;

// solved manually by drawing the map and picking up safe items
// unsafe items to pick up are:
// infinite loop (heh..), escape pod, photons, giant electromagnet, molten lava
// the correct combination for my input was: <name> (path from entrance)
// - mutex  (NNN)
// - loom   (NEN)
// - sand   (WW)
// - wreath (WWSEN)
// finally, exit is at (WWNEE)
// cargo run --release -- src/aoc25/input < src/aoc25/solution
pub fn solve_first(input: &str) -> u32 {
    let memory = read_codes(input);
    let mut intcode = Intcode::new(memory);
    let mut buf = String::new();
    let answer_regex = Regex::new(
        r"You should be able to get in by typing (\d+) on the keypad at the main airlock.",
    )
    .unwrap();

    while !intcode.finished {
        intcode.run_until_input();

        let output: String = intcode.output.iter().map(|&v| v as u8 as char).collect();
        println!("{}", output);
        if let Some(group) = answer_regex.captures(&output) {
            return group[1].parse().unwrap();
        }
        intcode.output.clear();
        io::stdin().read_line(&mut buf).unwrap();

        for c in buf.chars() {
            intcode.input.push_back(c as i64);
        }
        buf.clear();
    }
    0
}

// ------ INTCODE (added run_until_input and run_instruction returns early
// on input instruction if there's no input to consume) -----------
use std::collections::VecDeque;

fn read_codes(input: &str) -> Vec<i64> {
    let mut result = Vec::new();

    for number_str in input.trim().split(',') {
        result.push(number_str.parse().unwrap());
    }
    result
}

// returns (A, B, C, DE)
fn decode_op(code: i64) -> (ParameterMode, ParameterMode, ParameterMode, i64) {
    let de = code % 100;
    let c = ParameterMode::decode(code / 100 % 10);
    let b = ParameterMode::decode(code / 1000 % 10);
    let a = ParameterMode::decode(code / 10000 % 10);

    (a, b, c, de)
}

struct Intcode {
    memory: Vec<i64>,
    ptr: usize,
    input: VecDeque<i64>,
    output: VecDeque<i64>,
    relative: i64,
    finished: bool,
}

enum ParameterMode {
    Position,
    Immediate,
    Relative,
}

impl ParameterMode {
    fn decode(n: i64) -> ParameterMode {
        match n {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => panic!("Unexpected parameter mode"),
        }
    }
}

impl Intcode {
    fn new(memory: Vec<i64>) -> Self {
        Intcode {
            memory,
            ptr: 0,
            relative: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            finished: false,
        }
    }

    fn load_value(&mut self, index: usize, mode: ParameterMode) -> i64 {
        match mode {
            ParameterMode::Position => {
                let position = self.read_memory(index) as usize;
                self.read_memory(position)
            }
            ParameterMode::Immediate => self.read_memory(index),
            ParameterMode::Relative => {
                let position = self.read_memory(index) + self.relative;
                self.read_memory(position as usize)
            }
        }
    }

    fn save_value(&mut self, index: usize, mode: ParameterMode, value: i64) {
        match mode {
            ParameterMode::Position => {
                let position = self.read_memory(index) as usize;
                self.write_memory(position, value);
            }
            ParameterMode::Immediate => {
                self.write_memory(index, value);
            }
            ParameterMode::Relative => {
                let position = self.read_memory(index) + self.relative;
                self.write_memory(position as usize, value);
            }
        };
    }

    fn read_memory(&mut self, index: usize) -> i64 {
        if self.memory.len() <= index {
            self.memory.resize(index + 1, 0);
        }
        self.memory[index]
    }

    fn write_memory(&mut self, index: usize, value: i64) {
        if self.memory.len() <= index {
            self.memory.resize(index + 1, 0);
        }
        self.memory[index] = value;
    }

    // returns instruction ran
    fn run_instruction(&mut self) -> i64 {
        if self.finished {
            return 99;
        }

        let pointer = self.ptr;
        let code = self.memory[pointer];
        let (arg3_mode, arg2_mode, arg1_mode, op) = decode_op(code);
        let next_pointer = match op {
            1 | 2 => {
                let value1 = self.load_value(pointer + 1, arg1_mode);
                let value2 = self.load_value(pointer + 2, arg2_mode);
                let operation_result = match op {
                    1 => value1 + value2,
                    2 => value1 * value2,
                    _ => 0,
                };
                self.save_value(pointer + 3, arg3_mode, operation_result);
                pointer + 4
            }
            3 => {
                if let Some(input_value) = self.input.pop_front() {
                    self.save_value(pointer + 1, arg1_mode, input_value);
                    pointer + 2
                } else {
                    // special OP code to indicate intcode requires input
                    // somewhat hacky but we want to pause the intcode on input
                    // only when it asks for it and doesn't have any
                    return 10;
                }
            }
            4 => {
                let v = self.load_value(pointer + 1, arg1_mode);
                self.output.push_back(v);
                pointer + 2
            }
            5 => {
                let par1 = self.load_value(pointer + 1, arg1_mode);
                let par2 = self.load_value(pointer + 2, arg2_mode) as usize;
                if par1 != 0 {
                    par2
                } else {
                    pointer + 3
                }
            }
            6 => {
                let par1 = self.load_value(pointer + 1, arg1_mode);
                let par2 = self.load_value(pointer + 2, arg2_mode) as usize;
                if par1 == 0 {
                    par2
                } else {
                    pointer + 3
                }
            }
            7 => {
                let par1 = self.load_value(pointer + 1, arg1_mode);
                let par2 = self.load_value(pointer + 2, arg2_mode);

                let store_value = if par1 < par2 { 1 } else { 0 };
                self.save_value(pointer + 3, arg3_mode, store_value);
                pointer + 4
            }
            8 => {
                let par1 = self.load_value(pointer + 1, arg1_mode);
                let par2 = self.load_value(pointer + 2, arg2_mode);

                let store_value = if par1 == par2 { 1 } else { 0 };
                self.save_value(pointer + 3, arg3_mode, store_value);
                pointer + 4
            }
            9 => {
                let par1 = self.load_value(pointer + 1, arg1_mode);
                self.relative += par1;
                pointer + 2
            }
            99 => {
                self.finished = true;
                pointer + 1
            }
            _ => pointer + 1,
        };
        self.ptr = next_pointer;
        op
    }

    fn run_until_output(&mut self) {
        while self.ptr < self.memory.len() {
            let op = self.run_instruction();
            if op == 99 || op == 4 {
                break;
            }
        }
    }

    fn run_until_input(&mut self) {
        while self.ptr < self.memory.len() {
            let op = self.run_instruction();
            if op == 99 || op == 10 {
                break;
            }
        }
    }
}
