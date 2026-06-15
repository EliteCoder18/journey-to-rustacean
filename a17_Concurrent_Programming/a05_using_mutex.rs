
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);//Atomic Refrence counter will work because Rc doesnt have concurrency primitives that it doesnt have send trait 
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

        *num += 1;//here m has a type mutex<i32> and after calling lock it gives out mutexguard wrapped in a lockedresult that can be handled by calling unwrap
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
// there are two traits 
//send:- this trait is used if ownership can be safely tranferred to another thread
//sync:- a shared reference can safely be used from multiple threads
