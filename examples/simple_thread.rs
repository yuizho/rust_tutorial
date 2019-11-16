use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} fromthe spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // ここで結果を待ち受ける
    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}
