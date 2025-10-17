fn main() {
    let name = String::from("Prarthana");
    let len = get_str_len(name);
    println!("The length of the string is: {}", len);
}

fn get_str_len(str: String) -> usize {
    str.chars().count()
}
