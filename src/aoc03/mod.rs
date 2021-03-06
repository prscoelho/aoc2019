use std::collections::HashMap;
use std::collections::HashSet;

fn read_wire(input: &str) -> HashMap<(i32, i32), i32> {
    let mut result = HashMap::new();

    let mut position = (0, 0);
    let mut steps = 0;
    for wire in input.split_terminator(|c| c == ',' || c == '\n') {
        let num = wire[1..].parse::<i32>().unwrap();
        match wire.chars().nth(0).unwrap() {
            'U' => {
                let next_position = (position.0, position.1 + num);
                for y in position.1 + 1..=next_position.1 {
                    steps += 1;
                    result.insert((position.0, y), steps);
                }
                position = next_position;
            }
            'L' => {
                let next_position = (position.0 - num, position.1);
                for x in (next_position.0..position.0).rev() {
                    steps += 1;
                    result.insert((x, position.1), steps);
                }
                position = next_position;
            }
            'D' => {
                let next_position = (position.0, position.1 - num);
                for y in (next_position.1..position.1).rev() {
                    steps += 1;
                    result.insert((position.0, y), steps);
                }
                position = next_position;
            }
            'R' => {
                let next_position = (position.0 + num, position.1);
                for x in position.0 + 1..=next_position.0 {
                    steps += 1;
                    result.insert((x, position.1), steps);
                }
                position = next_position;
            }
            _ => {
                continue;
            }
        }
    }

    result
}

pub fn solve_first(input: &str) -> i32 {
    let mut lines = input.lines();
    let w1: HashSet<(i32, i32)> = read_wire(lines.next().unwrap()).keys().copied().collect();
    let w2: HashSet<(i32, i32)> = read_wire(lines.next().unwrap()).keys().copied().collect();
    let mut result = i32::max_value();
    for position in w1.intersection(&w2) {
        let distance = position.0.abs() + position.1.abs();
        if distance > 0 && distance < result {
            result = distance;
        }
    }
    result
}

pub fn solve_second(input: &str) -> i32 {
    let mut lines = input.lines();
    let map1 = read_wire(lines.next().unwrap());
    let map2 = read_wire(lines.next().unwrap());

    let set1: HashSet<(i32, i32)> = map1.keys().copied().collect();
    let set2: HashSet<(i32, i32)> = map2.keys().copied().collect();

    let mut min_steps = i32::max_value();

    for position in set1.intersection(&set2) {
        let position_steps = map1.get(&position).unwrap() + map2.get(&position).unwrap();
        if position_steps < min_steps {
            min_steps = position_steps;
        }
    }
    min_steps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first() {
        let input = include_str!("input");
        assert_eq!(solve_first(input), 225);
    }

    #[test]
    fn test_second() {
        let input = include_str!("input");
        assert_eq!(solve_second(input), 35194);
    }
}
