// it is a test directory if you dont want to write the code in your programme you can write your test here
// no need to mention #[cfg(test)]
// here if we make test in a subdirectory it wont run
// only lib crate can expose funtions that test crate can use we cant define functions in main.rs



// unit test test the indivisual funtions privately
// integeration test check that library as a whole works
// they use library's public api to test the code
use adder::add;
#[test]
fn it_adds(){
    let result = add(2,5);
    assert_eq!(result, 7);
}