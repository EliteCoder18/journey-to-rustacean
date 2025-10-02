    use std::collections::HashMap;

//A hash map is a data structure that stores key–value pairs.
// You don’t look up values by an index (like in arrays or vectors), but by a key.
// Keys can be almost any type: string, integer, struct (if Hash + Eq traits are implemented).
// Creating a HashMap

fn main(){
let mut scores = HashMap::new();
scores.insert(String::from("Blue"),50);
scores.insert(String::from("Yellow"),100);

let team_name = String::from("blue");
let score = scores.get(&team_name).copied().unwrap_or(0);

for (key, value) in &scores{
    println!("{key}: {value}");
}

// in hashmap ownership is transfered when when we input strings
let field_name=String::from("Green");
let field_value=34;
scores.insert(field_name,field_value);// here field name ownership is transferred to hash map
// overwriting the value in this when we use hash map


//.....................................Updating value in a Hash Map.....
scores.insert(String::from("Blue"),100);// here we have overwrite the value using the same key


// here we have used the api "entry" for this in which we are checking wether the value fort he 
// key yellow exist if not then insert the value 
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
   

    println!("{scores:?}");
//....................... updating a value based on the previous value....................
let text= "hello my name is rustyy. my name is unique";

let mut map = HashMap::new();
for word in text.split_whitespace(){
    let count = map.entry(word).or_insert(0);
    *count+=1;
}
println!("{map:?}");
}