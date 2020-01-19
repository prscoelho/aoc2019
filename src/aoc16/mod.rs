fn read_numbers(input: &str) -> Vec<i32> {
    let mut result = Vec::new();

    for c in input.trim().chars() {
        result.push(c.to_digit(10).unwrap() as i32);
    }

    result
}

fn apply_phase(mut numbers: Vec<i32>, phases: usize) -> Vec<i32> {
    let pattern = vec![1, 0, -1, 0];
    let mut next = Vec::with_capacity(numbers.len());
    for _ in 0..phases {
        for idx1 in 0..numbers.len() {
            let mut val1_result = 0;
            for (idx2, val2) in numbers.iter().enumerate().skip(idx1) {
                // (idx2 - idx1) >= 0, since we skip(idx1)
                let pattern_value = pattern[(idx2 - idx1) / (idx1 + 1) % 4];
                val1_result += val2 * pattern_value;
            }
            next.push(val1_result.abs() % 10)
        }
        std::mem::swap(&mut numbers, &mut next);
        next.clear();
    }
    numbers
}

fn fold_to_number(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .fold(0, |total, current| total * 10 + current)
}

pub fn solve_first(input: &str) -> i32 {
    let numbers = read_numbers(input);
    let result = apply_phase(numbers, 100);

    fold_to_number(&result[0..8])
}

pub fn solve_second(input: &str) -> i32 {
    let numbers = read_numbers(input);
    let start = fold_to_number(&numbers[0..7]) as usize;
    let end = numbers.len() * 10_000;

    let mut current = Vec::new();
    for i in start..end {
        current.push(numbers[i % numbers.len()]);
    }

    for _ in 0..100 {
        let mut sums = Vec::new();
        let mut total = 0;
        sums.push(0);
        for i in 0..current.len() {
            total += current[i];
            sums.push(total);
        }

        for i in 0..current.len() {
            let value = sums.last().unwrap() - sums[i];
            current[i] = value % 10;
        }
    }
    fold_to_number(&current[0..8])
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example1() {
        let input = "80871224585914546619083218645595";
        let first = solve_first(input);
        let expected = 24176176;
        assert_eq!(first, expected);
    }
    #[test]
    fn example2() {
        let input = "19617804207202209144916044189917";
        let first = solve_first(input);
        let expected = 73745418;
        assert_eq!(first, expected);
    }
    #[test]
    fn example3() {
        let input = "69317163492948606335995924319873";
        let first = solve_first(input);
        let expected = 52432133;
        assert_eq!(first, expected);
    }

    #[test]
    fn example4() {
        let input = "03036732577212944063491565474664";
        let second = solve_second(input);
        let expected = 84462026;
        assert_eq!(second, expected);
    }

    #[test]
    fn example5() {
        let input = "02935109699940807407585447034323";
        let second = solve_second(input);
        let expected = 78725270;
        assert_eq!(second, expected);
    }

    #[test]
    fn example6() {
        let input = "03081770884921959731165446850517";
        let second = solve_second(input);
        let expected = 53553731;
        assert_eq!(second, expected);
    }

    #[test]
    fn first() {
        let input = include_str!("input");
        let result = solve_first(input);
        let expected = 63483758;
        assert_eq!(result, expected);
    }

    #[test]
    fn second() {
        let input = include_str!("input");
        let result = solve_second(input);
        let expected = 96099551;
        assert_eq!(result, expected);
    }
}
