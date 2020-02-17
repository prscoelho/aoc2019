use std::collections::BTreeSet;
use std::collections::VecDeque;

// checks for pull in every coordinate of given area
fn check_pull(intcode: &Intcode, width: usize, height: usize) -> BTreeSet<(usize, usize)> {
    let mut result = BTreeSet::new();
    for w in 0..width {
        for h in 0..height {
            let mut intcode_clone = intcode.clone();
            intcode_clone.input.push_back(w as i64);
            intcode_clone.input.push_back(h as i64);
            intcode_clone.run_until_output();
            if let Some(pulled) = intcode_clone.output.pop_front() {
                if pulled == 1 {
                    result.insert((w, h));
                }
            }
        }
    }
    result
}

pub fn solve_first(input: &str) -> usize {
    let memory = read_codes(input);
    let intcode = Intcode::new(memory);
    let pulls = check_pull(&intcode, 50, 50);
    pulls.iter().count()
}

fn check_fits(
    pull_locations: &BTreeSet<(usize, usize)>,
    point_x: usize,
    point_y: usize,
    size_w: usize,
    size_h: usize,
) -> bool {
    // check point, right top corner and bottom left corner
    // because of the shape of the tractor beam, if these 3 points
    // are pull locations, then we have a square of size_w * size_h
    pull_locations.contains(&(point_x, point_y))
        && pull_locations.contains(&(point_x + size_w - 1, point_y))
        && pull_locations.contains(&(point_x, point_y + size_h - 1))
}

fn scan(intcode: &Intcode, height: usize) -> BTreeSet<(usize, usize)> {
    let mut result = BTreeSet::new();
    for y in 0..height {
        // we can verify from plotting part 1 that
        // x will approximately be inbetween 0.65 * y to 0.85 * y
        let left = f64::floor(0.65f64 * (y as f64)) as usize;
        let right = f64::ceil(0.85f64 * (y as f64)) as usize;

        for x in left..=right {
            let mut intcode_clone = intcode.clone();
            intcode_clone.input.push_back(x as i64);
            intcode_clone.input.push_back(y as i64);
            intcode_clone.run_until_output();
            if let Some(pulled) = intcode_clone.output.pop_front() {
                if pulled == 1 {
                    result.insert((x, y));
                }
            }
        }
    }
    result
}

pub fn solve_second(input: &str) -> usize {
    let memory = read_codes(input);
    let intcode = Intcode::new(memory);
    let pulls = scan(&intcode, 1500);
    for &(x, y) in pulls.iter() {
        if check_fits(&pulls, x, y, 100, 100) {
            return x * 10000 + y;
        }
    }
    0
}

// ------ INTCODE (same as day 15) -----------

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
#[derive(Clone)]
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
                let input_value = self.input.pop_front().expect("Could not get input.");
                self.save_value(pointer + 1, arg1_mode, input_value);
                pointer + 2
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
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn compare_scan() {
        let input = include_str!("input");
        let memory = read_codes(input);
        let intcode = Intcode::new(memory);
        let checking_all = check_pull(&intcode, 50, 50);
        let checking_formula = scan(&intcode, 50);
        assert_eq!(checking_all, checking_formula);
    }

    #[test]
    fn first() {
        let input = include_str!("input");
        assert_eq!(solve_first(input), 154);
    }

    #[test]
    fn second() {
        let input = include_str!("input");
        assert_eq!(solve_second(input), 9791328);
    }
}
