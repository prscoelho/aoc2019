use regex::Regex;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Vec3 { x, y, z }
    }
}
#[derive(Debug, Clone, Copy)]
struct Moon {
    position: Vec3,
    velocity: Vec3,
}

fn compare_axis(a: i32, b: i32) -> i32 {
    match a.cmp(&b) {
        Ordering::Less => 1,
        Ordering::Equal => 0,
        Ordering::Greater => -1,
    }
}

impl Moon {
    fn new(position: Vec3, velocity: Vec3) -> Self {
        Moon { position, velocity }
    }

    fn energy(&self) -> i32 {
        let pot = self.position.x.abs() + self.position.y.abs() + self.position.z.abs();
        let kin = self.velocity.x.abs() + self.velocity.y.abs() + self.velocity.z.abs();
        pot * kin
    }

    fn apply_gravity(&mut self, other: Moon) {
        self.velocity.x += compare_axis(self.position.x, other.position.x);
        self.velocity.y += compare_axis(self.position.y, other.position.y);
        self.velocity.z += compare_axis(self.position.z, other.position.z);
    }

    fn apply_velocity(&mut self) {
        self.position.x += self.velocity.x;
        self.position.y += self.velocity.y;
        self.position.z += self.velocity.z;
    }
}

fn read_moons(input: &str) -> Vec<Moon> {
    let mut result = Vec::with_capacity(4);
    // <x=(number), y=(number), z=(number)>
    let moon_regex = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();

    for cap in moon_regex.captures_iter(input) {
        let x = cap[1].parse().unwrap();
        let y = cap[2].parse().unwrap();
        let z = cap[3].parse().unwrap();

        result.push(Moon::new(Vec3::new(x, y, z), Vec3::new(0, 0, 0)));
    }
    result
}

fn step(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        for j in i + 1..moons.len() {
            // copying moons to appease the borrow checker
            let m1 = moons[i];
            let m2 = moons[j];
            moons[i].apply_gravity(m2);
            moons[j].apply_gravity(m1);
        }
    }

    for moon in moons.iter_mut() {
        moon.apply_velocity();
    }
}

fn count_energy(moons: &Vec<Moon>) -> i32 {
    moons.iter().map(|m| m.energy()).sum()
}

fn steps(moons: &mut Vec<Moon>, n: usize) {
    for _ in 0..n {
        step(moons);
    }
}

pub fn solve_first(input: &str) -> i32 {
    let mut moons = read_moons(input);
    steps(&mut moons, 1000);
    count_energy(&moons)
}

fn velocity_diff(positions: &[i32]) -> Vec<i32> {
    let mut result = vec![0; positions.len()];
    for (idx1, pos1) in positions.iter().enumerate() {
        for (idx2, pos2) in positions.iter().enumerate().skip(idx1 + 1) {
            result[idx1] += compare_axis(*pos1, *pos2);
            result[idx2] += compare_axis(*pos2, *pos1);
        }
    }
    result
}

fn find_steps_axis(mut positions: Vec<i32>) -> u64 {
    let mut velocities = vec![0; positions.len()];
    let velocities_end = velocities.clone();

    let mut steps = 0;
    loop {
        let velocity_change = velocity_diff(&positions);
        velocities = velocities
            .iter()
            .zip(velocity_change)
            .map(|(v, diff)| v + diff)
            .collect();
        positions = positions
            .iter()
            .zip(velocities.iter())
            .map(|(p, v)| p + v)
            .collect();

        steps += 1;

        if velocities == velocities_end {
            break;
        }
    }
    steps * 2
}

// From https://doc.rust-lang.org/std/ops/trait.Div.html
// Euclid's two-thousand-year-old algorithm for finding the greatest common
// divisor.
fn gcd(x: u64, y: u64) -> u64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn lcm3(a: u64, b: u64, c: u64) -> u64 {
    lcm(a, lcm(b, c))
}

pub fn solve_second(input: &str) -> u64 {
    let moons = read_moons(input);
    let x = find_steps_axis(moons.iter().map(|m| m.position.x).collect());
    let y = find_steps_axis(moons.iter().map(|m| m.position.y).collect());
    let z = find_steps_axis(moons.iter().map(|m| m.position.z).collect());
    lcm3(x, y, z)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_example1() {
        let input = include_str!("example1");
        let mut moons = read_moons(input);
        steps(&mut moons, 10);
        assert_eq!(count_energy(&moons), 179);
    }

    #[test]
    fn test_example2() {
        let input = include_str!("example2");
        let mut moons = read_moons(input);
        steps(&mut moons, 100);
        assert_eq!(count_energy(&moons), 1940);
    }

    #[test]
    fn test_first() {
        let input = include_str!("input");
        assert_eq!(solve_first(input), 6849);
    }

    #[test]
    fn test_second() {
        let input = include_str!("input");
        assert_eq!(solve_second(input), 356658899375688);
    }
}
