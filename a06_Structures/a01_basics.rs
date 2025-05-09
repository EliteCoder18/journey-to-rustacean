struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count:u64,
}
//tupple structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);


// Unit like Struct
stuct Alwaysequal;
fn main(){
    let mut user1 = User{
        active: true,
        username:String::from("someusername1234"),
        email:String::from("user@gmail.com"),
        sign_in_count:1,
    };
    user1.email=String::from("someone@gmail.com");
  
//..................................updating a structure........................
// let user2 = User {
//     active: user1.active,
//     username: user1.username,
//     email: String::from("another@example.com"),    ------------->  INSTEAD OF THIS WE CAN USE BELOW
//     sign_in_count: user1.sign_in_count,                                        WRITEN CODE
// };

// same code written but with shortcut

    let user3 = User {
        email: String::from("another@example.com"),
        ..user1//here user 2 inherits user 1 and updates the email for user 2 only
    };
    println!("{}",user1.email)//Here we can only print email but we can't print username as that value was moved
    // to user 3 so user1.username is invalid but 


// ..................................tupple structs.................................. 
// they are usefull when there is a need to create a tupple and make it different form others

let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
// ..................................creating instance for structure using function.................................. 


fn build_user(email:String,username:String)->User{
    User{active:true,
    username:username,// it seeems tedious if we write username
    // and email again there is a shortcut below
    email:email,
    sign_in_count:1,}

}
// as there are same parameters we can write email as only email


fn build_users(email:String,username:String)->User{
    User{active:true,
    username,
    email,
    sign_in_count:1,}

}
