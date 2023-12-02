fn main(){
	println!("How many siblings do you have", );
	let mut i1 = String::new();
	std::io::stdin().read_line(&mut i1).expect("Not a valid string");
	let a:i32 = i1.trim().parse().expect("Not a valid number");

	loop{
		println!("Name of sibling : {:?}",a);
		let mut input2 = String::new();
	    std::io::stdin().read_line(&mut input2).expect("Not a valid string");

		println!("Input 1 if you have more siblings and any other number if you dont");
		let mut i3 = String::new();
	    std::io::stdin().read_line(&mut i3).expect("Not a valid string");
	    let input3:i32 = i3.trim().parse().expect("Not a valid number");
	    if input3 == 1{
	    	continue;
	    }
	    else{
	    	println!("The name of your sibling(s) are : {:?}",input3 );
	    	break;
	    }

	}

	if a == 0{
		println!("Sorry you are not eligible for this program");
	}

}