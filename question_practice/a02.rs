//Generate the nth Fibonacci number.
use std::io;
fn main(){
    println!("Fibonacc Number Generator");
    println!("Enter the value of n:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    let n: u32 = match input.trim().parse(){
        Ok(num)=>num,
        Err(_)=>{
            println!("Invalid input. please enter a number");
            return;
        }
    };
    let result = fibonacci(n);
    println!("the {n}th term of a fibonacci series is {result}");

}
fn fibonacci(n:u32)->u64{
    if n==0{
        return 0;
    }else if n==1{
        return 1;
    }
    let mut a = 0;
    let mut b = 1;
    for _ in 2..=n
    

    {   //here _ ➔ ignore the current loop number
                        // 2..=n ➔ Rust range from 2 up to and including n.
        let temp = a+b;
        a = b;
        b = temp;
    }

    b
}