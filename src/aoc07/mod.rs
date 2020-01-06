use crossbeam::channel::unbounded;
use itertools::Itertools;
use std::thread;

fn read_codes(input: &str) -> Vec<i32> {
    let mut result = Vec::new();

    for number_str in input.split(',') {
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

struct Amplifier {
    memory: Vec<i32>,
    ptr: usize,
    input: crossbeam::Receiver<i32>,
    output: crossbeam::Sender<i32>,
}

impl Amplifier {
    fn new(
        memory: Vec<i32>,
        input: crossbeam::Receiver<i32>,
        output: crossbeam::Sender<i32>,
    ) -> Amplifier {
        Amplifier {
            memory,
            ptr: 0,
            input,
            output,
        }
    }

    fn load_value(&self, index: usize, mode: i32) -> i32 {
        if mode == 1 {
            self.memory[index]
        } else {
            self.memory[self.memory[index] as usize]
        }
    }

    fn save_value(&mut self, index: usize, mode: i32, value: i32) {
        if mode == 1 {
            self.memory[index] = value;
        } else {
            let position = self.memory[index] as usize;
            self.memory[position] = value;
        }
    }

    // returns instruction ran
    fn run_instruction(&mut self) -> i32 {
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
                let input_value = self.input.recv().unwrap();
                self.save_value(pointer + 1, arg1_mode, input_value);
                pointer + 2
            }
            4 => {
                let v = self.load_value(pointer + 1, arg1_mode);
                self.output.send(v).unwrap();
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
            _ => 1,
        };
        self.ptr = next_pointer;
        op
    }

    fn run(&mut self) {
        while self.ptr < self.memory.len() {
            let op = self.run_instruction();
            if op == 99 {
                break;
            }
        }
    }
}

pub fn solve_first(memory_input: &str) -> i32 {
    let mem = read_codes(memory_input);
    let mut max = i32::min_value();

    for permutation in (0..5).permutations(5) {
        let permutation_result = amplifier_circuit(mem.clone(), permutation);
        if permutation_result > max {
            max = permutation_result;
        }
    }
    max
}

pub fn solve_second(memory_input: &str) -> i32 {
    let mem = read_codes(memory_input);
    let mut max = i32::min_value();

    for permutation in (5..10).permutations(5) {
        let permutation_result = amplifier_circuit(mem.clone(), permutation);
        if permutation_result > max {
            max = permutation_result;
        }
    }
    max
}

fn amplifier_circuit(memory: Vec<i32>, amplifier_signals: Vec<i32>) -> i32 {
    // create all channels and set up initial input
    let mut senders: Vec<crossbeam::Sender<i32>> = Vec::new();
    let mut receivers: Vec<crossbeam::Receiver<i32>> = Vec::new();
    for i in 0..5 {
        let (sender, receiver) = unbounded();
        // send each amplifier its corresponding signal
        sender.send(amplifier_signals[i]).unwrap();

        senders.push(sender);
        receivers.push(receiver);
    }
    // send starting input to amplifier 0
    senders[0].send(0).unwrap();

    let mut children = Vec::new();
    // create tasks
    for id in 0..5 {
        let amplifier_mem = memory.clone();
        let amp_receiver = receivers[id].clone();
        // amplifier output is connected to the next amplifier's input
        // the last amplifier output is connected to amplifier 0
        let amp_sender = senders[(id + 1) % 5].clone();
        let child = thread::spawn(move || {
            let mut amp = Amplifier::new(amplifier_mem, amp_receiver, amp_sender);
            amp.run();
        });

        children.push(child);
    }

    // wait for all amplifiers to finish running
    for child in children {
        child.join().expect("panic on thread joining");
    }

    // read output of amp5, which is input of amp0, receivers[0]
    receivers[0].recv().unwrap()
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
    /*
    #[test]
    fn test_load() {
        let memory = vec![4,3,2,1,0];
        let loaded_value = load_value(&memory, 4, 0);
        assert_eq!(loaded_value, 4);

        let loaded_value = load_value(&memory, 4, 1);
        assert_eq!(loaded_value, 0);
    }

    #[test]
    fn test_save() {
        let mut memory = vec![4,3,2,1,0];

        save_value(&mut memory, 0,0,2);
        assert_eq!(memory[4], 2);

        save_value(&mut memory,0,1,3);
        assert_eq!(memory[0], 3);
    } */
    /*
    #[test]
    fn test_instruction() {
        let mut memory = vec![1101,100,-1,4,0];
        let mut input = VecDeque::new();
        let mut output = Vec::new();
        let (op, incr) = run_instruction(&mut memory,0, &mut input, &mut output);
        assert!(op == 1);
        assert!(incr == 4);
        assert!(memory[4] == 99);
    } */
    #[test]
    fn test_feedback1() {
        let input =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5";
        let result = solve_second(input);
        assert_eq!(result, 139629729);
    }

    #[test]
    fn test_feedback2() {
        let input = "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10";
        let result = solve_second(input);

        assert_eq!(result, 18216);
    }
}
