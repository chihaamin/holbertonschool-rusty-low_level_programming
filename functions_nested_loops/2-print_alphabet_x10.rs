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
 * print_alphabet_x10 - prints the alphabet 10 times in lowercase
 */
fn print_alphabet_x10() {
    let mut i = 0;
    while i < 10 {
        let mut c = b'a';
        while c <= b'z' {
            _putchar(c as char);
            c += 1;
        }
        _putchar('\n');
        i += 1;
    }
}

/**
 * main - Entry point
 *
 * Returns () nothing aka "void" in C.
 */
fn main() {
    print_alphabet_x10();
}
