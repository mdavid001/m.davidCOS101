fn main(){
	println!("How many siblings do you have", );
	let mut i1 = String::new();
	io::stdin().read_line(&mut i1).expect("Not a valid string");
	let input1:i32 = i1.trim().parse().expect("Not a valid number");
}