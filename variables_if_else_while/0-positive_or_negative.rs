use std::time::{SystemTime, UNIX_EPOCH};
/**
* main - Entry point of the program
* This program generates a pseudo-random number based on the current time
* and checks if it is positive, negative, or zero.
* Returns: Nothing
*/
 */

fn main() -> () {
	let seed = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap()
		.as_secs();

	let n = (seed.wrapping_mul(1103515245) + 12345) as i32;

	print!("{} ", n);
	if n > 0 {
		println!("is positive");
	} else if n < 0 {
		println!("is negative");
	} else {
		println!("is zero");
	}
}
