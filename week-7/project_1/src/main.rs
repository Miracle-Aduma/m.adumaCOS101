use std::io;

// Function to read input as f64
fn read_number(prompt: &str) -> f64 {
    println!("{}", prompt);
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read");
    input.trim().parse::<f64>().expect("Please enter a valid number")
}

// Area of a Trapezium
fn area_trapezium() -> f64 {
    let height = read_number("Enter height:");
    let base1 = read_number("Enter base 1:");
    let base2 = read_number("Enter base 2:");
    height / 2.0 * (base1 + base2)
}

// Area of a Rhombus
fn area_rhombus() -> f64 {
    let d1 = read_number("Enter diagonal 1:");
    let d2 = read_number("Enter diagonal 2:");
    0.5 * d1 * d2
}

// Area of a Parallelogram
fn area_parallelogram() -> f64 {
    let base = read_number("Enter base:");
    let altitude = read_number("Enter altitude:");
    base * altitude
}

// Area of a Cube
fn area_cube() -> f64 {
    let side = read_number("Enter the length of the side:");
    6.0 * side * side
}

// Volume of a Cylinder
fn volume_cylinder() -> f64 {
    let radius = read_number("Enter radius:");
    let height = read_number("Enter height:");
    std::f64::consts::PI * radius * radius * height
}

fn main() {
    println!("--- MTH 101 AREA & VOLUME CALCULATOR ---");
    println!("Select a calculation:");
    println!("1. Area of Trapezium");
    println!("2. Area of Rhombus");
    println!("3. Area of Parallelogram");
    println!("4. Area of Cube");
    println!("5. Volume of Cylinder");

    // Read user choice
    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read");
    let choice = choice.trim();

    let result = match choice {
        "1" => area_trapezium(),
        "2" => area_rhombus(),
        "3" => area_parallelogram(),
        "4" => area_cube(),
        "5" => volume_cylinder(),
        _ => {
            println!("Invalid choice.");
            return;
        }
    };

    println!("\nResult = {:.4}", result);
}
