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
 * print_24_hours - prints every minute of the day of Jack Bauer
 */
fn print_24_hours() {
    let mut h = 0;
    while h < 24 {
        let mut m = 0;
        while m < 60 {
            _putchar(((h / 10) as u8 + b'0') as char);
            _putchar(((h % 10) as u8 + b'0') as char);
            _putchar(':');
            _putchar(((m / 10) as u8 + b'0') as char);
            _putchar(((m % 10) as u8 + b'0') as char);
            _putchar('\n');
            m += 1;
        }
        h += 1;
    }
}

/**
 * main - Entry point
 *
 * Returns () nothing aka "void" in C.
 */
fn main() {
    print_24_hours();
}
