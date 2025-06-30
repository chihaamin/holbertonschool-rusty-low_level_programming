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
 * more_numbers - prints the numbers from 0 to 14, ten times
 * This function prints each number followed by a newline character.
 * It uses nested loops to achieve the desired output.
 * Returns () nothing aka "void" in C.
 */
fn more_numbers() -> () {
    for i in 0..10 {
        for j in 0..15 {
            // Print numbers from 0 to 14
            if j > 9 {
                _putchar((j / 10 + '0' as i32) as u8 as char); // Print tens place
            }
            _putchar((j % 10 + '0' as i32) as u8 as char); // Print units place
        }
        _putchar('\n'); // Print newline at the end
    }
}

/**
 * main - Entry point of the program
 * This function calls more_numbers to display numbers from 0 to 14,
 * ten times, each followed by a newline.
 * Returns () nothing aka "void" in C.
 */
fn main() -> () {
    more_numbers(); // Call the function to print numbers
}
