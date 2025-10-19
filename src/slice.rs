//With the string passed by value, the function takes ownership of it

/*fn main() {
    let name = String::from("hello world");
    let ans = first_word(name);
    println!("The first word is: {}", ans);
}

fn first_word(s: String) -> String {
    let mut ans = String::new();
    for ch in s.chars() {
        if ch == ' ' {
            break;
        } else {
            ans.push(ch);
        }
    }
    ans
}*/

//With the string slice passed by reference, the function borrows it

fn main(){
    let mut word = String::from("hello world");
    let word2 = find_first_word(&word);
    println!("The first word is: {}", word);    
    println!("The first word is: {}", word2);

}

fn find_first_word(word: &String) -> &str{
    let mut index = 0;
    for (_, i) in word.chars().enumerate(){
        if i == ' '{
            break;
        }
        index = index += 1;
    }
    return &word[0..index];
}