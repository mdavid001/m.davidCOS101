use std::io;
use std::io::Read;

fn main(){
	println!("Welcome to Globacom Ltd.");
	println!("");
	println!("Press 1 if you are an administrator", );
	println!("Press 2 if you are a project manager", );
	println!("Press 3 if you are an employee", );
	println!("Press 4 if you are a customer", );
	println!("Press 5 if you are a vendor");

	let mut input1= String::new();
	io::stdin().read_line(&mut input1).expect("Not a valid String");
	let a:i32 = input1.trim().parse().expect("Not a valid number");

	if a == 1{ database();	}
	if a == 2{ project();	}
	if a == 3{ staff();    }
	if a == 4{ customer(); }
	if a == 5{ dataplan(); }
	if a < 1 && a > 5{ println!("ERROR!"); }
}

fn database(){
	let mut file = std::fs::File::open("globacom_tb.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}",contents);
}
fn project(){
	let mut file = std::fs::File::open("project_tb.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}",contents);
}
fn staff(){
	let mut file = std::fs::File::open("staff_tb.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}",contents);
}
fn customer(){
	let mut file = std::fs::File::open("customer_tb.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}",contents);
}
fn dataplan(){
	let mut file = std::fs::File::open("dataplan_tb.sql").unwrap();
	let mut contents = String::new();
	file.read_to_string(&mut contents).unwrap();
	print!("{}",contents);
}