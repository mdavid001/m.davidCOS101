fn main(){
	println!("multiplication times table");
	let mut value;
	for i in 1..13 {
		for b in 1..13{
			value = i * b;
		    println!("{} x {} = {:?}",i,b,value );
		}
	
	}
}