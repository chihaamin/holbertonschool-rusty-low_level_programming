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
//void print_diagonal(int n);
/**
 * print_diagonal - prints a diagonal line of backslashes
 * @n: the number of backslashes to print
 * This function prints a diagonal line of backslashes ('\') followed by a newline.
 * If n is less than or equal to 0, it prints a newline only.
 * Returns () nothing aka "void" in C.
 */
fn print_diagonal(n: i32) -> () {
    if n <= 0 {
        _putchar('\n'); // Print newline if n is less than or equal to 0
    } else {
        for i in 0..n {
            for _ in 0..i {
                _putchar(' '); // Print spaces for indentation
            }
            _putchar('\\'); // Print backslash
            _putchar('\n'); // Print newline after each backslash
        }
    }
}
/** * main - Entry point of the program
 * This function calls print_diagonal to display a diagonal
 * line of backslashes based on the input value.
 * Returns () nothing aka "void" in C.
 */
fn main() -> () {
    print_diagonal(0);
    print_diagonal(2);
    print_diagonal(10);
    print_diagonal(-4);
}
