use std::fs::File; 

use std::io::Write; 

fn main() {
    let file_content = format!(
        "Nigerian Breweries Product Categories:\n\n\
        Category: Lager\n\
        - 33 Export\n\
        - Desperados\n\
        - Goldberg\n\
        - Gulder\n\
        - Heineken\n\
        - Star\n\n\
        Category: Stout\n\
        - Legend\n\
        - Turbo King\n\
        - Williams\n\n\
        Category: Non-Alcoholic\n\
        - Maltina\n\
        - Amstel Malta\n\
        - Malta Gold\n\
        - Fayrouz\n"
    );

    let mut file = File::create("nb_drink_categories.txt")
        .expect("Failed to create the file: nb_drink_categories.txt");

    file.write_all(file_content.as_bytes())
        .expect("Failed to write product category data to the file.");

     println!("Successfully saved all high-quality drink categories to 'nb_drink_categories.txt'.");
}
