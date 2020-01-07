struct Image {
    data: Vec<u8>,
    width: usize,
    height: usize,
}

impl Image {
    fn new(input: &str, width: usize, height: usize) -> Image {
        let data: Vec<u8> = input
            .trim()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|n| n as u8)
            .collect();
        Image {
            data,
            width,
            height,
        }
    }

    fn layers(&self) -> std::slice::Chunks<u8> {
        self.data.chunks(self.width * self.height)
    }

    fn find_low_corrupt(&self) -> &[u8] {
        let mut min = usize::max_value();
        let mut result: &[u8] = &[];
        for layer in self.layers() {
            let count = layer.iter().filter(|n| **n == 0).count();
            if count < min {
                min = count;
                result = layer;
            }
        }
        result
    }

    fn solve(&self) -> i32 {
        let fewest = self.find_low_corrupt();
        let mut ones = 0;
        let mut twos = 0;
        for digit in fewest {
            match digit {
                1 => {
                    ones += 1;
                }
                2 => {
                    twos += 1;
                }
                _ => {}
            }
        }
        ones * twos
    }

    fn final_image(&self) -> Vec<u8> {
        let layer_size = self.width * self.height;
        let mut result = vec![2u8; layer_size];
        'outer: for (idx, val) in result.iter_mut().enumerate() {
            for pixel_layer in self.data.iter().skip(idx).step_by(layer_size) {
                if *pixel_layer != 2 {
                    *val = *pixel_layer;
                    continue 'outer;
                }
            }
        }
        result
    }

    fn print(&self) {
        let i = self.final_image();
        for line in i.chunks(self.width) {
            println!("{:?}", line);
        }
    }
}

pub fn solve_first(input: &str) -> i32 {
    let img = Image::new(input, 25, 6);
    img.solve()
}

pub fn solve_second(input: &str) {
    let img = Image::new(input, 25, 6);
    img.print();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let input = "123456789012";
        let image = Image::new(input, 3, 2);
        let mut layers = image.layers();

        assert_eq!(layers.next().unwrap(), &[1, 2, 3, 4, 5, 6]);
        assert_eq!(layers.next().unwrap(), &[7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_find() {
        let input = "103050789012";
        let image = Image::new(input, 3, 2);
        assert_eq!(image.find_low_corrupt(), &[7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_solve() {
        let input = "111202000000";
        let image = Image::new(input, 3, 2);
        assert_eq!(image.solve(), 6);
    }

    #[test]
    fn test_first() {
        let input = include_str!("input");
        assert_eq!(solve_first(input), 1560);
    }
}
