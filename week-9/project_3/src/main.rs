use std::fs::File; 

use std::io::Write; 

fn main() {
    // Dataset 1: Name of Commissioner 
    let names: Vec<&str> = vec![
        "Aigbogun Alamba Daudu", 
        "Murtala Afeez Bendu", 
        "Okorocha Calistus Ogbona", 
        "Adewale Jimoh Akanbi", 
        "Osazuwa Faith Etieye"
    ];

    // Dataset 2: Geopolitical Zone
    let zones: Vec<&str> = vec![
        "South West", 
        "North East", 
        "South South", 
        "South West", 
        "South East"
    ];

    // Dataset 3: Ministry
    let ministries: Vec<&str> = vec![
        "Internal Affairs", 
        "Justice", 
        "Defense", 
        "Power & Steel", 
        "Petroleum"
    ];

    let mut merged_content = String::from("EFCC Convicted Ministers - Merged Dataset\n--------\n");
    
    let record_count = names.len();

    for i in 0..record_count {
        
        let name = names[i];
        let zone = zones[i];
        let ministry = ministries[i];
        
        let record = format!(
            "Minister {}:\nName: {}\nZone: {}\nMinistry: {}\n\n",
            i + 1, 
            name, 
            zone, 
            ministry
        );
        merged_content.push_str(&record);
    }

    let mut file = File::create("efcc_convicted_ministers.txt")
        .expect("Failed to create the output file.");

    file.write_all(merged_content.as_bytes())
        .expect("Failed to write merged data to the file.");

    println!("\n--- EFCC Data Merger Complete ---");
    println!("Successfully merged {} records into 'efcc_convicted_ministers.txt'.", record_count);
}
