use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
enum Action {
    Increment(i128),
    Cut(i128),
    Deal,
}

fn parse_actions(input: &str) -> Vec<Action> {
    let mut result = Vec::new();
    let r1 = Regex::new(r"deal with increment (\d+)").unwrap();
    let r2 = Regex::new(r"cut (-?\d+)").unwrap();
    let r3 = Regex::new("deal into new stack").unwrap();

    for line in input.lines() {
        if let Some(c) = r1.captures(line) {
            result.push(Action::Increment(c[1].parse().unwrap()));
        } else if let Some(c) = r2.captures(line) {
            result.push(Action::Cut(c[1].parse().unwrap()));
        } else if let Some(_) = r3.captures(line) {
            result.push(Action::Deal);
        }
    }

    result
}

// what is card's index after actions
fn card_find(mut idx: i128, size: i128, actions: &[Action]) -> i128 {
    for action in actions.iter() {
        match action {
            Action::Deal => idx = card_deal(idx, size),
            Action::Cut(cut) => idx = card_cut(idx, size, *cut),
            Action::Increment(incr) => idx = card_increment(idx, size, *incr),
        }
    }
    idx
}

fn card_increment(idx: i128, size: i128, incr: i128) -> i128 {
    (idx * incr) % size
}

fn card_cut(idx: i128, size: i128, cut: i128) -> i128 {
    (idx + size - cut) % size
}

fn card_deal(idx: i128, size: i128) -> i128 {
    size - idx - 1
}

fn rev_card_deal(res: i128, size: i128) -> i128 {
    size - res - 1
}

fn rev_card_cut(res: i128, size: i128, cut: i128) -> i128 {
    (res + cut + size) % size
}

fn egcd(a: i128, b: i128) -> (i128, i128, i128) {
    if a == 0 {
        (b, 0, 1)
    } else {
        let (g, y, x) = egcd(b % a, a);
        (g, x - (b / a) * y, y)
    }
}

fn modinv(a: i128, m: i128) -> i128 {
    let (g, x, _) = egcd(a, m);
    if g != 1 {
        panic!("Modular inverse does not exist");
    }
    x % m
}

fn mod_power(mut a: i128, mut b: i128, p: i128) -> i128 {
    let mut res = 1;

    a = a % p;
    if a == 0 {
        return 0;
    }
    while b > 0 {
        if b & 1 == 1 {
            res = (res * a) % p
        }
        b = b >> 1;
        a = (a * a) % p
    }
    res
}

fn rev_card_increment(res: i128, size: i128, incr: i128) -> i128 {
    (modinv(incr, size) * res) % size
}

pub fn solve_first(input: &str) -> u16 {
    let actions = parse_actions(input);
    card_find(2019, 10007, &actions) as u16
}

fn reverse_apply(mut res: i128, size: i128, actions: &[Action]) -> i128 {
    for action in actions.iter().rev() {
        match action {
            Action::Deal => res = rev_card_deal(res, size),
            Action::Cut(cut) => res = rev_card_cut(res, size, *cut),
            Action::Increment(incr) => res = rev_card_increment(res, size, *incr),
        }
    }
    res
}

fn mul_mod(mut a: i128, mut b: i128, m: i128) -> i128 {
    if a >= m {
        a %= m;
    }
    if b >= m {
        b %= m;
    }
    let x = a;
    let c = x * b / m;
    let r = (a * b - c * m) % m;
    if r < 0 {
        r + m
    } else {
        r
    }
}

pub fn solve_second(input: &str) -> i128 {
    let actions = parse_actions(input);
    let deck_size = 119315717514047;

    // https://www.reddit.com/r/adventofcode/comments/ee0rqi/2019_day_22_solutions/fbnifwk/
    let x = 2020;
    let y = reverse_apply(x, deck_size, &actions);
    let z = reverse_apply(y, deck_size, &actions);

    let a = ((y - z) * modinv(x - y, deck_size)) % deck_size;
    let b = (y - a * x) % deck_size;

    let times = 101741582076661;

    let one = mod_power(a, times, deck_size) * x;
    let two = mod_power(a, times, deck_size) - 1;
    let three = modinv(a - 1, deck_size);

    // mul_mod to avoid overflow
    // https://www.reddit.com/r/adventofcode/comments/ee0rqi/2019_day_22_solutions/fey51nx/
    (one + mul_mod(mul_mod(two, three, deck_size), b, deck_size)) % deck_size
}
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse() {
        let input = "deal into new stack
        cut -2203
        deal with increment 74";

        let result = parse_actions(input);
        let expected = vec![Action::Deal, Action::Cut(-2203), Action::Increment(74)];
        assert_eq!(result, expected);
    }

    #[test]
    fn first() {
        let input = include_str!("input");
        assert_eq!(solve_first(input), 7096);
    }

    #[test]
    fn example1() {
        let input = include_str!("example1");
        let actions = parse_actions(input);
        assert_eq!(card_find(1, 10, &actions), 7);
        assert_eq!(card_find(7, 10, &actions), 9);
        assert_eq!(card_find(4, 10, &actions), 8);
        //expected = vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7];
    }

    #[test]
    fn example2() {
        let input = include_str!("example2");
        let actions = parse_actions(input);
        assert_eq!(card_find(3, 10, &actions), 0);
        assert_eq!(card_find(6, 10, &actions), 9);
        assert_eq!(card_find(1, 10, &actions), 4);
        //expected = vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6];
    }

    #[test]
    fn single_cut() {
        assert_eq!(card_cut(0, 10, 3), 7);
        assert_eq!(card_cut(3, 10, 3), 0);
        assert_eq!(card_cut(6, 10, -4), 0);
        assert_eq!(card_cut(0, 10, -4), 4);
    }
    #[test]
    fn second() {
        let input = include_str!("input");
        assert_eq!(solve_second(input), 27697279941366);
    }
}
