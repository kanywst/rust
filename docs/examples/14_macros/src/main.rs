macro_rules! say_hello {
    () => { println!("hello from macro"); };
}

fn main() {
    say_hello!();
}
