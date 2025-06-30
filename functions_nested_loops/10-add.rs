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
 * add - adds two integers and returns the result
 * @a: first integer
 * @b: second integer
 *
 * Return: the sum
 */
fn add(a: i32, b: i32) -> i32 {
    a + b
}

/**
 * main - Entry point
 *
 * Returns () nothing aka "void" in C.
 */
fn main() {
    let sum = add(3, 5);
    if sum >= 10 {
        _putchar(((sum / 10) as u8 + b'0') as char);
    }
    _putchar(((sum % 10) as u8 + b'0') as char);
    _putchar('\n');
}
