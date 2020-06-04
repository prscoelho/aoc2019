pub fn read_input(input: &str) -> u32 {
    let mut result = 0;
    let mut idx = 0;

    for line in input.lines() {
        for c in line.chars() {
            if c == '#' {
                result += 1 << idx;
            }
            idx += 1;
        }
    }
    result
}

pub fn step(state: u32) -> u32 {
    let mut result = 0;

    for idx in 0..25 {
        let infested = state >> idx & 1 == 1;
        let n = neighbours(state, idx);
        match (infested, n) {
            (true, 1) | (false, 1) | (false, 2) => {
                // tile is infested on next step
                result += 1 << idx;
            }
            _ => {}
        }
    }
    result
}

fn neighbours(state: u32, idx: u32) -> u32 {
    let mut result = 0;
    if !(0..5).any(|i| i * 5 == idx) {
        // not left edge, so we can add left neighbour
        result += state << 1 >> idx & 1;
    }

    if !(0..5).any(|i| i * 5 + 4 == idx) {
        // not right edge, so we can add right neighbour
        result += state >> 1 >> idx & 1;
    }

    // these two next neighbour checks don't require bound checking
    // as they'll result in 0 if it's out of bounds

    // 5 tiles above
    result += state << 5 >> idx & 1;
    // 5 tiles below
    result += state >> idx + 5 & 1;

    result
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn input_simple() {
        let input = ".";
        assert_eq!(read_input(input), 0);

        let input2 = "#";
        assert_eq!(read_input(input2), 1);

        let input3 = "##";
        assert_eq!(read_input(input3), 3);

        let input4 = "#.#";
        assert_eq!(read_input(input4), 5);
    }
    #[test]
    fn input() {
        let input = "#...#\n....#";
        assert_eq!(read_input(input), 1 + 16 + 512);
    }

    #[test]
    fn neighbours_simple() {
        // "#.#"
        let state = 5;
        assert_eq!(neighbours(state, 1), 2);
        assert_eq!(neighbours(state, 0), 0);
        assert_eq!(neighbours(state, 2), 0);
    }

    #[test]
    fn neighbours_normal() {
        let input = include_str!("example1");
        let state = read_input(input);

        assert_eq!(neighbours(state, 0), 1);
        assert_eq!(neighbours(state, 9), 3);
    }

    #[test]
    fn state_is_biodiversity() {
        let input = ".....\n.....\n.....\n#....\n.#...";
        let state = read_input(input);
        assert_eq!(state, 2129920);
    }
}
