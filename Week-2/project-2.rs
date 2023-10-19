fn main() {
	let toshiba_qty:f64 = 2.0;
	let mac_qty:f64 = 1.0;
	let hp_qty:f64 = 3.0;
	let dell_qty:f64 = 3.0;
	let acer_qty:f64 = 1.0;
	let toshiba_amount:f64 = 450_000.00;
	let mac_amount:f64 = 1_500_000.00;
	let hp_amount:f64 = 750_000.00;
	let dell_amount:f64 = 2_850_000.00;
	let acer_amount:f64 = 250_000.00;
	 
	 //sum
	 let toshiba_sum = toshiba_qty * toshiba_amount;
	 println!("the total sum of acer sales is {}",toshiba_sum );
	 let mac_sum = mac_qty * mac_amount;
	 println!("the total sum of mac sales is {}",mac_sum );
	 let hp_sum = hp_qty * hp_amount;
	 println!("the total sum of hp sales is {}",hp_sum );
	 let dell_sum = dell_qty * dell_amount;
	 println!("the total sum of dell sales is {}",dell_sum );
	 let acer_sum = acer_qty * acer_amount;
	 println!("the total sum of acer sales is {}",acer_sum );

	 //total sum
	 let total_sum = toshiba_sum + mac_sum + hp_sum + dell_sum + acer_sum;
	 println!("the total sum of all sales made by P.N Okeke and Sons Ltd is {}",total_sum ); 

	 //total quantity 
	 let total_quantity = toshiba_qty + mac_qty + hp_qty + dell_qty + acer_qty;
	 println!("the total quantity of goods sold by P.N Okeke and Sons Ltd is {}",total_quantity );

	 //average
	 let average = total_sum / total_quantity;
	 println!("the average sales made by P.N Okeke and Sons Ltd is {}",average );
}