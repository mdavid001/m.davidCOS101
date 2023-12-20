struct Electronics{
	name:String,
	price:f64,
	qty:i32
}

fn main(){
	let input1 = Electronics{
		name:String::from("HP"),
		price:650000.00,
		qty:10
	};
	let input2 = Electronics{
		name:String::from("IBM"),
		price:755000.00,
		qty:6
	};
    let input3 = Electronics{
		name:String::from("Toshiba"),
		price:550000.00,
		qty:10
	};
	let input4 = Electronics{
		name:String::from("Dell"),
		price:850000.00,
		qty:4
	};
display(input1);
display(input2);
display(input3);
display(input4);
}

fn display(input:Electronics){
		println!("Name of brand: {} electronics\nPrice of laptops: {} \nQuantity of laptops available: {}",input.name, input.price, input.qty );
        println!("\nYou have decided to buy three laptops from each brand ");
        println!("Invoice:");
        println!("3 {} laptops : 3 x {} = {} ",input.name, input.price, 3.0*input.price );

}