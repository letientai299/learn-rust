mod mess;

#[derive(Debug)]
struct MyPointer {
    data: String,
}

impl Drop for MyPointer {
    fn drop(&mut self) {
        println!("dropping data: {}", self.data);
    }
}

fn main() {
    let c = MyPointer { data: String::from("hello C") };
    let d = MyPointer { data: String::from("hello D") };
    drop(d);
    println!("c: {:?}", c);
}
