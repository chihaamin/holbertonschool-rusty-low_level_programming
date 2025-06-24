use std::io::{self, Write};
/**
 * putchar - prints a character to standard output
 * @c: the character to print
 * This function writes a single character to the standard output
 * and flushes the output buffer to ensure it appears immediately.
 * Returns () nothing aka "void" in C.
 */
fn putchar(c: char) -> () {
    let mut stdout = io::stdout();
    stdout.write(&[c as u8]).unwrap();
    stdout.flush().unwrap();
}

/**
 * main - entry point of the program
 * This function prints the numbers from 0 to 9
 * using the putchar function to output each number,
 * followed by a newline character.
 * Returns () nothing aka "void" in C.
 */
fn main() -> () {
    for i in 0..=9 {
        putchar(char::from_digit(i, 10).unwrap());
    }
    putchar('\n');
}
