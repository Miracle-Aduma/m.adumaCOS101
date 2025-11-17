//Rust program for calculating student's avarage

use std::io;

fn main() {
    println!("\n Students average calculator!");

    //Input name
    println!("\nPlease Enter your name:");
    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("Failed to read input");
    println!("Your name is: {}", name);

     // For test scores
     let mut input1 = String::new();
     let mut input2 = String::new();
     let mut input3 = String::new();

     println!("Enter first test score: ");
     io::stdin().read_line(&mut input1).expect("Not a valid string");
     let first_score:f32 = input1.trim().parse().expect("Not a valid number");

     println!("Enter second test score: ");
     io::stdin().read_line(&mut input2).expect("Not a valid string");
     let second_score:f32 = input2.trim().parse().expect("Not a valid number");

     println!("Enter third test score: ");
     io::stdin().read_line(&mut input3).expect("Not a valid string");
     let third_score:f32 = input3.trim().parse().expect("Not a valid number");

     let total:f32 = (first_score + second_score + third_score) / 3.0;


     println!("Test scores average is: {:2}", total);

     // For Grade

     if total >= 0.0 && total <= 44.0
        {
            println!("Your grade is F");
        }
    else if total >=45.0 && total <=49.0
        {
            println!("Your grade is D");
        }
    else if total >=50.0 && total <=59.0
        {
            println!("Your grade is C");
        }
    else if total >=60.0  && total <=69.0
        {
            println!("Your grade is B");
        }
    else
        {
            println!("Your grade is A");
        }

    println!("Your name is: {}", name);
    println!("Your garde is: {}", total);
}
