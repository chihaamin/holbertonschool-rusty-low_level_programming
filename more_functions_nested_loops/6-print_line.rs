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
//print_line(int n)
/**
 * print_line - prints a line of underscores
 * @n: the number of underscores to print
 * This function prints a line of underscores followed by a newline character.
 * Returns () nothing aka "void" in C.
 */
fn print_line(n: i32) -> () {
    if n <= 0 {
        _putchar('\n'); // Print newline if n is less than or equal to 0
    } else {
        for _ in 0..n {
            _putchar('_'); // Print underscores
        }
        _putchar('\n'); // Print newline at the end
    }
}
/** * main - Entry point of the program
 * This function calls print_line to display a line of underscores.
 * Returns () nothing aka "void" in C.
 */
fn main() -> () {
    print_line(0);
    print_line(2);
    print_line(10);
    print_line(-4);
}
