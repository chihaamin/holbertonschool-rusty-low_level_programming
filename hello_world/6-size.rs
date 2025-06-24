/*
main - Entry point of the program
This function prints the sizes of various data types in Rust to the standard output.
Returns: ()
 */
fn main() -> () {
    println!("Size of char: {} byte(s)", std::mem::size_of::<char>());
    println!("Size of short: {} byte(s)", std::mem::size_of::<i16>());
    println!("Size of int: {} byte(s)", std::mem::size_of::<i32>());
    println!("Size of long: {} byte(s)", std::mem::size_of::<i64>());
    println!("Size of long long: {} byte(s)", std::mem::size_of::<i128>());
    println!("Size of float: {} byte(s)", std::mem::size_of::<f32>());
    println!("Size of double: {} byte(s)", std::mem::size_of::<f64>());
    println!("Size of pointer: {} byte(s)", std::mem::size_of::<usize>());
}
