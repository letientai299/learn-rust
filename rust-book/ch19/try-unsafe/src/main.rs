extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    let x = -5;
    unsafe {
        println!("abs(x) = {}", abs(x));
    }
}
