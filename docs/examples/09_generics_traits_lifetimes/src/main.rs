trait Summable { fn sum(&self) -> i32; }

struct Numbers(Vec<i32>);
impl Summable for Numbers { fn sum(&self) -> i32 { self.0.iter().sum() } }

fn print_sum<T: Summable>(v: &T) { println!("sum = {}", v.sum()); }

fn main() {
    let v = Numbers(vec![1,2,3]);
    print_sum(&v);
}
