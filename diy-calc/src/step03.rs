use std::io::{stdin, Read};
const BUFFER_SIZE: usize = 256;
fn main() {
    let mut buf = [0_u8; BUFFER_SIZE];

    let bytes_read = stdin().read(&mut buf).expect("failed to read");

    let input = parse_input(&buf, bytes_read);

    if let Some(num) = input {
        println!("{}", num);
    }
}

fn parse_input(buffer: &[u8], bytes: usize) -> Option<u32> {
    let input = String::from_utf8_lossy(&buffer[..bytes]);
    let mut chars = input.chars();
    let mut res = 0;
    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            if let Some(num) = c.to_digit(10) {
                res = res * 10 + num;
            }
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
        let bytes = s.as_bytes();
        let length = bytes.len().min(BUFFER_SIZE - 1);
        buffer[..length].copy_from_slice(&bytes[..length]);
        buffer
    }

    #[test]
    fn test_01() {
        let input = string_to_fixed_buffer("1");
        let res = parse_input(&input, 2);
        assert_eq!(res.unwrap().to_string(), "1");
    }

    #[test]
    fn test_02() {
        let input = string_to_fixed_buffer("42");
        let res = parse_input(&input, 4);
        assert_eq!(res.unwrap().to_string(), "42");
    }

    #[test]
    fn test_03() {
        let input = string_to_fixed_buffer("123");
        let res = parse_input(&input, 6);
        assert_eq!(res.unwrap().to_string(), "123");
    }

    #[test]
    fn test_04() {
        let input = string_to_fixed_buffer("12+34");
        let res = parse_input(&input, 10);
        assert_eq!(res.unwrap().to_string(), "12");
    }
}
