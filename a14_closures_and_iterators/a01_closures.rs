

use std::thread;
use std::time::Duration;

enum ShirtColor{
    Red,
    Blue,
}

struct Inventory{
    shirts:Vec<ShirtColor>,
}

impl Inventory{
    fn giveaway(&self,user_preference:Option<ShirtColor>) -> ShirtColor{
        user_preference.unwrap_or_else(|| self.most_stocked())// here closure is used as if Option<T> returns some 
        // then do not run closure but if option<t> gives none it calls closure and runs most_stocked 

    }

    fn most_stocked(&self)->ShirtColor{
        let mut num_red = 0;
        let mut num_blue = 0;
        
        for color in &self.shirts{
            match color{
                ShirtColor::Red => num_red+=1,
                ShirtColor::Blue => num_blue+=1,
            }
          
        }
        if num_blue<num_red{
            ShirtColor::Red
        }else{
            ShirtColor::Blue
        }
    }
}
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
fn main(){
    let store = Inventory{
        shirts:vec![ShirtColor::Red,ShirtColor::Red,ShirtColor::Blue]
    };
    // let user_pref1=Some(ShirtColor::Red);
    // let giveaway1 = store.giveaway(user_pref1);
    // println!(
    //      "The user with prefrence{:?}gets {:?}",
    //     user_pref1,giveaway1
    // );

    //   let user_pref2=Some(ShirtColor::Red);
    // let giveaway2 = store.giveaway(user_pref2);
    // println!(
    //      "The user with prefrence{:?}gets {:?}",
    //     user_pref2,giveaway2
    // );
    //   let user_pref3=None;
    // let giveaway3 = store.giveaway(user_pref3);
    // println!(
    //    "The user with prefrence{:?}gets {:?}",
    //     user_pref3,giveaway3
    // );
    let expensive_closure = |num: u32| -> u32 {// here a closure is annotated and defined as if a function but is 
        // totally different as if function can be used as a api in other crates but this cant be done 
        // in this type
        // Secondly it is not necessary to annotate as the first line could be 
        //let expensive_closure = |num|{ no need to mention inputs and return as num 
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
    // here it means same 
                    let x:u32 = 4;
                    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
                let add_one_v2 = |x: u32| -> u32 { x + 1 };
                // let add_one_v3 = |x|             { x + 1 }; 
               //  let add_one_v4 = |x|               x + 1  ;
// the last two also also mean same but here you need to infer types as rust doesnt know what is the type for this
 let example_closure = |x| x;

    let s = example_closure(String::from("hello"));
 
    //let n = example_closure(5); // here this will give a error as closure associates a concrete value after the x is a String it cant be integer

//...................Three Ways of Capturing Ownership..........................................
//borrowing immutably 
//borrowing mutably
//taking ownership


              let list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let only_borrows = || println!("From closure: {list:?}");

    println!("Before calling closure: {list:?}");
    only_borrows();
    println!("After calling closure: {list:?}");

    //making a mutable borrow
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {list:?}");

    let mut borrows_mutably = || list.push(7);
    borrows_mutably();
    println!("After calling closure: {list:?}");


    // forcing ownership from environment via move keyword
    println!("Before defining closure: {list:?}");

    thread::spawn(move || println!("From thread: {list:?}"))
        .join()//firstly spawned thread is not allowed any sort of refrence
        // rust says a thread must own the data as borrwing would be dangerous
        .unwrap();




        //there are three traits the closure can implement which depends on what the closure 
        // does with the captured value those are....
        //fnonce - can be applied only once.A closure that moves the capt value out of the closure

        //fnmut-that dont move captured value but might mutate it. Can be called more than once.
        //fn - applies to closure that dont move the captured value. dont mutate the env. these closure can be called more than once
        
         let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);//this is used to return r.width in the list and this is used by sort_by_key rust's internal function multiple times
    // so it is a fn 

    println!("{list:#?}");

    //..............................................here the code below wont run because the value's ownership is transferred to sort_operations
    // this is a example of the fn mut
    // let mut sort_operations = vec![];                
    // let value = String::from("closure called");

    // list.sort_by_key(|r| {
    //     sort_operations.push(value.clone());
    //     r.width
    // });                                                      
    // println!("{list:#?}"); 
    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{list:#?}, sorted in {num_sort_operations} operations");                                  


}

// impl<T> Option<T> {
//     pub fn unwrap_or_else<F>(self, f: F) -> T// here we are setting the function of unwrap_or_else to be called only once 
//     where
//         F: FnOnce() -> T// here we used a generic type where we set it to be called only once and return T
//     {
//         match self {
//             Some(x) => x,
//             None => f(),
//         }
//     }
// }
