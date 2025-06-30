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
 * print_last_digit - prints the last digit of a number
 * @n: the number to process
 *
 * Return: the last digit
 */
fn print_last_digit(n: i32) -> i32 {
    let last = (n.abs() % 10) as i32;
    _putchar((last as u8 + b'0') as char);
    last
}

/**
 * main - Entry point
 *
 * Returns () nothing aka "void" in C.
 */
fn main() {
    print_last_digit(1234);
    print_last_digit(-567);
    _putchar('\n');
}
