fn takes_ownership(s: String) {
    println!("I took: {}", s);
}

fn makes_copy(i: i32) {
    println!("i = {}", i);
}

fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone(); // copy value if we want to keep using s1
    takes_ownership(s2);

    let x = 5;
    makes_copy(x); // i32はCopyトレイトを持つ
    println!("still can use x = {}", x);
}
