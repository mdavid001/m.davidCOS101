//rust program to count numbers

use std::io;
fn main() {
	println!("Enter lower bound : ");
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("Not a valid string");
	let lower_bound:i32 = input1.trim().parse().expect("Not a valid number");

	println!("\nEnter upper bound : ");	
	let mut input2 = String::new();
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let upper_bound:i32 = input2.trim().parse().expect("Not a valid number");

	for x in lower_bound..upper_bound{//upper bound is not inclusive
	println!("\nCount level is : {}",x ); 
	}


}