//unlike tupple and struct the collection that is vector, string and hash map the size is not 
// to be unknown during the compile time and it is stored in heap 

fn main(){

// the first type is vector represented by Vec<T> (not vect<T>)
// vector allows you to store multiple data values in a single data structure
// Vec<T> type can store data of any types that is generic

let d: Vec<i32> = Vec::new(); // here you created a vector with i32 type 

let g = vec![1,2,3]; // vec! macro initialized the vector with i32 type as the elements in it were 1,2,3



//........................Updating a vector.................................
let mut l = Vec::new();
l.push(5);
l.push(6);
l.push(7);
l.push(9);

//........................Reading a vector............................
let v = vec![1,2,3,4,5];

let third: &i32 = &v[2];
println!("the third element is {third}"); // use this when you are sure that the index is valid
// because if you enter v[7] it will become invalid

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("the third element is {third}"),
    None => println!("the third element is none"),
    //while if we talk about this case in this case the rust won't panic as it can handle the case when vectors index is out of bound
}

//..................................iterating over a loop..................................
let z = vec![100,32,57];
for i in &z {
    println!("{i}");
}   

let mut z_mut = vec![100,32,57]; // Create a separate mutable vector
for i in &mut z_mut {
    *i += 50; // Use dereference operator * to modify the value
    println!("{i}");
}   
//...................................USING AN ENUM TO STORE MULTIPLE TYPES.........................
enum SpreadsheetExcel{
    Int(i32),
    Float(f64),
    Text(String),
}
let row = vec![//rust needs to know what type of vector it is to allocate memory in heap
    SpreadsheetExcel::Int(24),
    SpreadsheetExcel::Float(32.5),
    SpreadsheetExcel::Text(String::from("hi, What's up?")),
];
//Like any other struct, a vector is freed when it goes out of scope
}

