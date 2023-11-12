//rust program to read the average height of a person
//and then print whether the person is tall, dwarf or average height

use std::io;
fn main() {
	let mut input = String::new();
	println!("\nEnter your height (in cm): ");
	io::stdin().read_line(&mut input).expect("Not a valid string");
	let height:f32 = input.trim().parse().expect("Not a valid number");

	if height >= 150.0 && height <= 170.0
	{
		println!("\nYou are the height of an average person");
	}
	else if height > 170.0 && height <= 195.0
	{
		println!("\nYou are a tall ");
	}
	else if height < 150.0 && height > 100.0
	{
		println!("\nYou are a dwarf");
	}
	else {
		println!("\nYou have an abnormal height");
	}
}