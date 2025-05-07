fn first_word(s:&String)->usize{// takes reference of a string
    let bytes = s.as_bytes();// converts string into byte
    for(i,&items) in bytes.iter().enumerate(){//i stands for index items is the value at that index 
        // here iter stands for like each item value one by one and enumerate returns both index and value
        // one by one
        if items==b' ' {// here b is a byte converter for " "(space)
            return i;
        }
    }
    s.len()
}


fn first_word_using_slice(s:&String)->&str{
    let byte = s.as_bytes();
    for(i,&items) in byte.iter().enumerate(){
        if items==b' '{
            return &s[0..i];
        }
        
    }&s[..]
}


fn main(){
    let mut s = String::from("hello world");

    let word = first_word(&s); // word will get the value 5

    s.clear(); // this empties the String, making it equal to ""
println!("{word}");
    // `word` still has the value `5` here, but `s` no longer has any content
    // that we could meaningfully use with the value `5`, so `word` is now
    // totally invalid!
//..............................................SLICE.............................................
// slice is a refrence to a part of the string

let mut s = String::from("hello world");
let hello = &s[0..5];// here hello is a refrence to the some part of the string
let world = &s[6..11];// here hello is a refrence to the some part of the string
// here hello contains a pointer to the the 0th index and length of the string that is 5
//if here we have to store the value it should be one more than the last index

let mut wordd = first_word_using_slice(&s);
println!("{wordd}");

s.clear();//here as you can see after using slice wordd also changes as the wordd changes

}

