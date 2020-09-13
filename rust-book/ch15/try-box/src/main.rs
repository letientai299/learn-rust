use self::List::*;
use std::ops::Deref;

fn main() {
    let list = Cons(1, Box::new(Cons(2, Box::new(Nil))));
    println!("{:?}", list);

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    // the * in the code is different with the plain dereference that Rust does
    // behind the scene: *(y.deref()).
    assert_eq!(5, *y);
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}
