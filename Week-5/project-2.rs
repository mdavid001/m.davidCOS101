//rust program to take experience and age of an employee

use std::io;

fn main() {
	println!("Enter your age: ");
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("Not a valid string");
	let age:i32 = input1.trim().parse().expect("Not a valid number");

	println!("\nEnter number of years of experience.\nEnter '0' if inexperienced");
	let mut exp = String::new();
	io::stdin().read_line(&mut exp).expect("Not a valid string");
	let yrs:i32 = exp.trim().parse().expect("Not a valid number");
	

	if age >= 40 && yrs > 0
	{
		println!("\nThe incentive is 1_560_000");
	}

	else if age >=30 && age < 40 && yrs > 0
	{
		println!("\nThe incentive is 1_480_000");
	}

	else if age < 30 && yrs > 0
	{
		println!("\nThe incentive is 1_300_000");
	}

	else 
	{
		println!("\nThe incentive is 100_000");
	}

}