// define dimensions of a rectangle
struct Rectangle {
    width:u32, 
    height:u32
}

// Logic to calculate area of a traingle
impl Rectangle {
    fn area(&self)->u32 {
        // use the . operator to fetch the value of a field via tje self keyword
        self.width * self.height
    }
}

fn main() {
    // instanatiate the structure
    let small = Rectangle {
        width:10,
        height:20
    };
    // Print the recatangle's area
    println!("width is {} \n height is {} \n area of Rectangle is {}",small.width,small.height,small.area());
}
