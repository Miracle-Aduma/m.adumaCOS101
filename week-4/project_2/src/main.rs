//Rust code for finding employees annual incentive

use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    println!("Annual Incentive of Employees");

    println!("\nIs the employee experienced? (yes/no): ");
    io::stdin().read_line(&mut input1).expect("Failed to read input");
    let experience = input1.trim().to_lowercase();

    println!("\nPlease enter your Age: ");
    io::stdin().read_line(&mut input2).expect("Not a valid string");
    let age:u32 = input2.trim().parse().expect("Not a valid number");

    //Finding incentive
    

    if experience == "yes" {


        if age >= 40
        {
            println!("Your incentive is ₦1560000");
        }
        else if age >= 30 && age < 40
        {
            println!("Your incentive is ₦1480000");
        }
        else if age < 28
        {
            println!("Your incentive is ₦1300000");
        }
        else
        {
        println!("Your incentive is ₦100000");
        }
    }
}