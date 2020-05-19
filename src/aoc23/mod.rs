use crossbeam::{channel::unbounded, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

// starts every computer in its own thread,
// receives and sends packets to intcode threads
// finishes when it receives a packet addressed to 255
// returns the y value of that packet
fn run_network_255(network: Network) -> i64 {
    for mut computer in network.computers.into_iter() {
        thread::spawn(move || {
            computer.run();
        });
    }

    loop {
        for r in network.receive.iter() {
            if let Ok(to) = r.try_recv() {
                let x = r.recv().unwrap();
                let y = r.recv().unwrap();

                if to == 255 {
                    return y;
                }

                network.send[to as usize].send(x).unwrap();
                network.send[to as usize].send(y).unwrap();
            }
        }
    }
}

// forwards last nat packet to computer 0,
// keeps track of last nat packet y value sent, stop when it repeats
// returns the repeated y value
fn run_network_nat(network: Network) -> i64 {
    for mut computer in network.computers.into_iter() {
        thread::spawn(move || {
            computer.run();
        });
    }
    let mut nat_x = 0;
    let mut nat_y = 0;

    let mut last_sent = -1;
    let mut last_active = Instant::now();

    loop {
        for r in network.receive.iter() {
            if let Ok(to) = r.try_recv() {
                let x = r.recv().unwrap();
                let y = r.recv().unwrap();

                if to == 255 {
                    nat_x = x;
                    nat_y = y;
                } else {
                    network.send[to as usize].send(x).unwrap();
                    network.send[to as usize].send(y).unwrap();
                }
                last_active = Instant::now();
            }
        }
        // if there's no activity, send nat packet
        if last_active.elapsed() > Duration::from_millis(50) {
            if last_sent == nat_y {
                return last_sent;
            }
            last_sent = nat_y;
            network.send[0].send(nat_x).unwrap();
            network.send[0].send(nat_y).unwrap();
            last_active = Instant::now();
        }
    }
}

struct Network {
    send: Vec<Sender<i64>>,
    receive: Vec<Receiver<i64>>,
    computers: Vec<Intcode>,
}

fn create_network(n: usize, memory: Vec<i64>) -> Network {
    // how we send input to different computers
    let mut send = Vec::new();
    // how we receive output from different computers
    let mut receive = Vec::new();

    let mut computers = Vec::new();

    for address in 0..n {
        // network sends input to computers through this channel
        let (send_input, receive_input) = unbounded();

        // network receives output from computers through this channel
        let (send_output, receive_output) = unbounded();

        // send each computer its address
        send_input.send(address as i64).unwrap();

        send.push(send_input);
        receive.push(receive_output);

        let intcode = Intcode::new(memory.clone(), receive_input, send_output);
        computers.push(intcode);
    }

    Network {
        send,
        receive,
        computers,
    }
}

pub fn solve_first(input: &str) -> i64 {
    let memory = read_codes(input);
    let network = create_network(50, memory);
    run_network_255(network)
}

pub fn solve_second(input: &str) -> i64 {
    let memory = read_codes(input);
    let network = create_network(50, memory);
    run_network_nat(network)
}
#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn first() {
        let input = include_str!("input");
        assert_eq!(solve_first(input), 23815);
    }

    #[test]
    fn second() {
        let input = include_str!("input");
        assert_eq!(solve_second(input), 16666);
    }
}

// ------ INTCODE ---
// input and output are crossbeam channels
// timeouts for 5ms on no input with recv_timeout(5ms)
// and then continues with -1
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
    input: Receiver<i64>,
    output: Sender<i64>,
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
    fn new(memory: Vec<i64>, input: Receiver<i64>, output: Sender<i64>) -> Self {
        Intcode {
            memory,
            ptr: 0,
            relative: 0,
            input,
            output,
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
                // wait for input, then continue with value -1
                let input_value = match self.input.recv_timeout(Duration::from_millis(5)) {
                    Ok(value) => value,
                    _ => -1,
                };
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

    fn run(&mut self) {
        while self.ptr < self.memory.len() {
            let op = self.run_instruction();
            if op == 99 {
                break;
            }
        }
    }
}
