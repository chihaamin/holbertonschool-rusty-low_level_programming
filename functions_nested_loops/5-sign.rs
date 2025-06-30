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
 * print_sign - prints the sign of a number
 * @n: the number to check
 *
 * Return: 1 if positive, 0 if zero, -1 if negative
 */
fn print_sign(n: i32) -> i32 {
    if n > 0 {
        _putchar('+');
        1
    } else if n == 0 {
        _putchar('0');
        0
    } else {
        _putchar('-');
        -1
    }
}

/**
 * main - Entry point
 *
 * Returns () nothing aka "void" in C.
 */
fn main() {
    print_sign(10);
    print_sign(0);
    print_sign(-5);
    _putchar('\n');
}
