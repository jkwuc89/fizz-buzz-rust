use std::env;

const DEFAULT_RANGE_END: i32 = 100;
const FIZZ: &str = "Fizz";
const BUZZ: &str = "Buzz";
const FIZZBUZZ: &str = "FizzBuzz";

fn main() {
    let range_end = get_range_end_from_command_line(env::args().collect());

    for number in 1..=range_end {
        println!("{}", fizz_buzz_using_if_then_else(number));
    }

    println!();

    for number in 1..=range_end {
        println!("{}", fizz_buzz_using_match(number));
    }
}

fn fizz_buzz_using_if_then_else(number: i32) -> String {
    return if number % 15 == 0 {
        FIZZBUZZ.to_string()
    } else if number % 3 == 0 {
        FIZZ.to_string()
    } else if number % 5 == 0 {
        BUZZ.to_string()
    } else {
        number.to_string()
    }
}

fn fizz_buzz_using_match(number: i32) -> String {
    return match number {
        number if number % 15 == 0 => FIZZBUZZ.to_string(),
        number if number % 3 == 0 => FIZZ.to_string(),
        number if number % 5 == 0 => BUZZ.to_string(),
        _ => number.to_string()
    }
}

fn get_range_end_from_command_line(args: Vec<String>) -> i32 {
    let mut range_end = DEFAULT_RANGE_END;
    if args.len() == 2 {
        match args[1].parse::<i32>() {
            Ok(parsed_number) => range_end = parsed_number,
            Err(error) => {
                println!("***ERROR*** Unable to convert '{}' to a number: {error}", args[1]);
                println!("Using default: {}", DEFAULT_RANGE_END);
            }
        }
    }

    return range_end;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn verify_fizz_buzz_function(fizz_buzz_fn: fn(i32) -> String) {
        assert_eq!(fizz_buzz_fn(1), "1");
        assert_eq!(fizz_buzz_fn(2), "2");
        assert_eq!(fizz_buzz_fn(4), "4");
        assert_eq!(fizz_buzz_fn(7), "7");
        assert_eq!(fizz_buzz_fn(8), "8");

        assert_eq!(fizz_buzz_fn(3), FIZZ);
        assert_eq!(fizz_buzz_fn(6), FIZZ);
        assert_eq!(fizz_buzz_fn(9), FIZZ);
        assert_eq!(fizz_buzz_fn(12), FIZZ);

        assert_eq!(fizz_buzz_fn(5), BUZZ);
        assert_eq!(fizz_buzz_fn(10), BUZZ);
        assert_eq!(fizz_buzz_fn(20), BUZZ);
        assert_eq!(fizz_buzz_fn(25), BUZZ);

        assert_eq!(fizz_buzz_fn(15), FIZZBUZZ);
        assert_eq!(fizz_buzz_fn(30), FIZZBUZZ);
        assert_eq!(fizz_buzz_fn(45), FIZZBUZZ);
    }

    #[test]
    fn test_fizz_buzz_using_if_then_else() {
        verify_fizz_buzz_function(fizz_buzz_using_if_then_else);
    }

    #[test]
    fn test_fizz_buzz_using_match() {
        verify_fizz_buzz_function(fizz_buzz_using_match);
    }

    #[test]
    fn test_get_range_end_from_command_line() {
        // Range end not specified uses default
        let mut args = vec!["executable".to_string()];
        assert_eq!(get_range_end_from_command_line(args), DEFAULT_RANGE_END);

        // Too many args provided uses default
        args = vec!["executable".to_string(), "10".to_string(), "20".to_string()];
        assert_eq!(get_range_end_from_command_line(args), DEFAULT_RANGE_END);

        // Valid (integer) range end provided returns provided range end
        args = vec!["executable".to_string(), "15".to_string()];
        assert_eq!(get_range_end_from_command_line(args), 15);

        // Invalid (non-integer chars) range end uses default
        args = vec!["executable".to_string(), "1A".to_string()];
        assert_eq!(get_range_end_from_command_line(args), DEFAULT_RANGE_END);
    }
}
