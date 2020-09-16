use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::time::Duration;

fn main() {
    // echo_number();
    message_passing();
}

fn echo_number() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1))
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1))
    }

    handle.join().unwrap();
}

fn message_passing() {
    let (tx, rx): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    thread::spawn(move || {
        for i in 1..4 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_millis(500))
        }
    });

    for msg in rx {
        println!("Message from other thread: {}", msg);
    }

    // handle.join().unwrap();
}
