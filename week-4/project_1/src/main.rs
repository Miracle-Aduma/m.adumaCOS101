//Rust program to calculate the roots of a quadratic equation

use std::io;

fn main() 
{
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();

    println!("Enter the value of a");
    io::stdin().read_line(&mut input1).expect("Not a valid string");
    let a:f32 = input1.trim().parse().expect("Not a valid number");

    println!("Enter the value of b");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let b:f32 = input2.trim().parse().expect("Not a valid number");

    println!("Enter the value of c");
    io::stdin().read_line(&mut input3).expect("Not a valid string");
    let c:f32 = input3.trim().parse().expect("Not a valid number");

    if a == 0.0 {
        println!("This is not a quadratic equation (a cannot be 0).");
        return;
    }

    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let root1 = (-b + discriminant.sqrt()) / (2.0 * a);
        let root2 = (-b - discriminant.sqrt()) / (2.0 * a);
        println!("Two distinct real roots: x1 = {}, x2 = {}", root1, root2);
    }
    else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        println!("One real root: x = {}", root);
    } 
    else {
        let real_part = -b / (2.0 * a);
        let imag_part = (-discriminant).sqrt() / (2.0 * a);
        println!("Complex roots: x1 = {} + {}i, x2 = {} - {}i",real_part, imag_part, real_part, imag_part);
    }
}
