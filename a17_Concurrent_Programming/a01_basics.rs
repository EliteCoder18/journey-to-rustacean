//Concurrent Programming is where different parts of the code execute independently
//parallell prgramming where the whole programme is executed at the same time
// in most os the programmes are run in a process and os can manage multiple processes at once 
// the feautures that run these indivisual parts are called threads
// the rust std lib uses 1:1 model of thread implementation

use std::thread;
use std::time::Duration;

fn main (){
  let handle =  thread::spawn(||{// to create a new thread we call thread::spawn and pass it to closure
        for i in 1..10{
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(1));

        }
    });
    for i in 1..5{
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
//note when the main thread gets over no matter where the spawned thread is it will halt and this is what happnes in the above example 
//we can fix the premauture handling of the spawned thread by saving the return type in a variable the return type of thread::spqwn function is JoinHandle<T>
        handle.join().unwrap();
        //just comment the above line to see the difference
// calling handle stops the main thread that is blocking the main thread until the main thread appears just comment out the 



}