pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for comp in self.components.iter() {
            comp.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("drawing Button: {:?}", self);
    }
}


#[derive(Debug)]
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("drawing SelectBox: {:?}", self);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 100,
                options: vec![
                    String::from("hello"),
                    String::from("world"),
                ],
            }),
            Box::new(Button {
                height: 200,
                width: 50,
                label: String::from("Rust"),
            })
        ],
    };

    screen.run();
}

