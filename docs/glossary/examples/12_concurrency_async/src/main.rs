use std::sync::{Arc, Mutex};
use std::thread;
use futures::executor::block_on;

async fn say_async() { println!("Hello from async"); }

fn main() {
    // thread example
    let n = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..4 {
        let n = Arc::clone(&n);
        handles.push(thread::spawn(move || {
            let mut num = n.lock().unwrap();
            *num += 1;
        }));
    }
    for h in handles { h.join().unwrap(); }
    println!("count = {}", *n.lock().unwrap());

    // simple async example using futures crate
    block_on(say_async());
}
