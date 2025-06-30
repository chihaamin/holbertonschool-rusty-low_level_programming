/*
* _isdigit - checks if a character is a digit
* @c: The character to check
* Return: 1 if c is a digit, 0 otherwise
 */
fn _isdigit(c: i32) -> i32 {
    if c >= '0' as i32 && c <= '9' as i32 {
        return 1;
    } else {
        return 0;
    }
}
/**
 * main - Entry point of the program
 * Return: () indicates successful execution
 */
fn main() -> () {
    let mut c: i32 = '0' as i32; // Example character to check
    print!("{}:  {}\n", c as u8 as char, _isdigit(c)); // Print the result
    c = 'a' as i32; // Change character to lowercase
    print!("{}:  {}\n", c as u8 as char, _isdigit(c)); //
}
