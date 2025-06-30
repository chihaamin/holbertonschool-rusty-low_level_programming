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
//print_square(int size);
/**
 * print_square - prints a square of size 'size'
 * @size: the size of the square to print
 * This function prints a square made of '#' characters.
 * If size is less than or equal to 0, it prints nothing.
 * Returns () nothing aka "void" in C.
 */
fn print_square(size: i32) -> () {
    if size <= 0 {
        return; // Do nothing if size is less than or equal to 0
    }
    for _ in 0..size {
        for _ in 0..size {
            _putchar('#'); // Print '#' character
        }
        _putchar('\n'); // Print newline after each row
    }
}
/** * main - Entry point of the program
 * This function calls print_square to display a square of
 * the specified size.
 * Returns () nothing aka "void" in C.
 */
fn main() -> () {
    print_square(2);
    print_square(10);
    print_square(0);
}
