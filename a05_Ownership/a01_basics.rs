fn main() {
    let s = String::from("hello");
    // "hello" is copied to the heap,
    // and s (on the stack) holds pointer + length + capacity.

    let mut j = "hello";
    // "hello" is a string literal (static memory),
    // j (on the stack) is a pointer (&str) to that static memory.

    let j = "world";//here first j is written in ROM and then 
    // a pointer is created and give to j 
    println!("{}", j);

//..............................memory allocation..........................................
    let s1 = String::from("hello");
    let s2 = s1;

  //  println!("{s1}, world!");


// here s2's pointer length and capacity is copied to s2 and after that s1 is invalid and now s2 refers to heap
// created by s1 and here it will show a error if the above statement is executed thuse here s1 is moved to s2



//...................................scope and assignment.....................................
let mut s = String::from("hello");
s = String::from("ahoy");

println!("{s}, world!");
//here firt a string is created hello and assigned to s and then another string is created ahoy and assigned
// to s thus calling a drop function to delete previous heap



//....................................cloning a data.........................................
let s1 = String::from("hello");
let s2 = s1.clone();

println!("s1 = {s1}, s2 = {s2}");
//here s1 creates a heap and s2 creates other heap so here both s1 and s2 are valid


let mut x = 5;
let mut y = x;
 y = 6;
println!("x = {x}, y = {y}");
//.....................................ownership and function..............................
let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.
  fn gives_ownership() -> String {             // gives_ownership will move its
    // return value into the function
    // that calls it

let some_string = String::from("yours"); // some_string comes into scope

some_string                              // some_string is returned and
    // moves out to the calling
    // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
             // scope

a_string  // a_string is returned and moves out to the calling function
}