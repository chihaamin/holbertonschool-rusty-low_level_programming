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
//void print_triangle(int size)
/**
 * print_triangle - prints a triangle of size 'size'
 * @size: the size of the triangle to print
 * This function prints a right-angled triangle made of '#' characters.
 * If size is less than or equal to 0, it prints nothing.
 * Returns () nothing aka "void" in C.
 */
fn print_triangle(size: i32) -> () {
    if size <= 0 {
        return; // Do nothing if size is less than or equal to 0
    }
    for i in 1..=size {
        for _ in 0..(size - i) {
            _putchar(' '); // Print spaces for right alignment
        }
        for _ in 0..i {
            _putchar('#'); // Print '#' character
        }
        _putchar('\n'); // Print newline after each row
    }
}
/** * main - Entry point of the program
 * This function calls print_triangle to display a triangle of
 * the specified size.
 * Returns () nothing aka "void" in C.
 */
fn main() -> () {
    print_triangle(2);
    print_triangle(10);
    print_triangle(1);
    print_triangle(0);
}
