use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let a = Rc::new(RefCell::new(5));
    {
        let mut v = a.borrow_mut();
        *v += 1;
    }
    println!("value = {}", a.borrow());

    let b = Box::new(10);
    println!("boxed = {}", b);
}
