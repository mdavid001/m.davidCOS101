//program to develop a a researchers pUblic incentive system for NRG
use std::io;

fn main(){
	println!("WELCOME TO RPIS!");

	println!("\nInput 1 if you are part of the first 500 researchers to take this survey.");
	println!("Input any other number if you are not part of the first 500 researchers to take this survey");
	let mut i1 = String::new();
	io::stdin().read_line(&mut i1).expect("Not a valid string");
	let input1:i32 = i1.trim().parse().expect("Not a valid number");

	if input1 == 1{
		println!("\nWhat is your name? : ");
		let mut input2 = String::new();
	    io::stdin().read_line(&mut input2).expect("Not a valid string");

	    println!("\nHow many papers have you published? : ");
	    let mut i3 = String::new();
	    io::stdin().read_line(&mut i3).expect("Not a valid string");
	    let input3:i32 = i3.trim().parse().expect("Not a valid number");

        println!("");
	    println!("Name : {}",input2 );

	    if input3 < 3 && input3 >= 0{
	    	println!("Your incentive is N100,000.");
	    }
	    else if input3 >= 3 && input3 <= 5{
	    	println!("Your incentive is N500,000.");
	    }
	    else if input3 > 5 && input3 < 10{
	    	println!("Your incentive is 800,000.");
	    }
	    else if input3 > 10 {
	    	println!("Your incentive is N1,000,000.");
	    }
	    else {
	    	println!("ERROR");
	    }

	}
	else {
		println!("You cannot take this survey!");
	}

}
