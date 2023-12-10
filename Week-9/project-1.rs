use std::io::Write;

fn main() {
	let lager = vec!["\n33 Export\n","Desperados\n","Goldberg\n","Gulder\n","Heineken\n","Star\n"];
    let stout = vec!["\nLegend\n","Turbo King\n","Williams\n"];
    let nonalcohol = vec!["\nMaltina\n","Amstel malta\n","Malta Gold\n","Fayrouz\n"];

    let mut file = std::fs::File::create("project_1.txt").expect("create failed");
    file.write_all("Welcome to Nigerian Breweries PLC.!\nOur high quality categories of drinks include:".as_bytes()).expect("write failed");
    file.write_all("\n1)Lager drinks:".as_bytes()).expect("Failed to write into file");
    for index in 0..lager.len() {
    	file.write_all(lager[index].as_bytes()).expect("Failed to write into file");
    }

      file.write_all("\n2)Stout drinks:".as_bytes()).expect("Failed to write into file");
    for index in 0..stout.len() {
    	file.write_all(stout[index].as_bytes()).expect("Failed to write into file");
    }

      file.write_all("\n3)Non-alcoholic drinks:".as_bytes()).expect("Failed to write into file");
    for index in 0..nonalcohol.len() {
    	file.write_all(nonalcohol[index].as_bytes()).expect("Failed to write into file");
    }

    println!("Data written to file.");
}