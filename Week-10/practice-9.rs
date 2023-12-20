//define dimensions of a rectangle

struct Rectangle {
	width:u32 , height:u32
}

//logic to calculate the area of a rectangle 
impl Rectangle {
	fn area(&self)->u32 {
		//use the . operator to fetch the value of a field via the self keyword
		self.width * self.height 
	}
}

fn main(){
	//instantiate the structure
	let small = Rectangle {
		width:10,
		height:20
	};
	//print the rectangles area
	println!("width is {} \nheight is {} \narea of rectangle is {} ",small.width, small.height,small.area());
}