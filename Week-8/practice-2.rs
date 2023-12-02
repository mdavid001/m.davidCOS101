fn main(){
	let v = vec!['C','O','M','P','U','T','E','R'];

	let mut input1 = String::new();
	println!("Enter an index value between 0-7");
	std::io::stdin().read_line(&mut input1).expect("Not a valid input");
	//index is the non negative value which is smaller than the size of the vector
	let index:usize = input1.trim().parse().expect("Not a valid number");

	//getting value at given index value
	let ch:char = v[index];

	println!("{} is the character for index [{}]\n",ch,index );
}