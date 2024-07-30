use std::{io::{stdin, Read}, str::from_utf8};
const BUFFER_SIZE: usize = 256;

fn main() {
    let mut buffer = [0_u8; BUFFER_SIZE];

    let bytes_read = stdin().read(&mut buffer).expect("failed to read");

    let mut read_position = 0;
    let res = adjustment(0, &buffer, bytes_read, &mut read_position);
    println!("{}", res);
}

fn adjustment(input: i32, buffer: &[u8], bytes_read: usize, read_position: &mut usize) -> i32 {
    let mut res = if let Some(num) = parse_input(buffer, bytes_read, read_position) {
        mul_and_div(num, buffer, bytes_read, read_position)
    } else {
        input
    };
    let binding = String::from_utf8_lossy(&buffer[*read_position..bytes_read]);
    let mut chars = binding.chars();
    while let Some(c) = chars.next() {
        match c {
            '+' => {
                *read_position += 1;
                if let Some(add) = parse_input(buffer, bytes_read, read_position){
                    res += mul_and_div(add, buffer, bytes_read, read_position);
                }
            }
            '-' => {
                *read_position += 1;
                if let Some(sub) = parse_input(buffer, bytes_read, read_position){
                    res -= mul_and_div(sub, buffer, bytes_read, read_position);
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
            },
            '+' | '-' => {
                break;
            }
            _ => {}
        }
    }
    res
}

fn parse_input(buffer: &[u8], bytes_read: usize, read_position: &mut usize) -> Option<i32> {
    let mut sign = 1;
    let mut start = *read_position;

    if buffer[start] == b'+' {
        *read_position += 1;
        start += 1;
    } else if buffer[start] == b'-' {
        sign = -1;
        *read_position += 1;
        start += 1;
    }

    let end = buffer[start..bytes_read].iter()
        .position(|&c| !c.is_ascii_digit())
        .map_or(bytes_read, |pos| start + pos);
    let number_str = &buffer[start..end];
    let number_str = from_utf8(number_str).ok()?;
    *read_position = end;
    
    number_str.parse::<i32>().ok().map(|num| num * sign)
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
        let buffer = string_to_fixed_buffer("42");
        let mut read_position = 0;
        let res = adjustment(0, &buffer, 4, &mut read_position);
        assert_eq!(res, 42);
    }

    #[test]
    fn test_02() {
        let buffer = string_to_fixed_buffer("4*10+2");
        let mut read_position = 0;
        let res = adjustment(0, &buffer, 10, &mut read_position);
        assert_eq!(res, 42);
    }

    #[test]
    fn test_03() {
        let buffer = string_to_fixed_buffer("10*-4-2");
        let mut read_position = 0;
        let res = adjustment(0, &buffer, 14, &mut read_position);
        assert_eq!(res, -42);
    }

    #[test]
    fn test_04() {
        let buffer = string_to_fixed_buffer("40--2");
        let mut read_position = 0;
        let res = adjustment(0, &buffer, 12, &mut read_position);
        assert_eq!(res, 42);
    }

    #[test]
    fn test_05() {
        let buffer = string_to_fixed_buffer("+40--2");
        let mut read_position = 0;
        let res = adjustment(0, &buffer, 12, &mut read_position);
        assert_eq!(res, 42);
    }

}
