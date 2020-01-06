fn read_ints() -> Vec<i32> {
    let mut input = String::new();
    let mut result = Vec::new();
    while let Ok(n) = std::io::stdin().read_line(&mut input) {
        if n == 0 {
            break;
        }
        result.push(input.trim().parse::<i32>().unwrap());
        input.clear();
    }
    result
}

fn fuel_required(mass: i32) -> i32 {
    mass / 3 - 2
}

pub fn solve_first() -> i32 {
    let masses = read_ints();
    let mut result = 0;
    for mass in masses {
        result += fuel_required(mass);
    }
    result
}

pub fn solve_second() -> i32 {
    let masses = read_ints();
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
