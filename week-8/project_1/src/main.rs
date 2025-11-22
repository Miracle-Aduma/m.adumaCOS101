use std::io;

#[derive(Debug)]
struct RoleInfo {
    role: &'static str,
    min_exp: i32,
    max_exp: i32,
    aps_level: &'static str,
}

fn main() {
    // APS TABLE
    let roles: Vec<RoleInfo> = vec![
        RoleInfo { role: "Paralegal",           min_exp: 0,  max_exp: 2,  aps_level: "APS 1-2" },
        RoleInfo { role: "Junior Associate",    min_exp: 3,  max_exp: 5,  aps_level: "APS 3-5" },
        RoleInfo { role: "Associate",           min_exp: 5,  max_exp: 8,  aps_level: "APS 5-8" },
        RoleInfo { role: "Senior Associate 1-2",min_exp: 8,  max_exp: 10, aps_level: "EL1 8-10" },
        RoleInfo { role: "Senior Associate 3-4",min_exp: 10, max_exp: 13, aps_level: "EL2 10-13" },
        RoleInfo { role: "Partner",             min_exp: 13, max_exp: 40, aps_level: "SES" },
        
        // Academic category
        RoleInfo { role: "Research Assistant",  min_exp: 3,  max_exp: 5,  aps_level: "APS 3-5" },
        RoleInfo { role: "PhD Candidate",       min_exp: 5,  max_exp: 8,  aps_level: "APS 5-8" },
        RoleInfo { role: "Post-Doc Researcher", min_exp: 8,  max_exp: 10, aps_level: "EL1 8-10" },
        RoleInfo { role: "Senior Lecturer",     min_exp: 10, max_exp: 13, aps_level: "EL2 10-13" },
        RoleInfo { role: "Dean",                min_exp: 13, max_exp: 40, aps_level: "SES" },
    ];

    // INPUT SECTION
    let mut staff_role_input = String::new();
    println!("Enter staff role (e.g., Associate):");
    io::stdin().read_line(&mut staff_role_input).expect("Failed to read input");
    let staff_role = staff_role_input.trim().to_lowercase();

    let mut exp_input = String::new();
    println!("Enter years of experience (whole number):");
    io::stdin().read_line(&mut exp_input).expect("Failed to read input");
    let experience: i32 = exp_input.trim().parse().expect("Invalid number");

 
    let mut determined_level = "Level Not Found";

    for entry in &roles {
        if entry.role.to_lowercase() == staff_role
            && experience >= entry.min_exp
            && experience <= entry.max_exp 
        {
            determined_level = entry.aps_level;
            break;
        }
    }

    
    println!("\n--- APS Level Validation ---");
    println!("Role: {}", staff_role);
    println!("Experience: {}", experience);
    println!("APS Level: {}", determined_level);
}