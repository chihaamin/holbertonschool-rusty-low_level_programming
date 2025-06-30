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
 * isalpha - checks for alphabetic character
 * @c: the character to check
 *
 * Return: 1 if c is alphabetic, 0 otherwise
 */
fn isalpha(c: char) -> i32 {
    if (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') {
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
    let test = 'A';
    let res = isalpha(test);
    _putchar((res as u8 + b'0') as char);
    _putchar('\n');
}
