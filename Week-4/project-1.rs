//rust program to calculate speed

use std::io;

fn main() {

	//program to calculate first speed
	let d:f64 = 80.0;
	let t:f64 = 2.0;

	let dkm:f64 = d * 1.60934;
	let s = dkm / t;

	println!("The speed of the car when the distance is 80 miles is {}", s);

	//input distance of car in miles
	println!("\nEnter the number of miles taken by the car \n");
	let mut distance = String::new();
	io::stdin()
	.read_line(&mut distance)
	.expect("Failed to read input");
	let a:f64 = distance.trim().parse().expect("Not a valid number");


	//input distance of car in hours
	println!("\nEnter the number of hours used by the car\n");
	let mut time = String::new();
	io::stdin()
	.read_line(&mut time)
	.expect("Failed to read input");
	let b:f64 = time.trim().parse().expect("Not a valid number");

	//conversion of speed from miles to kilometers
	let km:f64 = a * 1.60934;

	//formula to find the speed 
	let speed:f64 = km / b;

	println!("\nThe speed of the car is {} km per hour", speed);
}