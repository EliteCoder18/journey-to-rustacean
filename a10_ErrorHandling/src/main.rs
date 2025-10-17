use std::fs::File;
use std::io::ErrorKind;
use std::io::{self,Read};

fn main() {
   // here the panic is called as a macro 
    // panic!("Hello, world!");
    // let v = vec![1,2,4];
    // v[99];
    // we can write RUST_BACKTRACE=1 cargo run that is any value other than 0 to cargo run to check if 
    //the program runs properly


//     enum Result<T, E> {
//     Ok(T), here t and e are generic types
//     Err(E),
// }
    let greeting_file_result = File::open("hello txt");

    let greeting_file = match greeting_file_result{
        Ok(file)=>file,
        Err(error)=>match error.kind(){
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc)=>fc,
                Err(e)=>panic!("error creating the file {e:?}"),
            },
            _=>{
                panic!("Problem creating the file: {error:?}")
            }
            
        }
    };

    //
        let greeting_file = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {error:?}");
            })
        } else {
            panic!("Problem opening the file: {error:?}");
        }
    });
        //....................................UNWRAP....................................

    // unwrap is similar to macro where if the result variant is ok unwrap will open what is inside ok
    // if its Err it will call panic!
  // let file = File::open("heello.txt").unwrap();// here it will call panic
    // which will say no such file or directory exist if the file does not exist

    //....................................EXPECT........................................
    // expect is similar to unwrap but in expect we can call customize the expect macro to return any value

    let fiile = File::open("hellow.txt").
    expect("hellow.txt should be included in this project");

   //....................................PROPOGATING THE ERROR ....................................
   //the propogating error exactly means that propogating the error to the calling code
   // after the function is being called supposingly error occurs in the function now sending the error
   // the calling code .. as the calling code might be having the logic to handle the error
 




}
  fn read_username_from_file() ->Result<String,io::Error>{
    let username_file_result = File::open("hello.txt");
    let mut username_file = match username_file_result{
        Ok(file) => file,
        Err(er) => return Err(er)
    };
    let mut username = String::new();
    match username_file.read_to_string(&mut username){
    Ok(_) => Ok(username),
    Err(e) => Err(e),
    }
// here at last we need not to write return as Err(e)is at the last part of the programme
   }





// ....................................?(reading question mark operator)....................................
fn read_username_from_file_using_question_mark()->Result<String,io::Error>{
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}