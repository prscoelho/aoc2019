use std::cmp::Ordering;

fn split_number(mut number: i32) -> Vec<i32> {
    let mut result = Vec::new();
    while number > 0 {
        result.push(number % 10);
        number = number / 10;
    }
    result.reverse();
    result
}

fn valid_number_first(number: i32) -> bool {
    let nums = split_number(number);
    if nums.len() != 6 {
        return false;
    }

    let mut two_equal = false;

    for idx in 1..nums.len() {
        if nums[idx - 1] > nums[idx] {
            return false;
        }
        if nums[idx - 1] == nums[idx] {
            two_equal = true;
        }
    }

    two_equal
}

fn valid_number_second(number: i32) -> bool {
    let nums = split_number(number);
    if nums.len() != 6 {
        return false;
    }

    let mut two_equal = false;
    let mut equal_size = 1;

    for idx in 1..nums.len() {
        match nums[idx - 1].cmp(&nums[idx]) {
            Ordering::Less => {
                if equal_size == 2 {
                    two_equal = true;
                }
                equal_size = 1;
            }
            Ordering::Equal => {
                equal_size += 1;
            }
            Ordering::Greater => {
                return false;
            }
        }
    }
    two_equal || equal_size == 2
}

pub fn solve_first() -> i32 {
    let mut result = 0;
    for n in 359282..820401 {
        if valid_number_first(n) {
            result += 1
        }
    }
    result
}

pub fn solve_second() -> i32 {
    let mut result = 0;
    for n in 359282..820401 {
        if valid_number_second(n) {
            result += 1
        }
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_first() {
        assert!(valid_number_first(111111));
        assert!(!valid_number_first(223450));
        assert!(!valid_number_first(123789));
        assert!(valid_number_first(111123));
    }

    #[test]
    fn test_valid_second() {
        assert!(valid_number_second(112233));
        assert!(!valid_number_second(123444));
        assert!(valid_number_second(111122));
    }

    #[test]
    fn test_first() {
        assert_eq!(solve_first(), 511);
    }
    #[test]
    fn test_second() {
        assert_eq!(solve_second(), 316);
    }
}
