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
 * abs - computes the absolute value of an integer
 * @n: the integer to compute
 *
 * Return: the absolute value
 */
fn abs(n: i32) -> i32 {
    if n < 0 {
        -n
    } else {
        n
    }
}

/**
 * main - Entry point
 *
 * Returns () nothing aka "void" in C.
 */
fn main() {
    let a = abs(-10);
    let b = abs(5);
    _putchar((a / 10 + b'0' as i32) as u8 as char);
    _putchar((a % 10 + b'0' as i32) as u8 as char);
    _putchar('\n');
    _putchar((b + b'0' as i32) as u8 as char);
    _putchar('\n');
}
