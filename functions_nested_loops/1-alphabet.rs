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
 * print_alphabet - prints the alphabet in lowercase followed by a new line
 */
fn print_alphabet() {
    let mut c = b'a';
    while c <= b'z' {
        _putchar(c as char);
        c += 1;
    }
    _putchar('\n');
}

/**
 * main - Entry point
 *
 * Returns () nothing aka "void" in C.
 */
fn main() {
    print_alphabet();
}
