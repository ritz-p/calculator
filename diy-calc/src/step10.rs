use std::io::{stdin, Read};
const BUFFER_SIZE: usize = 256;

fn main() {
    let mut buffer = [0_u8; BUFFER_SIZE];

    let bytes_read = stdin().read(&mut buffer).expect("failed to read");

    let mut read_position = 0;

    let mut input = parse_input(&buffer, bytes_read, &mut read_position);
    if let Some(ref mut num) = input {
        *num = mul_and_div(*num, &buffer, bytes_read, &mut read_position);
    }
    println!("{}", input.unwrap());
}

fn adjustment(input: i32, buffer: &[u8], bytes_read: usize, read_position: &mut usize) -> i32 {
    let mut res = input;
    let binding = String::from_utf8_lossy(&buffer[*read_position..bytes_read]);
    let mut chars = binding.chars();
    while let Some(c) = chars.next() {
        match c {
            '+' => {
                *read_position += 1;
                if let Some(add) = parse_input(buffer, bytes_read, read_position) {
                    res += add;
                }
            }
            '-' => {
                *read_position += 1;
                if let Some(sub) = parse_input(buffer, bytes_read, read_position) {
                    res -= sub;
                }
            }
            _ => {}
        }
    }
    res
}

fn mul_and_div(input: i32, buffer: &[u8], bytes_read: usize, read_position: &mut usize) -> i32 {
    let mut res = input;
    let binding = String::from_utf8_lossy(&buffer[*read_position..bytes_read]);
    let mut chars = binding.chars();
    while let Some(c) = chars.next() {
        match c {
            '*' => {
                *read_position += 1;
                if let Some(mul) = parse_input(buffer, bytes_read, read_position) {
                    res *= mul;
                }
            }
            '/' => {
                *read_position += 1;
                if let Some(div) = parse_input(buffer, bytes_read, read_position) {
                    res /= div;
                }
            }
            _ => {}
        }
    }
    res
}

fn parse_input(buffer: &[u8], bytes_read: usize, read_position: &mut usize) -> Option<i32> {
    let input = String::from_utf8_lossy(&buffer[*read_position..bytes_read]);
    let mut chars = input.chars();
    let mut res = 0;
    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            if let Some(num) = c.to_digit(10) {
                res = res * 10 + num as i32;
            }
            *read_position += 1;
        } else {
            break;
        }
    }
    Some(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn string_to_fixed_buffer(s: &str) -> [u8; BUFFER_SIZE] {
        let mut buffer = [0_u8; BUFFER_SIZE];
        let bytes_read = s.as_bytes();
        let length = bytes_read.len().min(BUFFER_SIZE - 1);
        buffer[..length].copy_from_slice(&bytes_read[..length]);
        buffer
    }

    #[test]
    fn test_01() {
        let input = string_to_fixed_buffer("1");
        let mut read_position = 0;
        let mut res = parse_input(&input, 2, &mut read_position);
        if let Some(ref mut num) = res {
            *num = mul_and_div(*num, &input, 2, &mut read_position);
        }
        assert_eq!(res, Some(1));
    }

    #[test]
    fn test_02() {
        let input = string_to_fixed_buffer("42");
        let mut read_position = 0;
        let mut res = parse_input(&input, 4, &mut read_position);
        if let Some(ref mut num) = res {
            *num = mul_and_div(*num, &input, 4, &mut read_position);
        }
        assert_eq!(res, Some(42));
    }

    #[test]
    fn test_03() {
        let input = string_to_fixed_buffer("1*2");
        let mut read_position = 0;
        let mut res = parse_input(&input, 6, &mut read_position);
        if let Some(ref mut num) = res {
            *num = mul_and_div(*num, &input, 6, &mut read_position);
        }
        assert_eq!(res, Some(2));
    }

    #[test]
    fn test_04() {
        let input = string_to_fixed_buffer("4/2");
        let mut read_position = 0;
        let mut res = parse_input(&input, 6, &mut read_position);
        if let Some(ref mut num) = res {
            *num = mul_and_div(*num, &input, 6, &mut read_position);
        }
        assert_eq!(res, Some(2));
    }

    #[test]
    fn test_05() {
        let input = string_to_fixed_buffer("2*3/5");
        let mut read_position = 0;
        let mut res = parse_input(&input, 10, &mut read_position);
        if let Some(ref mut num) = res {
            *num = mul_and_div(*num, &input, 10, &mut read_position);
        }
        assert_eq!(res, Some(1));
    }

    #[test]
    fn test_06() {
        let input = string_to_fixed_buffer("10/2*3");
        let mut read_position = 0;
        let mut res = parse_input(&input, 10, &mut read_position);
        if let Some(ref mut num) = res {
            *num = mul_and_div(*num, &input, 10, &mut read_position);
        }
        assert_eq!(res, Some(15));
    }

    #[test]
    fn test_07() {
        let input = string_to_fixed_buffer("12*34/56");
        let mut read_position = 0;
        let mut res = parse_input(&input, 10, &mut read_position);
        if let Some(ref mut num) = res {
            *num = mul_and_div(*num, &input, 10, &mut read_position);
        }
        assert_eq!(res, Some(7));
    }
}
