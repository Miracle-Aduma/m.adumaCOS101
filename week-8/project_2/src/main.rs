use std::io;

fn main() {
    let mut developer_records: Vec<(String, i32)> = Vec::new();

    println!("--- EY Developer Scouting Program ---");

    // How many developers will be entered
    println!("Enter number of candidates to evaluate:");

    let mut count_input = String::new();
    io::stdin().read_line(&mut count_input).expect("Failed to read input");
    let candidate_count: usize = count_input.trim().parse().expect("Please enter a valid number");

    for i in 1..=candidate_count {
        println!("\nCandidate {}:", i);

        // Input developer name
        println!("Enter developer's name:");
        let mut name = String::new();
        io::stdin().read_line(&mut name).expect("Failed to read name");
        let name = name.trim().to_string();

        // Input developer experience
        println!("Enter years of programming experience:");
        let mut exp_input = String::new();
        io::stdin().read_line(&mut exp_input).expect("Failed to read experience");
        let experience: i32 = exp_input.trim().parse().expect("Enter a valid number");

        developer_records.push((name, experience));
    }

    // Determine the developer with the highest experience
    let mut highest_experience = -1;
    let mut winner_name = String::new();

    for (name, experience) in &developer_records {
        if *experience > highest_experience {
            highest_experience = *experience;
            winner_name = name.clone();
        }
    }

    println!("\n--- EY Developer Scouting Results ---");
    println!("Total candidates evaluated: {}", developer_records.len());
    println!("Top Developer:");
    println!("Name: {}", winner_name);
    println!("Experience: {} years", highest_experience);
}
