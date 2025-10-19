/*fn longest(a: String, b: String) -> String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = String::from("xyz");

    let result = longest(string1, string2);
    println!("The longest string is: {}", result);
}*/

//lifetime annotation references
fn main(){
    let ans;

    let str1 = String::from("longer string");
    {
        let str2 = String::from("short");
        ans = longest(&str1, &str2);
    }
}

fn longest<'a>(a: &'a String, b: &'a String) -> &'a String {
    if a.len() > b.len() {
        a
    } else {
        b
    }
}