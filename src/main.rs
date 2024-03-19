const FIZZ: &str = "Fizz";
const BUZZ: &str = "Buzz";

fn main() {
    for number in 1..=100 {
        println!("{}", fizz_buzz_using_if_then_else(number));
    }

    for number in 1..=100 {
        println!("{}", fizz_buzz_using_match(number));
    }
}

fn fizz_buzz_using_if_then_else(number: i32) -> String {
    return if number % 15 == 0 {
        format!("{0}{1}", FIZZ, BUZZ)
    } else if number % 3 == 0 {
        format!("{}", FIZZ)
    } else if number % 5 == 0 {
        format!("{}", BUZZ)
    } else {
        format!("{}", number)
    }
}

fn fizz_buzz_using_match(number: i32) -> String {
    return match number {
        number if number % 15 == 0 => format!("{0}{1}", FIZZ, BUZZ),
        number if number % 3 == 0 => format!("{}", FIZZ),
        number if number % 5 == 0 => format!("{}", BUZZ),
        _ => format!("{}", number)
    }
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

        assert_eq!(fizz_buzz_fn(15), format!("{0}{1}", FIZZ, BUZZ));
        assert_eq!(fizz_buzz_fn(30), format!("{0}{1}", FIZZ, BUZZ));
        assert_eq!(fizz_buzz_fn(45), format!("{0}{1}", FIZZ, BUZZ));
    }

    #[test]
    fn test_fizz_buzz_using_if_then_else() {
        verify_fizz_buzz_function(fizz_buzz_using_if_then_else);
    }

    #[test]
    fn test_fizz_buzz_using_match() {
        verify_fizz_buzz_function(fizz_buzz_using_match);
    }
}
