/*
* mul - Function to multiply two integers
* @a: First integer
* @b: Second integer
* Return: The product of a and b
 */
fn mul(a: i32, b: i32) -> i32 {
    a * b
}
/**
 * main - Entry point of the program
 * Return: () indicates successful execution
 */
fn main() -> () {
    print!("{}\n", mul(98, 1024)); // Print the result
    print!("{}\n", mul(-402, 4096)); // Print the result
}
