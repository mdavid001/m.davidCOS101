fn main() { 
let fullname = "Michelle Ada David";
let department = "Computer Science";
let uni = "Pan Atlantic University";

let mut school = "School of science".to_string();
//push string
school.push_str(" and Technology");

println!("My name is {}",fullname );
//check length 
println!("The length of my fullname is {}",fullname.len());
println!("I am a student of {} department",department );
println!("{}",school );
println!("{}",uni);

}