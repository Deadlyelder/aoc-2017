use std::env;
use std::io::{self, Write};
use std::process;

pub fn parse(input: &str, offset: usize) -> Option<u32> {
    let input_len = input.len();
    // Check if the string is not empty
    if input_len == 0 || offset >= input_len {
        return None;
    }
    // Contains the "list" of numbers
    let vector_of_number = input.chars().map(|x| x.to_digit(10).expect(&format!("Character {} can be parsed as u32 type", x))).collect::<Vec<u32>>();
    let mut sum_vector = 0;
    for (index, number) in vector_of_number.iter().enumerate() {
        let compare_number = vector_of_number[(index + offset) % input_len];
        if *number == compare_number {
            sum_vector += *number;
        }
    }
    Some(sum_vector)
}

fn main() {

    // Get the arguments
    let args = env::args();

    // If the length of the list is greater/lower than 2, exit
    if args.len() != 2 {
        writeln!(io::stdout(), "Please to provide only one argument: the number to parse!").unwrap();
        process::exit(1);
    }

    let input = args.last().unwrap();
    let input = input.as_str();
    let offset: usize = input.len() / 2;

    // Return the result
    match parse(input, offset) {
        Some(value) => println!("{}", value),
        None => process::exit(1),
    }

}

#[cfg(test)]

mod test {

    use super::parse;

    #[test]
    fn test_wrong_input_1() {
        assert_eq!(None, parse("", 1));
    }

    #[test]
    #[should_panic]
    fn test_wrong_input_2() {
        assert_eq!(None, parse("1234h", 1));
    }

    #[test]
    #[should_panic]
    fn test_wrong_input_3() {
        assert_eq!(None, parse(" 1234  2", 1));
    }

    #[test]
    #[should_panic]
    fn test_wrong_input_4() {
        assert_eq!(None, parse("1helloworld0", 1));
    }

    #[test]
    fn test_1122() {
        assert_eq!(Some(3), parse("1122", 1));
        assert_eq!(Some(0), parse("1122", 2));
    }

    #[test]
    fn test_1212() {
        assert_eq!(Some(0), parse("1212", 1));
        assert_eq!(Some(6), parse("1212", 2));
    }

    #[test]
    fn test_1221() {
        assert_eq!(Some(3), parse("1221", 1));
        assert_eq!(Some(0), parse("1221", 2));
    }

    #[test]
    fn test_1111() {
        assert_eq!(Some(4), parse("1111", 1));
        assert_eq!(Some(4), parse("1111", 2));
    }

    #[test]
    fn test_1234() {
        assert_eq!(Some(0), parse("1234", 1));
        assert_eq!(Some(0), parse("1234", 2));
    }

    #[test]
    fn test_123425() {
        assert_eq!(Some(0), parse("123425", 1));
        assert_eq!(Some(4), parse("123425", 3));
    }

    #[test]
    fn test_123123() {
        assert_eq!(Some(0), parse("123123", 1));
        assert_eq!(Some(12), parse("123123", 3));
    }

    #[test]
    fn test_12131415() {
        assert_eq!(Some(0), parse("12131415", 1));
        assert_eq!(Some(4), parse("12131415", 4));
    }

    #[test]
    fn test_91212129() {
        assert_eq!(Some(9), parse("91212129", 1));
        assert_eq!(Some(6), parse("91212129", 4));
    }

}