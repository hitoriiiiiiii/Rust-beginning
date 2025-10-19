use std::fs::read_to_string;

fn main() {
    // Pass a String, not &str
    let result = read_from_file("example.txt".to_string());

    // Handle the Result properly
    match result {
        Ok(content) => println!("File content:\n{}", content),
        Err(err) => eprintln!("Error reading file: {}", err),
    }
}

fn read_from_file(file_path: String) -> Result<String, String> {
    match read_to_string(file_path) {
        Ok(content) => Ok(content),
        Err(err) => Err(err.to_string()),
    }
}
