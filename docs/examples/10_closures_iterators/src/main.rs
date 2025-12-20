fn main() {
    let v = vec![1,2,3,4,5];
    let processed: Vec<i32> = v.iter().map(|x| x * 2).filter(|x| x % 3 == 0).cloned().collect();
    println!("processed = {:?}", processed);
}
