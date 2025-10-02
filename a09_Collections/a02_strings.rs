fn main(){
    let mut s = String::new();// we can create a string using this line
    let data = "Rohnit";// we can also create a string using below to string
    let mut s = data.to_string();// converts &str to String
    let mut data1 = String::from("Hello "); // Must be String, not &str
    
    data1.push_str(&s); // push_str takes &str, so use &s
    println!("{data1}");
    println!("{s}")
}
