fn main(){
    //..................................................referencing.....................................
    let s1 = String::from("hello");

    let len = calculate_length(&s1);// here a reference of s1 is made

    println!("The length of '{s1}' is {len}.");
    let s = String::from("hello");

   // change(&s);

    let mut s = String::from("hello");
    change(&mut s);

// you cant create two mutable refrences for the same string variable
    // let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;

    // println!("{}, {}", r1, r2);
}

fn calculate_length(s: &String) -> usize {// here s has the values of s1 only but the ownership is still in the 
    // hands of s1
    s.len()

}
fn change(some_string: & mut String) {
   some_string.push_str(", world");    
     // this statement wont work as the string it borrowed isnt mutable

println!("{some_string}");

}
