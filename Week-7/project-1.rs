use std::io;

fn trapezium(){
	//to find the area of the trapezium
	println!("AREA OF TRAPEZIUM!");

	println!("Input the height of the trapezium: ");
	let mut i1 = String::new();
	io::stdin().read_line(&mut i1).expect("Not a valid string");
	let input1:f64 = i1.trim().parse().expect("Not a valid number");

	println!("Input the first base of the trapezium: ");
	let mut i2 = String::new();
	io::stdin().read_line(&mut i2).expect("Not a valid string");
	let input2:f64 = i2.trim().parse().expect("Not a valid number");

	println!("Input the second base of the trapezium: ");
	let mut i3 = String::new();
	io::stdin().read_line(&mut i3).expect("Not a valid string");
	let input3:f64 = i3.trim().parse().expect("Not a valid number");

	let a = (input1 / 2.0) * (input2 + input3);
	println!("The area of the trapezium is {} cm^2.",a );

}

fn rhombus(){
	//to find the area of the rhombus
	println!("AREA OF RHOMBUS!");

	println!("Input the first diagonal of the rhombus: ");
	let mut i1 = String::new();
	io::stdin().read_line(&mut i1).expect("Not a valid string");
	let input1:f64 = i1.trim().parse().expect("Not a valid number");

	println!("Input the second diagonal of the rhombus: ");
	let mut i2 = String::new();
	io::stdin().read_line(&mut i2).expect("Not a valid string");
	let input2:f64 = i2.trim().parse().expect("Not a valid number");

	let a = 0.5 * input1 * input2;
	println!("The area of the rhombus is {} cm^2.",a );

}

fn parallelogram(){
	//to find the area of the parallelogram
	println!("AREA OF PARALLELOGRAM!");

	println!("Input the base of the parallelogram: ");
	let mut i1 = String::new();
	io::stdin().read_line(&mut i1).expect("Not a valid string");
	let input1:f64 = i1.trim().parse().expect("Not a valid number");

	println!("Input the altitude of the parallelogram: ");
	let mut i2 = String::new();
	io::stdin().read_line(&mut i2).expect("Not a valid string");
	let input2:f64 = i2.trim().parse().expect("Not a valid number");

	let a = input1 * input2;
	println!("The area of the parallelogram is {} cm^2.",a );

}

fn cube(){
	println!("AREA OF A CUBE!");

	//to find the area of a cube
	println!("Input the length of the side of the cube: ");
	let mut i1 = String::new();
	io::stdin().read_line(&mut i1).expect("Not a valid string");
	let input1:f64 = i1.trim().parse().expect("Not a valid number");

	let a = 6.0 * input1 * input1;
	println!("The area of the cube is {} cm^2.",a );

}

fn cylinder(){
	//to find the volume of the cyclinder
	println!("VOLUME OF CYLINDER!");

	println!("Input the radius of the cylinder: ");
	let mut i1 = String::new();
	io::stdin().read_line(&mut i1).expect("Not a valid string");
	let input1:f64 = i1.trim().parse().expect("Not a valid number");

	println!("Input the height of the cylinder: ");
	let mut i2 = String::new();
	io::stdin().read_line(&mut i2).expect("Not a valid string");
	let input2:f64 = i2.trim().parse().expect("Not a valid number");

	let v = (22.0 / 7.0) * input1 * input1 * input2;
	println!("The volume of the cylinder is {} cm^2.",v );

}

fn main(){
	println!("Welcome to the calculator for shapes");
	println!("\nYou are to choose from the following shapes");
	println!("Click 1 to find the area of the trapezium", );
	println!("Click 2 to find the area of the rhombus", );
	println!("Click 3 to find the area of the parallelogram", );
	println!("Click 4 to find the area of the cube", );
	println!("Click 5 to find the volume of the cylinder", );

	let mut i12 = String::new();
	io::stdin().read_line(&mut i12).expect("Not a valid string");
	let input12:i32 = i12.trim().parse().expect("Not a valid number");

	if input12 == 1{
		trapezium();
	}
	if input12 == 2{
		rhombus();
	}
	if input12 == 3{
		parallelogram();
	}
	if input12 == 4{
		cube();
	}
	if input12 == 5{
		cylinder();
	}
	if input12 < 1 && input12 > 5{
		println!("ERROR!");
	}
}



