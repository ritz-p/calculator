use std::io::stdin;
fn main() {
    let mut buf = String::new();

    let _ = stdin().read_line(&mut buf);

    let trimmed = buf.trim();
    match parse_input(trimmed) {
        Ok(num) => {
            println!("{}", num);
        }
        Err(_) => {}
    }
}

fn parse_input(input: &str) -> Result<i32, std::num::ParseIntError> {
    input.trim().parse::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_01() {
        let input = "1\n";
        let result = parse_input(input);
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn test_02() {
        let input = "42\n";
        let result = parse_input(input);
        assert_eq!(result.unwrap(), 42);
    }
}
