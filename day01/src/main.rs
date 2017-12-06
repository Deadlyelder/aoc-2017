use std::env;
use std::io::{self, Write};
use std::process;

trait CharConversion {
    fn parse(&self) -> Option<u32>;
}

impl CharConversion for char {
    fn parse(&self) -> Option<u32> {
        self.to_digit(10)
    }
}

pub fn parse(input: &str) -> Option<u32> {
    // Check if the string is not empty
    if input.len() == 0 {
        return None
    }
    // Contains the "list" of numbers
    let mut vector_of_number: Vec<u32> = Vec::new();
    // Take the last character of the string reference, and add it to the vector of numbers
    let mut last_character = match input.chars().last().unwrap().parse() {
        Some(value) => value,
        // If the last character can't be parse to u32 type, exit
        None => {
            writeln!(io::stdout(), "Error when parsing the last character of the string").unwrap();
            return None
        } 
    };
    // Convert each character to number
    for (index, character) in input.chars().enumerate() {
        match character.parse() {
            Some(value) => {
                // If the new character is the same than the last one, add it to the vector of numbers
                if value == last_character {
                    vector_of_number.push(value);
                }
                last_character = value;
            }
            // If the character is not a number, exit
            None => {
                writeln!(io::stdout(), "Error when parsing the string: canno't convert char to usize at position {} for char '{}", index, character).unwrap();
                return None
            }
        }
    }
    // Return the sum
    Some(vector_of_number.iter().sum())
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

    // Return the result
    match parse(input.as_str()) {
        Some(value) => println!("{}", value),
        None => process::exit(1),
    }

}

#[cfg(test)]

mod test {

    use super::parse;

    #[test]
    fn test_wrong_input_1() {
        assert_eq!(None, parse(""));
    }

    #[test]
    fn test_wrong_input_2() {
        assert_eq!(None, parse("1234h"));
    }

    #[test]
    fn test_wrong_input_3() {
        assert_eq!(None, parse(" 1234  2"));
    }

    #[test]
    fn test_wrong_input_4() {
        assert_eq!(None, parse("1helloworld0"));
    }

    #[test]
    fn test_1122() {
        assert_eq!(Some(3), parse("1122"));
    }

    #[test]
    fn test_1111() {
        assert_eq!(Some(4), parse("1111"));
    }

    #[test]
    fn test_1234() {
        assert_eq!(Some(0), parse("1234"));
    }

    #[test]
    fn test_91212129() {
        assert_eq!(Some(9), parse("91212129"));
    }

}