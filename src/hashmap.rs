use std:: collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name); //score is an Option type
    match score {   
        Some(&value) => println!("Score for {} team is {}", team_name, value),
        None => println!("No score found for {}", team_name),
    }
}