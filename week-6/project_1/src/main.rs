use std::io::{self, Write};

fn main() {
    // Define menu: (code, name, unit_price)
    let menu = vec![
        ("P", "Poundo Yam / Edinkaiko Soup", 3_200.0),
        ("F", "Fried Rice & Chicken", 3_000.0),
        ("A", "Amala & Ewedu Egusi Soup", 2_500.0),
        ("E", "Eba & Ewedu Egusi Soup", 2_000.0),
        ("W", "White Rice & Stew Soup", 2_500.0),
    ];

    println!("Welcome to the restaurant!");
    println!("Here is our menu:");
    println!("{:<5} {:<35} {:>10}", "Code", "Food Item", "Price (₦)");
    for (code, name, price) in &menu {
        println!("{:<5} {:<35} {:>10.2}", code, name, price);
    }

    let mut total: f64 = 0.0;

    loop {
        println!("\nEnter the food code to order (or 'Q' to finish):");
        print!("> ");
        io::stdout().flush().unwrap();

        let mut code_input = String::new();
        io::stdin().read_line(&mut code_input).expect("Failed to read input");
        let code_input = code_input.trim().to_uppercase();
        if code_input == "Q" {
            break;
        }

        // Find the item in menu
        let menu_item = menu.iter().find(|(c, _n, _p)| c == &code_input);
        let (food_name, unit_price) = if let Some((_c, name, price)) = menu_item {
            (name, *price)
        } else {
            println!("Invalid code. Please enter a valid food code from the menu.");
            continue;
        };

        println!("Enter quantity for {}:", food_name);
        print!("> ");
        io::stdout().flush().unwrap();
        let mut qty_input = String::new();
        io::stdin().read_line(&mut qty_input).expect("Failed to read input");
        let qty_input = qty_input.trim();
        let qty: i32 = match qty_input.parse() {
            Ok(n) if n > 0 => n,
            _ => {
                println!("Invalid quantity. Please enter a positive whole number.");
                continue;
            }
        };

        let cost = unit_price * qty as f64;
        println!("Added to order: {} x {} — Subtotal: ₦{:.2}", food_name, qty, cost);
        total += cost;
    }

    if total == 0.0 {
        println!("\nNo items ordered. Thank you!");
    } else {
        println!("\nOrder summary:");
        println!("Subtotal: ₦{:.2}", total);
        if total > 10_000.0 {
            let discount = total * 0.05;
            let final_total = total - discount;
            println!("Discount (5%): -₦{:.2}", discount);
            println!("Total after discount: ₦{:.2}", final_total);
        } else {
            println!("Total: ₦{:.2}", total);
        }
        println!("Thank you for your order!");
    }
}
