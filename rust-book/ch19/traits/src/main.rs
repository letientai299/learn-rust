trait Pilot { fn fly(&self); }

trait Wizard { fn fly(&self); }

struct Human {}

impl Pilot for Human { fn fly(&self) { println!("Pilot!") } }

impl Wizard for Human { fn fly(&self) { println!("Wizard!") } }

impl Human { fn fly(&self) { println!("Human!") } }

fn main() {
    let p = Human {};
    p.fly();
    Pilot::fly(&p);
    Wizard::fly(&p);
    vec!
}
