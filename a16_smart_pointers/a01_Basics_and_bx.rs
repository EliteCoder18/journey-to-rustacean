#[derive(Debug)]
enum List{
    Cons(i32,Box<List>),
    Nil,
}
use crate::List::{Cons,Nil};
use std::rc::Rc;
//box is a  straight forward smart pointer which is allowed to store data on heap and just the pointer on the stack
fn main(){
    let b =Box::new(5);
    println!("b={b}");
//-------------------------------CONS LISt=-----------------
//cons list is a datastructure fromt eh lisp programming lang
//pseudocode for cons list is (1,(2,(3,Nil)))//
//canonical name to denote a base class is a Nil

// enum List_without_box {
//     Cons(i32,List),
//     Nil
// }
    //let list_box = Cons(1, Cons(2, Cons(3,Nil)));

//compiler can not deteermine what is the size of the enum so it give error and suggestion 
//of indirection which means instead of storing the value above compiler should 
//just change a datastructure to store the value indirectly by storing a pointer to the value instead


let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3,Box::new(Nil))))));
println!("{:?}", list);

//....................Treating Smart Pointers like Regular Pointers..............................
let x=5;
let y= &x;
assert_eq!(5,x);
assert_eq!(5,*y);

let z=Box::new(x);
assert_eq!(5,*y);//using box as a reference it creates a instace that is a copy of the the x to which it points
//..........................creating our on smart pointer similar to box to understand about the deref trait........
struct MyBox<T>(T);
impl <T> MyBox<T>{
    fn new(x:T)->MyBox<T>{
        MyBox(x)
    }
}
let x=5;
let y=MyBox::new(x);
assert_eq!(5,x);
assert_eq!(5,*y);
use std::ops::Deref;
impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
}}
//deref also implements deref coercion like converting from one type to other automatically to satisfy the function
//.................................................Drop Trait.....................................................
struct CustomerSmartPointer{
    data:String,
}
impl Drop for CustomerSmartPointer{
    fn drop(&mut self){
        println!("Dropping CustomerPointer with data `{}`", self.data);

    }
};
    let c = CustomerSmartPointer{
        data: String::from("my stuff")
    };
    let d= CustomerSmartPointer{
        data: String::from ("bad stuff")
    };
//variablaes are dropped in reverse order of the creation 
//if you want to force to drop a value before the end of the scope you need to call a std::mem::drop function rust 
//doesnt let you call the drop function directly 
//c.drop() will cause error
drop(c);
drop(d);
println!("dropped before");
// drop can always be called once per variable

}
