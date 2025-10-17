struct User {
    first_name: String,
    last_name: String,
    age: u32,
}

fn main(){
    let user = User {
        first_name: String::from("Prarthana"),
        last_name: String::from("Gade"),
        age: 20,    
    };

    printIn!("{} {} is {} years old.", user.first_name, user.last_name, user.age);  
}