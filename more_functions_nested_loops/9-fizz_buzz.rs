use std::io::{self, Write};
/**
 * _putchar - prints a character to standard output
 * @c: the character to print
 * This function writes a single character to the standard output
 * and flushes the output buffer to ensure it appears immediately.
 * Returns () nothing aka "void" in C.
 */
fn _putchar(c: char) -> () {
    let mut stdout = io::stdout();
    stdout.write(&[c as u8]).unwrap();
    stdout.flush().unwrap();
}
/**
 * FizzBuzz program in Rust
 * This program prints the numbers from 1 to 100, but for multiples of three,
 * it prints "Fizz" instead of the number, for multiples of five, it prints "Buzz",
 * and for numbers that are multiples of both three and five, it prints "FizzBuzz".
 * Returns () nothing aka "void" in C.
 */
fn fizz_buzz() -> () {
    const FIZZ: &str = "Fizz";
    const BUZZ: &str = "Buzz";
    const FIZZBUZZ: &str = "FizzBuzz";

    for i in 1..=100 {
        if i % 3 == 0 && i % 5 == 0 {
            for c in FIZZBUZZ.chars() {
                _putchar(c);
            }
        } else if i % 3 == 0 {
            for c in FIZZ.chars() {
                _putchar(c);
            }
        } else if i % 5 == 0 {
            for c in BUZZ.chars() {
                _putchar(c);
            }
        } else {
            // Print the number
            let num_str = i.to_string();
            for c in num_str.chars() {
                _putchar(c);
            }
        }
        _putchar(' '); // Print space after each output
    }
    _putchar('\n'); // Print newline at the end of each line
}
/**
 * main - Entry point of the program
 * This function calls fizz_buzz to display the FizzBuzz sequence.
 * Returns () nothing aka "void" in C.
 */
fn main() -> () {
    fizz_buzz(); // Call the function to print FizzBuzz sequence
}
