use std::collections::HashSet;
use std::f64::consts::{FRAC_PI_2, PI};
use std::ops;

struct Asteroids {
    data: HashSet<Position>,
    width: i32,
    height: i32,
}

impl Asteroids {
    fn read_asteroids(input: &str) -> Asteroids {
        let mut data = HashSet::new();
        let mut width = 0;
        let mut height = 0;
        for line in input.lines() {
            width = 0;
            for c in line.trim().chars() {
                if c == '#' {
                    data.insert(Position(width, height));
                }

                width += 1;
            }
            height += 1;
        }
        Asteroids {
            data,
            width,
            height,
        }
    }

    fn count_reach(&self, from_position: Position) -> u32 {
        let mut normalized_found: HashSet<Position> = HashSet::new();

        for asteroid_position in self.data.iter() {
            if *asteroid_position == from_position {
                continue;
            }
            let diff = *asteroid_position - from_position;
            let minimum_vec = diff / gcd(diff.0, diff.1);

            normalized_found.insert(minimum_vec);
        }

        normalized_found.len() as u32
    }

    // returns (pos, count)
    fn best_position(&self) -> (Position, u32) {
        let mut best = 0;
        let mut best_position = Position(0, 0);
        // for each position, calculate ammount of asteroids it can reach
        for asteroid_position in self.data.iter() {
            let count = self.count_reach(*asteroid_position);
            if count > best {
                best = count;
                best_position = *asteroid_position;
            }
        }

        (best_position, best)
    }

    // vaporizes n asteroids around base position
    // returns last asteroid vaporized
    fn vaporize(&self, base: Position, n: usize) -> Position {
        let mut list = Vec::new();
        for asteroid in self.data.iter() {
            if *asteroid == base {
                continue;
            }
            let diff = *asteroid - base;
            list.push((diff.angle(), diff.mag(), *asteroid));
        }

        list.sort_by(|a, b| {
            // sort by reverse angle (as we want to process angles in a clockwise order)
            // then magnitude if same angle
            // partial_cmp is safe here as atan2 can't return nan/infinity
            a.0.partial_cmp(&b.0).unwrap().reverse().then(a.1.cmp(&b.1))
        });

        let mut removed = Vec::with_capacity(n);
        // find first occurence of lower than PI/2, otherwise use first element
        for (idx, v) in list.iter().enumerate() {
            if v.0 <= FRAC_PI_2 {
                removed.push(list.remove(idx));
                break;
            }
        }
        if let None = removed.last() {
            removed.push(list.remove(0));
        }

        'outer: for _ in 1..n {
            for (idx, v) in list.iter().enumerate() {
                if v.0 < removed.last().unwrap().0 {
                    removed.push(list.remove(idx));
                    continue 'outer;
                }
            }
            // reached end of list without removing, remove first element
            removed.push(list.remove(0));
        }
        removed.last().unwrap().2
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
// Position(x,y)
struct Position(i32, i32);

impl ops::Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Position) -> Self::Output {
        // because 0,0 is top left instead of bottom left,
        // reverse the y values. seems hacky: any better way to do this?
        Position(self.0 - rhs.0, rhs.1 - self.1)
    }
}

impl ops::Div<i32> for Position {
    type Output = Position;

    fn div(self, rhs: i32) -> Self::Output {
        Position(self.0 / rhs, self.1 / rhs)
    }
}

impl Position {
    fn angle(&self) -> f64 {
        // y.atan2(x)
        let result = (self.1 as f64).atan2(self.0 as f64);
        if result < 0.0 {
            result + 2.0 * PI
        } else {
            result
        }
    }

    fn mag(&self) -> i32 {
        self.0 * self.0 + self.1 * self.1
    }
}

// From https://doc.rust-lang.org/std/ops/trait.Div.html
// Euclid's two-thousand-year-old algorithm for finding the greatest common
// divisor.
fn gcd(x: i32, y: i32) -> i32 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    // added abs here as we need to always return positive numbers
    x.abs()
}

pub fn solve_first(input: &str) -> u32 {
    let asts = Asteroids::read_asteroids(input);
    asts.best_position().1
}

pub fn solve_second(input: &str) -> i32 {
    let asts = Asteroids::read_asteroids(input);
    let best = asts.best_position();
    let nth_position = asts.vaporize(best.0, 200);

    nth_position.0 * 100 + nth_position.1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_read() {
        let input = include_str!("example1");
        let asts = Asteroids::read_asteroids(input);
        assert!(asts.data.contains(&Position(8, 3)));
        assert!(asts.data.contains(&Position(8, 1)));
        assert!(asts.data.contains(&Position(8, 0)));
    }

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(12, 6), 6);
        assert_eq!(gcd(54, 24), 6);
        assert_eq!(gcd(54, -24), 6);
    }

    #[test]
    fn test_vaporize1() {
        let input = include_str!("example1");
        let asts = Asteroids::read_asteroids(input);

        assert_eq!(asts.vaporize(Position(8, 3), 36), Position(14, 3));
    }

    #[test]
    fn test_vaporize2() {
        let input = include_str!("example2");
        let asts = Asteroids::read_asteroids(input);

        assert_eq!(asts.vaporize(Position(11, 13), 200), Position(8, 02));
    }

    #[test]
    fn test_first() {
        let input = include_str!("input");
        assert_eq!(solve_first(input), 344);
    }

    #[test]
    fn test_second() {
        let input = include_str!("input");
        assert_eq!(solve_second(input), 2732);
    }
}
