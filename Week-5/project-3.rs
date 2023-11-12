//rust program to display menu for food items available

use std::io;

fn main(){
	println!("Welcome to the resturant.\nWe have the following meals available");
	println!("\nA portion of poundo yam and edinkaiko soup is N3200 ");
	println!("\nA portion of fried rice and chicken is N3000");
	println!("\nA portion of amala and ewedu soup is N2500");
	println!("\nA portion of eba and egusi soup is N2000");
	println!("\nA portion of white rice and stew is N2500");

	println!("");

	println!("\nEnter portion of poundo yam and edinkaiko you want");
		let mut want1 = String::new();
	io::stdin().read_line(&mut want1).expect("Not a valid string");
	let input1:f64 = want1.trim().parse().expect("Not a valid number");

	println!("\nEnter portion of fried rice and chicken you want");
		let mut want2 = String::new();
	io::stdin().read_line(&mut want2).expect("Not a valid string");
	let input2:f64 = want2.trim().parse().expect("Not a valid number");
	
	println!("\nEnter portion of amala and ewedu soup you want");
		let mut want3 = String::new();
	io::stdin().read_line(&mut want3).expect("Not a valid string");
	let input3:f64 = want3.trim().parse().expect("Not a valid number");
	
	println!("\nEnter portion of eba and egusi soup you want");
		let mut want4 = String::new();
	io::stdin().read_line(&mut want4).expect("Not a valid string");
	let input4:f64 = want4.trim().parse().expect("Not a valid number");
	
	println!("\nEnter portion of white rice and stew you want");
		let mut want5 = String::new();
	io::stdin().read_line(&mut want5).expect("Not a valid string");
	let input5:f64 = want5.trim().parse().expect("Not a valid number");

	let pcost:f64 = 3200.0;
	let fcost:f64 = 3000.0;
	let acost:f64 = 2500.0;
	let ecost:f64 = 2000.0;
	let wcost:f64 = 2500.0;

	let p = input1 * pcost; 

	let f = input2 * fcost; 

	let a = input3 * acost; 

	let e = input4 * ecost; 

	let w = input5 * wcost; 

	let bill = p + f + a + e + w;

	if bill > 10000.0 {
		let discount = 0.95 * bill;
		println!("\nYou have a discount of 5% and your new bill is {}",discount );
	}

	else {
		println!("\nYour total bill is {}",bill );
	}

}	