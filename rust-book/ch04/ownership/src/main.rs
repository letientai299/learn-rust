fn main() {
    use std::any::type_name;
    println!("{}", type_name::<i32>());
    println!("{}", type_name(1));
}
