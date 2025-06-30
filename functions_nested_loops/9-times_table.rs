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
 * times_table - prints the 9 times table, starting with 0
 */
fn times_table() {
    let mut i = 0;
    while i <= 9 {
        let mut j = 0;
        while j <= 9 {
            let prod = i * j;
            if j == 0 {
                _putchar((prod as u8 + b'0') as char);
            } else {
                _putchar(',');
                _putchar(' ');
                if prod < 10 {
                    _putchar(' ');
                }
                if prod >= 10 {
                    _putchar(((prod / 10) as u8 + b'0') as char);
                }
                _putchar(((prod % 10) as u8 + b'0') as char);
            }
            j += 1;
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
    times_table();
}
