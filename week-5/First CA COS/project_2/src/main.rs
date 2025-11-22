//Rust code for calculating compound interest

use std::io;

fn main() {
     let mut input1 = String::new();
     let mut input2 = String::new();
     let mut input3 = String::new();


     println!("Enter the value of p: ");
     io::stdin().read_line(&mut input1).expect("Not a valid string");
     let p:f32 = input1.trim().parse().expect("Not a valid number");

     println!("Enter the value of r: ");
     io::stdin().read_line(&mut input2).expect("Not a valid string");
     let r:f32 = input2.trim().parse().expect("Not a valid number");

     println!("Enter the value of t: ");
     io::stdin().read_line(&mut input3).expect("Not a valid string");
     let t:f32 = input3.trim().parse().expect("Not a valid number");

    //Compound interest
    let amount::f32 = p * (1 + r / 100.0) ^ t;
    println!("Amount is {}", amount);

}
