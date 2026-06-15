// a channel is a general programming concept where message is sent from one thread to another
// it has two parts transmitter and reciever
use std::sync::mpsc;
use std::thread;
// we can create new channel using mpsc::channel which is multiple producer and single consume
//in rust standard library mpsc can have like multiple produces but only single reciever
fn main(){
    let (tx,rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();// here tx has one function to send which has two Result return value 
       // println("the value in send is {val}")
      // the above line will cause error because ownership is tranferred to send function
    });
    let recieved = rx.recv().unwrap();// it has two methods recv and try_recv 
    //recv returns result <T,E> ok if the message is availabe and error if isnt and keeps the main thread on and will wait until the value comes down
    //try_recv immediately returns ok or error
    println!("Got: recieved {recieved}");

}