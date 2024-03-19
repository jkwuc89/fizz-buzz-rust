const FIZZ: &str = "Fizz";
const BUZZ: &str = "Buzz";

fn main() {
    for number in 1..=100 {
        println!("{}", fizz_buzz_using_if_then_else(number));
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_fizz_buzz_using_if_then_else() {
        assert_eq!(fizz_buzz_using_if_then_else(1), "1");

        assert_eq!(fizz_buzz_using_if_then_else(3), FIZZ);
        assert_eq!(fizz_buzz_using_if_then_else(6), FIZZ);
        assert_eq!(fizz_buzz_using_if_then_else(9), FIZZ);
        assert_eq!(fizz_buzz_using_if_then_else(12), FIZZ);

        assert_eq!(fizz_buzz_using_if_then_else(5), BUZZ);
        assert_eq!(fizz_buzz_using_if_then_else(10), BUZZ);
        assert_eq!(fizz_buzz_using_if_then_else(20), BUZZ);
        assert_eq!(fizz_buzz_using_if_then_else(25), BUZZ);

        assert_eq!(fizz_buzz_using_if_then_else(15), format!("{0}{1}", FIZZ, BUZZ));
        assert_eq!(fizz_buzz_using_if_then_else(30), format!("{0}{1}", FIZZ, BUZZ));
        assert_eq!(fizz_buzz_using_if_then_else(45), format!("{0}{1}", FIZZ, BUZZ));
    }
}
