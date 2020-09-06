struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rec = Rectangle {
        width: 30,
        height: 50,
    };

    println!("The area of the rectable is {} square pixels.", area(&rec));
    println!("rec: {:#?}", rec);
}

fn area(rec: &Rectangle) -> u32 {
    rec.width * rec.height
}
