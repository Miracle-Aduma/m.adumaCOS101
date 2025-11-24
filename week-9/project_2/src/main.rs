use std::fs::File;

use std::io::Write;

fn main() {
    let student_details: Vec<(&str, &str, &str)> = vec![
        ("John Doe", "PAU1010", "Computer Science"),
        ("Jane Smith", "PAU1015", "Mass Communication"),
        ("Chris Mark", "PAU1022", "Accounting"),
        ("Maria Bello", "PAU1030", "Law"),
    ];

    let mut file_content = String::from("PAU-SMIS Student Personal Details\n-----------------------------------\n");
    println!("\n--- PAU Student Management Information System (PAU-SMIS) ---");

    for (name, id, department) in student_details {
        
        println!("Student Name: {}, ID: {}, Department: {}", name, id, department);

        let student_record = format!(
            "Name: {}\nID: {}\nDepartment: {}\n\n", 
            name, id, department
        );

    file_content.push_str(&student_record);
    }

    let mut file = File::create("pau_student_details.txt")
        .expect("Failed to create the file."); // expect() returns a custom message if an error occurs [16].

    // The write_all() method attempts to write the entire content (converted to bytes) to the file [5].
    file.write_all(file_content.as_bytes())
        .expect("Failed to write student data to the file.");

    println!("\nSuccessfully recorded and saved student details to 'pau_student_details.txt'.");
}
