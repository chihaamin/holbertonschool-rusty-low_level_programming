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
 * islower - checks for lowercase character
 * @c: the character to check
 *
 * Return: 1 if c is lowercase, 0 otherwise
 */
fn islower(c: char) -> i32 {
    if c >= 'a' && c <= 'z' {
        1
    } else {
        0
    }
}

/**
 * main - Entry point
 *
 * Returns () nothing aka "void" in C.
 */
fn main() {
    let test = 'a';
    let res = islower(test);
    _putchar((res as u8 + b'0') as char);
    _putchar('\n');
}
