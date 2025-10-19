use std::thread;

fn main() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 0..5 {
            println!("Hello from spawned thread: {}", i);
        }
    });

    // Main thread work
    for i in 0..50 {
        println!("Hello from main thread: {}", i);
    }

    // Wait for the spawned thread to finish
    handle.join().unwrap();
}
