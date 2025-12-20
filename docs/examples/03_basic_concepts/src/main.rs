fn square(n: i32) -> i32 {
    n * n
}

fn main() {
    // 変数と可変性
    let x = 5; // 不変
    let mut y = 6; // 可変
    y += 1;
    let y = y; // shadowing

    println!("x = {}, y = {}", x, y);

    // 関数
    println!("square(4) = {}", square(4));
}
