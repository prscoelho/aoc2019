fn read_ints(input: &str) -> Vec<i32> {
    let mut result = Vec::new();

    for line in input.lines() {
        result.push(line.trim().parse::<i32>().unwrap());
    }
    result
}

fn fuel_required(mass: i32) -> i32 {
    mass / 3 - 2
}

pub fn solve_first(input: &str) -> i32 {
    let masses = read_ints(input);
    let mut result = 0;
    for mass in masses {
        result += fuel_required(mass);
    }
    result
}

pub fn solve_second(input: &str) -> i32 {
    let masses = read_ints(input);
    let mut result = 0;
    for mass in masses {
        let mut required = fuel_required(mass);
        result += required;
        loop {
            required = fuel_required(required);
            if required <= 0 {
                break;
            }
            result += required;
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_first() {
        let input = include_str!("input");
        assert_eq!(solve_first(input), 3315383);
    }

    #[test]
    fn test_second() {
        let input = include_str!("input");
        assert_eq!(solve_second(input), 4970206);
    }
}
