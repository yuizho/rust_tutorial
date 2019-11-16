use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // moveキーワードを使用して、vの所有権を奪う
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
