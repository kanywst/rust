use std::pin::Pin;

fn main() {
    let b = Box::new(5);
    let p = Pin::new(b);
    // Pin guarantees the Box's pointee won't be moved via the Pin handle.
    println!("pinned: {}", *p);
}
