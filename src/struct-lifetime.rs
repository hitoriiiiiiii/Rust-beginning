struct User {
    name: &str,
}

fn main() {
    let name = String::from("Prarthana");
    let user = User {
        name: &name
    };
    println!("User's name is: {}", user.name);
}