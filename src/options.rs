fn main(){
    let index = find_first_a(String::from("banana"));

    match index {
        Some(value) => printIn!("The index of first 'a' is: {}", value),
        None => printIn!("'a' not found in the string"),
    }
}

fn find_first_a(s: String) -> Option<usize> {
    for (index, char) in s.chars().enumerate(){
        if char == 'a' {
            return Some(index);
        }
    }
    return None;
}