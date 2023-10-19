fn main() {
	let p:f64 = 210_000.00;
	let r:f64  = 5.00;
	let n:f64 = 3.00;

	//depreciation
	let a = p * (1.0-(r/100.0)).powf(n); 
	println!("the value of the tv after 3 years is {}",a );
}