fn main() {
    for number in 1..=100 {
        fizz_buzz_using_if_then_else(number)
    }
}

fn fizz_buzz_using_if_then_else(number: i32) {
    if number % 15 == 0 {
        println!("fizzbuzz");
    } else if number % 3 == 0 {
        println!("fizz");
    } else if number % 5 == 0 {
        println!("buzz");
    } else {
        println!("{}", number);
    }
}
