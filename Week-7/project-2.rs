use std::io;
fn main(){
	println!("Welcome!");
	println!("How many siblings do you have", );
	let mut i1 = String::new();
	std::io::stdin().read_line(&mut i1).expect("Not a valid string");
	let input1:i32 = i1.trim().parse().expect("Not a valid number");

	let mut input2 = vec![];

	for a in 0..input1  {
		println!("Enter the name of sibling {} : ",a+1);
		let mut i2 = String::new();
	    io::stdin().read_line(&mut i2).expect("Not a valid string");
	    i2 = i2.trim().to_string();
	    let input2 = i2.trim().to_string();
	    input2.push(input2)

	}

}

	
	