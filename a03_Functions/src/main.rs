fn main() {// function name should be in lower case
    println!("Hello, world!");

    another_function(5,'h');
    //let z=y=6// it acts as a statement so wont execute
    // instead this should be
    let y = {
        let x = 3;
        x + 1// here there is no semicolon as adding semicoln will turn this into a statemnet which wont return a value
    };
    println!("The value of y is: {y}");

    let x = five();
    println!("The value of x is: {x}");

}
   



fn five() -> i32 {// here i32 specifies that funciton returns something
    5
}


fn another_function(x:i32,unit_label:char) {
    println!("Another function{x}{unit_label}");
}
