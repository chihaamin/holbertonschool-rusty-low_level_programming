use std::time::{SystemTime, UNIX_EPOCH};
/**
* main - Entry point of the program
* This program generates a pseudo-random number based on the current time
* and checks the last digit of the number.
* It prints whether the last digit is 0,
* less than 6 and not 0, or greater than 5.
* Returns: Nothing
*/
fn main() -> () {
    let seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let n = (seed.wrapping_mul(1103515245) + 12345) as i32;
    let last_int = n % 10;

    if n == 0 {
        println!("Last digit of {} is 0 and is 0\n", n);
    } else if n < 5 {
        println!(
            "Last digit of {} is {} and is less than 6 and not 0\n",
            n, last_int
        );
    } else {
        println!(
            "Last digit of {} is {} and is greater than 5\n",
            n, last_int
        );
    }
}
