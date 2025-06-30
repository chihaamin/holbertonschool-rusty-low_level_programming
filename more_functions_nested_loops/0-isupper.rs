/*
* _isupper - Check if a character is uppercase
* @c: The character to check
* * Return: 1 if c is uppercase, 0 otherwise
 */
fn _isupper(c: i32) -> i32 {
    if c >= 'A' as i32 && c <= 'Z' as i32 {
        return 1;
    } else {
        return 0;
    }
}
/*
* main - Entry point of the program
* Return: () indicates successful execution
 */
fn main() -> () {
    let mut c: i32 = 'A' as i32; // Example character to check
    print!("{}:  {}\n", c as u8 as char, _isupper(c)); // Print the result
    c = 'a' as i32; // Change character to lowercase
    print!("{}:  {}\n", c as u8 as char, _isupper(c)); //
}
