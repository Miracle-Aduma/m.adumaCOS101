use std::io::{self, Write};

struct Laptop {
    brand_name: String,
    unit_cost: f64,
    stock_quantity: i32,
}

impl Laptop {
    // Return unit price
    fn unit_price(&self) -> f64 {
        self.unit_cost
    }

    // Return how many are available
    fn available(&self) -> i32 {
        self.stock_quantity
    }

    // Attempt to purchase `qty`. If successful, reduce stock and return total cost.
    fn purchase(&mut self, qty: i32) -> Result<f64, String> {
        if qty <= 0 {
            return Err("Quantity must be positive.".to_string());
        }
        if qty > self.stock_quantity {
            return Err(format!(
                "Not enough stock. {} has only {} unit(s) left.",
                self.brand_name, self.stock_quantity
            ));
        }
        self.stock_quantity -= qty;
        Ok(self.unit_cost * qty as f64)
    }
}

fn read_line_trimmed() -> String {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).expect("Failed to read input");
    buf.trim().to_string()
}

fn main() {
    // Initialize inventory
    let mut inventory = vec![
        Laptop { brand_name: "HP".to_string(), unit_cost: 650_000.0, stock_quantity: 10 },
        Laptop { brand_name: "IBM".to_string(), unit_cost: 755_000.0, stock_quantity: 6  },
        Laptop { brand_name: "Toshiba".to_string(), unit_cost: 550_000.0, stock_quantity: 10 },
        Laptop { brand_name: "Dell".to_string(), unit_cost: 850_000.0, stock_quantity: 4  },
    ];

    println!("--- Mr. Ogbeifuna's Alaba Market Inventory ---");

    // Purchases record: (brand, qty, line_total)
    let mut purchases: Vec<(String, i32, f64)> = Vec::new();
    let mut grand_total: f64 = 0.0;

    loop {
        // Display inventory
        println!("\nAvailable devices:");
        for (i, item) in inventory.iter().enumerate() {
            println!(
                "{}. {} — Unit Price: N{:.2} — Units left: {}",
                i + 1,
                item.brand_name,
                item.unit_price(),
                item.available()
            );
        }

        println!("\nEnter the item number to buy (or 'q' to finish):");
        print!("> ");
        io::stdout().flush().unwrap();
        let choice = read_line_trimmed();
        if choice.eq_ignore_ascii_case("q") {
            break;
        }

        // Parse item number
        let item_index: usize = match choice.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid input. Please enter a valid item number or 'q'.");
                continue;
            }
        };

        if item_index == 0 || item_index > inventory.len() {
            println!("Item number out of range. Choose a number between 1 and {}.", inventory.len());
            continue;
        }

        // Ask for quantity
        println!("Enter quantity to purchase for {}:", inventory[item_index - 1].brand_name);
        print!("> ");
        io::stdout().flush().unwrap();
        let qty_input = read_line_trimmed();
        let qty: i32 = match qty_input.parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invalid quantity. Please enter a whole number.");
                continue;
            }
        };

        // Attempt purchase
        let purchase_result = {
            // get mutable reference to the selected laptop
            let laptop = &mut inventory[item_index - 1];
            laptop.purchase(qty)
        };

        match purchase_result {
            Ok(line_total) => {
                let brand = inventory[item_index - 1].brand_name.clone();
                println!(
                    "Added to cart: {} x {} — Line total: N{:.2}",
                    brand, qty, line_total
                );
                purchases.push((brand, qty, line_total));
                grand_total += line_total;
            }
            Err(msg) => {
                println!("Could not purchase: {}", msg);
            }
        }

        // Ask if user wants to continue or finish quickly
        println!("Do you want to keep shopping? (y/n) [default: y]");
        print!("> ");
        io::stdout().flush().unwrap();
        let cont = read_line_trimmed();
        if cont.eq_ignore_ascii_case("n") {
            break;
        }
    }

    // Print receipt
    println!("\n--- Purchase Receipt ---");
    if purchases.is_empty() {
        println!("No items purchased.");
    } else {
        println!("{:<12} {:>8} {:>18}", "Brand", "Qty", "Line Total (N)");
        for (brand, qty, line_total) in &purchases {
            println!("{:<12} {:>8} {:>18.2}", brand, qty, line_total);
        }
        println!("\nGrand Total: N{:.2}", grand_total);
    }

    // Print remaining inventory
    println!("\n--- Remaining Inventory ---");
    for item in &inventory {
        println!(
            "{} — Unit Price: N{:.2} — Units left: {}",
            item.brand_name,
            item.unit_price(),
            item.available()
        );
    }

    println!("\nThank you for shopping at Mr. Ogbeifuna's Alaba Market!");
}
