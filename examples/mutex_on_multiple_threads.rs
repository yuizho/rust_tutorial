use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    // 複数のThreadへmoveするのでArcで囲う
    // mutextは内部の可変性を提供するので値を変更できる
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += i;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}