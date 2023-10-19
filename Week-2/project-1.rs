fn main() {
let p:f64 = 525_000.00;
let n:f64 = 5.0;
let r:f64 = 10.00;

//compound interest
let a = p * (1.0 + (r/100.0)) * n;
println!("the amount is {}", a );
let ci = a - p;
println!("the compound {}", ci );
}