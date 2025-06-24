use std::io::{self, Write};
/*
main - Entry point of the program
This function writes a specific string to the standard output without using println macro.
Returns: ()
 */
fn main() -> () {
    match io::stdout().write_all(b"\"Programming is like building a multilingual puzzle\n") {
        Ok(_) => (),
        Err(e) => {
            eprintln!("Failed to write to stdout: {}", e);
        }
    }
}
