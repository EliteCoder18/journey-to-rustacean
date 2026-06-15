use std::thread;
// now we will use move to take the ownership from the main thread and and pass it to the other thread

fn main(){
        let v = vec![1,2,3];

        let handle = thread::spawn(move||{
            println!("here is a vector{v:?}");
        }
        );
        // drop(v);
        // here if you dont use move keyword then it wont compile because rust doesnt know how long the ref is valid and drop will drop the refrence so under these scenarios it 
        //wont compile
        handle.join().unwrap();
}