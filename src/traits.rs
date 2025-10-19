trait Summary {
    fn summarize(&self) -> String;
}

struct User {
    name: String,
    age: u32,
}

impl Summary for User {
    fn summarize(&self) -> String {
        return format!("{} is {} years old.", self.name, self.age);
    }
}

struct Fix;
impl Fix for User {}
impl Summary for User {}

fn main() {
    let user = User {
        name: String::from("Prarthana"),
        age: 20,
    };
    println!("{}", user.summarize());
    let f = Fix;
    notify(f);
}

fn notify(u: impl Summary){
    println!("{}", u.summarize());
}