use std::io;
fn main(){
	println!("Welcome!");
	println!("How many siblings do you have", );
	let mut i1 = String::new();
	std::io::stdin().read_line(&mut i1).expect("Not a valid string");
	let i1:usize = i1.trim().parse().expect("Not a valid number");

	let mut input2 = vec![];
	let mut input3 = vec![];

	for input1 in 0..i1  {
		println!("Input information for sibling{}
		\nPlease input his/her name of sibling {} : ",i1 + 1,i1 + 1);
		let mut i2 = String::new();
	    io::stdin().read_line(&mut i2).expect("Not a valid input");
	    i2 = i2.trim().to_string();
	    let input2 = i2.trim().to_string();
	    input2.push(input2);

	    println!("Please input {:?}'s age",i2 );
	    let mut i3 = String::new();
	    io::stdin().read_line(&mut i3).expect("Not a valid string");
	    let input3::u16 = i3.trim().parse().expect("Not a valid number");
	    input3.push(input3);


	}

}

	
	