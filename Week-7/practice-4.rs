use std::io;

fn add(a:i32, b:i32){
	let sum = a + b;
	println!("Sum of a and b = {}",sum );
}

fn main(){
	println!("Enter input for parameter A : ");
	let mut input1 = String::new();
	io::stdin().read_line(&mut input1).expect("Not a valid string");
	let a:i32 = input1.trim().parse().expect("Not a valid number");

	println!("Enter input for parameter B : ");
	let mut input2 = String::new();
	io::stdin().read_line(&mut input2).expect("Not a valid string");
	let b:i32 = input2.trim().parse().expect("Not a valid number");

	//call add function with arguments
	add(a,b);
}