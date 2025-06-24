/**
 * main - entry point of the program
 * This function prints the numbers from 0 to 9
 * using the print! macro to output each number,
 * followed by a newline character.
 * Returns () nothing aka "void" in C.
 */
fn main() -> () {
    for i in 0..=9 {
        print!("{}", i);
    }
    print!("\n");
}
