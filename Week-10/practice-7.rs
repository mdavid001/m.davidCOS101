struct Employee{
	name:String,
	company:String,
	age:u32
}

fn main(){
	let emp1 = Employee{
		company:String::from("Ernest and Young"),
		name:String::from("Ada David"),
		age:25
	};
	println!("Name = {} \n",emp1.name );
	println!("Company = {} \n",emp1.company );
	println!("Age = {} \n",emp1.age );

}