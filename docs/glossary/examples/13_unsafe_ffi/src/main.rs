fn main() {
    let mut x = 10;
    let r = &mut x as *mut i32; // raw pointer
    unsafe {
        *r += 5; // unsafe deref, but safe here because r is valid
        println!("x = {}", *r);
    }
}
