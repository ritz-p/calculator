use std::io::{stdin, Read};
const BUFFER_SIZE: usize = 256;

fn main() {
    let mut buffer = [0_u8; BUFFER_SIZE];

    let bytes_read = stdin().read(&mut buffer).expect("failed to read");

    let mut read_position = 0;

    let mut input = parse_input(&buffer, bytes_read,&mut read_position);
    if let Some(ref mut num) = input{
        *num = calc(*num,&buffer,bytes_read,&mut read_position);
    }
    println!("{}",input.unwrap());
}

fn calc(input: u32,buffer: &[u8], bytes_read: usize,read_position: &mut usize) -> u32{
    let mut res = input;
    if let Some(c) = String::from_utf8_lossy(&buffer[*read_position..bytes_read]).chars().next(){
        match c{
            '+' => {
                *read_position += 1;
                if let Some(add) = parse_input(buffer, bytes_read, read_position){
                    res += add;
                }
            },
            _ => {}
        }
    }
    res
}

fn parse_input(buffer: &[u8], bytes_read: usize,read_position: &mut usize) -> Option<u32> {
    let input = String::from_utf8_lossy(&buffer[*read_position..bytes_read]);
    let mut chars = input.chars();
    let mut res = 0;
    while let Some(c) = chars.next() {
        if c.is_digit(10) {
            if let Some(num) = c.to_digit(10) {
                res = res * 10 + num;
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
        let mut res = parse_input(&input, 6,&mut read_position);
        if let Some(ref mut num) = res{
            *num = calc(*num,&input,6,&mut read_position);
        }
        assert_eq!(res, Some(1));
    }

    #[test]
    fn test_02() {
        let input = string_to_fixed_buffer("1+2");
        let mut read_position = 0;
        let mut res = parse_input(&input, 6,&mut read_position);
        if let Some(ref mut num) = res{
            *num = calc(*num,&input,6,&mut read_position);
        }
        assert_eq!(res, Some(3));
    }

    #[test]
    fn test_03() {
        let input = string_to_fixed_buffer("123+456");
        let mut read_position = 0;
        let mut res = parse_input(&input, 14,&mut read_position);
        if let Some(ref mut num) = res{
            *num = calc(*num,&input,14,&mut read_position);
        }
        assert_eq!(res, Some(579));
    }

    #[test]
    fn test_04() {
        let input = string_to_fixed_buffer("579");
        let mut read_position = 0;
        let mut res = parse_input(&input, 6,&mut read_position);
        if let Some(ref mut num) = res{
            *num = calc(*num,&input,6,&mut read_position);
        }
        assert_eq!(res, Some(579));
    }

    #[test]
    fn test_05() {
        let input = string_to_fixed_buffer("1+2+3");
        let mut read_position = 0;
        let mut res = parse_input(&input, 10,&mut read_position);
        if let Some(ref mut num) = res{
            *num = calc(*num,&input,10,&mut read_position);
        }
        assert_eq!(res, Some(3));
    }
}
