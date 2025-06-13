// enum is used where you just need to use only one field 

enum Ipaddr{
    V4,
    V6,//here it nevers store data
}

enum IpAddr{
    V4(String),
    V6(String),// here enum can also be used to store data 
}

enum IpAddrNew{// rust can store multiple data types in one field
    V4(u8,u8,u8,u8),
    V6(String),
}
// rust doesnt have a null feature you cant declare a variable as null
// but you have a option to use enum instead of that



// here t means that inside enum where ever t comes it means all types
// here option t enum is so usefull it is included in prelude means you do not 
// need to mention "option "before using enum 


// enum Option<T>{
//     None,
//     Some(T),
// }



fn main(){


// here we can create instace with enum
let four = Ipaddr::V4;
let six = Ipaddr::V6;


let home = IpAddr::V6(String::from("127.0.0.1"));
let loopback = IpAddr::V4(String::from("::1"));


// here enum stores multiple data types
let home1 = IpAddrNew::V4(127,4,3,1);
let loopback1 = IpAddrNew::V6(String::from("::6"));


let some_string = Some(String::from("rahul"));
let some_number = Some(12);
let absent_number: Option<i32> = None;


   let x: i8 = 5;
    //let y: Option<i8> = Some(5);

   // let sum = x + y;
    // this code wont work as y is not mentioned for the case y is none

}
