enum List{
    Cons(i32,Rc<List>),
    Nil,
}
use crate::List::{Cons,Nil};
use std::rc::Rc;
fn main(){


//-----------------------------------------------Refrence Counter (Rc)-----------------------------------------
//we use rc when we want the the code to be shared by multiple part of the program and we dont know which part 
//will finish first 
// struct list {
// Cons(i32,bx<list>),
// Nil,
// }
// let a = Cons(5, Box::new(Cons(10,Box::new(Nil))));
// let b = Cons(3, Box::new(a));
// let c = Cons(4, Box::new(a));
// this will cause a error because ownership is first transferred from a to be of a and then c is asking for ownership
// of a which doesn not exist now so it becomes invalid
let a = Rc::new(Cons(5,Rc::new(Cons(10,Rc::new(Nil)))));
println!("count after creating a = {}",Rc::strong_count(&a));
let b = Cons(3,Rc::clone(&a));
println!("count after creating b = {}",Rc::strong_count(&a));
{
let c = Cons(4,Rc::clone(&a));
println!("count after creating c = {}", Rc::strong_count(&a));
}
println!("count after creating c = {}", Rc::strong_count(&a));
//rc::clone(&a) just creates a refrence clone for refrence count and it doesn not create a deep clone that other clone does
//rc::strong_count is used to print the refrence count
//when you call Rc::Downgrade the weak pointer increases by 1 instead of strong pointer
//they are like non ownership refrence and they wont effect anything and become 0 when strong count becomes 0
// the diff is weak count doesnt need to be 0 for rc<t> to get out of scope
}