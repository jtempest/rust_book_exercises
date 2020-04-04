use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    // no data
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap();

    // using data
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();

    // message passing
    let (tx, rx) = mpsc::channel();
    let make_thread = |name: &'static str| {
        let txc = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            for i in 1..10 {
                let val = format!("{}: {}", name, i);
                txc.send(val).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        });
    };

    make_thread("one");
    make_thread("two");
    make_thread("three");

    drop(tx);

    for received in rx {
        println!("Got: {}", received);
    }
}
