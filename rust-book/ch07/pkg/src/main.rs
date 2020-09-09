mod front;
mod sub;

use front::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

fn main() {
    eat_at_restaurant();
    crate::sub::a::hello_a();
}
