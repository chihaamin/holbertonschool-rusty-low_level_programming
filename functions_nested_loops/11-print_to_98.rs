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
 * print_to_98 - prints all natural numbers from n to 98
 * @n: starting number
 */
fn print_to_98(n: i32) {
    if n <= 98 {
        let mut i = n;
        while i <= 98 {
            if i < 0 {
                _putchar('-');
                let mut num = -i;
                if num >= 10 {
                    _putchar(((num / 10) as u8 + b'0') as char);
                }
                _putchar(((num % 10) as u8 + b'0') as char);
            } else {
                if i >= 10 {
                    _putchar(((i / 10) as u8 + b'0') as char);
                }
                _putchar(((i % 10) as u8 + b'0') as char);
            }
            if i != 98 {
                _putchar(',');
                _putchar(' ');
            } else {
                _putchar('\n');
            }
            i += 1;
        }
    } else {
        let mut i = n;
        while i >= 98 {
            if i < 0 {
                _putchar('-');
                let mut num = -i;
                if num >= 10 {
                    _putchar(((num / 10) as u8 + b'0') as char);
                }
                _putchar(((num % 10) as u8 + b'0') as char);
            } else {
                if i >= 10 {
                    _putchar(((i / 10) as u8 + b'0') as char);
                }
                _putchar(((i % 10) as u8 + b'0') as char);
            }
            if i != 98 {
                _putchar(',');
                _putchar(' ');
            } else {
                _putchar('\n');
            }
            i -= 1;
        }
    }
}

/**
 * main - Entry point
 *
 * Returns () nothing aka "void" in C.
 */
fn main() {
    print_to_98(90);
    print_to_98(111);
}
