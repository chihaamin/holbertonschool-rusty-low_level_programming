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
 * print_numbers - prints the numbers from 0 to 9
 * This function prints each number followed by a newline character.
 * Returns () nothing aka "void" in C.
 */
fn print_numbers() -> () {
    for i in 0..10 {
        _putchar((i + '0' as i32) as u8 as char);
        _putchar('\n');
    }
}
/** * main - Entry point of the program
 * This function calls print_numbers to display the numbers from 0 to 9.
 * Returns () nothing aka "void" in C.
 */
fn main() -> () {
    print_numbers(); // Call the function to print numbers
}
