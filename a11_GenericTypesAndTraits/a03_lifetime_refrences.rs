
// there are .................................3........................... rules to be followed to while giving lifetime

 //the first rule, which specifies that each parameter gets its own lifetime.

fn first_word<'a>(s: &'a str) -> &str {
 s
}


 //if there is exactly one input lifetime parameter, that lifetime is assigned to 
 // all output lifetime parameters: 
 fn first_words<'a>(s: &'a str) -> &'a str {
    s
 }


//The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a 
//method, the lifetime of self is assigned to all output lifetime parameters.










fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
// fn longest(x: &str, y: &str) -> &str {

// here it throws a error because rust doesn't know wether
// x and y that are refrenced as a slice wether  x will have a refrence in return or 
// or y will have a refrence 

// these are lifetime annotations they are used to make relationship between the 
// variables

// &i32        // a reference
// &'a i32     // a reference with an explicit lifetime
// &'a mut i32 // a mutable reference with an explicit lifetime
struct ImportantExcerpt<'a> {
    part: &'a str,
}



fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {result}");


       let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();
    let i = ImportantExcerpt {
        part: first_sentence,// here we have used lifetime so that to tile compiler novel doesnt go out of scope untill 
        // struct goes out of scope
    
    };
    let s: &'static str = "I have a static lifetime.";// here we have used static as it has a lifetime of a program
}