use std::thread;

fn main(){
    thread::handle(|| {
        for i in  0..5 {
            println!("Hello from thread: {}", i);
        }
    });

    for i in 0..50 {
        println!("Hello from main: {}", i);
    }
}