use std::fs::File;
use std::io;
use std::io::Read;

/* code with nested match

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },

            other_err => panic!("Problem opening the file: {:?}", other_err),
        },
    };

    println!("File: {:?}", f);
}
*/


// long version, introduced in chapter 9
fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username() -> Result<String, io::Error> {
    let mut s = String::new();
    match File::open("hello.txt").and_then(|mut f| f.read_to_string(&mut s)) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}


fn read() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("hello.txt")?
        .read_to_string(&mut s)?;
    Ok(s)
}

fn main() {
    println!("long version: {:?}", read_username_from_file());
    println!("short version: {:?}", read_username());
    println!("shortest: {:?}", read());
}
