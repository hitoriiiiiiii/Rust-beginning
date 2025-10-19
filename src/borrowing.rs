fn main(){
    let s1 = String::from("hello");
    do_something(&s1);
    println!("s1 is still accessible: {}", s1);
}

fn do_something(s2: &String){
    println!("Inside do_something: {}", s2);  //s2 is a reference to s1 it is borrowed not owned
}