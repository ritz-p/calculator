use std::io::{stdin, Read};
const BUFFER_SIZE:usize = 256;
fn main(){
    let mut buf = [0_u8;BUFFER_SIZE];

    let bytes_read = stdin().read(&mut buf).expect("failed to read");
    println!("{:?}",buf);

    let input = parse_input(&buf,bytes_read);
    
    if let Some(c) = input{
        println!("{}",c as i32 - '0' as i32);
    }
}

fn parse_input(buffer: &[u8],bytes: usize) -> Option<char>{
    let input = String::from_utf8_lossy(&buffer[..bytes]);
    input.chars().next()
}

#[cfg(test)]
mod tests{
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
        assert_eq!(res.unwrap().to_string(),"1");
    }

    #[test]
    fn test_02() {
        let input = string_to_fixed_buffer("42");
        let res = parse_input(&input, 4);
        assert_eq!(res.unwrap().to_string(),"4");
    }
}