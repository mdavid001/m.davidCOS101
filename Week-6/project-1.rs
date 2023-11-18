//a program to conduct PAU student council voting system
use std::io;

fn main() {
	println!("Welcome to The Official PAU Student Council Voting System");
	println!("\nWhat is your name? : ");
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("Not a valid string");

	println!("\nPlease type your email: ");
	let mut input2 = String::new();
	io::stdin().read_line(&mut input2).expect("Not a valid string");

	println!("\nWhich department are you in? : ");
	let mut input3 = String::new();
	io::stdin().read_line(&mut input3).expect("Not a valid string");

	println!("\nWhat is your state of origin? : ");
	let mut input4 = String::new();
	io::stdin().read_line(&mut input4).expect("Not a valid string");

	println!("\nWhich level are you in? : ", );
	let mut i5 = String::new();
	io::stdin().read_line(&mut i5).expect("Not a valid string");
	let input5:i32 = i5.trim().parse().expect("Not a valid number");

	println!("\nWhat is your CGPA? : ");
    let mut i6 = String::new();
	io::stdin().read_line(&mut i6).expect("Not a valid string");
	let input6:f64 = i6.trim().parse().expect("Not a valid number");

	println!("\nInput 1 if you are among the first 150 candidates to vote and any other number if you are not.");
	let mut i7 = String::new();
	io::stdin().read_line(&mut i7).expect("Not a valid string");
	let input7:i32 = i7.trim().parse().expect("Not a valid number");

	println!("\nInput 5 if you are a current class representative and any other number if you are not");
    let mut i8 = String::new();
	io::stdin().read_line(&mut i8).expect("Not a valid string");
	let input8:i32 = i8.trim().parse().expect("Not a valid number");

	println!("");

	println!("\nName: {}",input1 );
	println!("Email address: {}",input2 );
	println!("Department: {}",input3 );
	println!("State of Origin: {}",input4 );

	if input7 == 1 && input5 != 100 && input6 > 4.0 && input8 == 5{
		println!("You can vote!");
	}
	else {
		println!("Sorry, you are not eligible to vote.");
	}
}