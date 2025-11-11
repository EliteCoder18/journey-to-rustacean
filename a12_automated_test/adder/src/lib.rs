//some CLI tools
//cargo test -- --test-thread=1          to stop parallelism and run test one by one
//cargo test -- --show-output            to print the output of each test that is successfull
//cargo test -- --nameofthetest          if you want to print particular test then write name of hte test
//                                       supposingly we want to ignore any particular test we can write #[ignore] before any particular test in the code
//cargo test -- --ignored                and if we want to run the ignored test than we can used this command
//cargo test -- --include-ignored        if you want to run all the test then you can use this command it include even the ignored

// there are two types of test
// unit test and integeration test
// unit test are more likely testing one model more likely at a time
// Integeration test are like using public intergace and exercising multiple modules at a time
// unit test can use private functions that are part of programme like below add_two is private but it's still being tested
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
 fn add_two(a: u64)->u64{
    a+2
}
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}


pub struct Guess {
    value: i32,
}

// impl Guess {
//     pub fn new(value: i32) -> Guess {
//         if value < 1 || value > 100 {
//             panic!("Guess value must be between 1 and 100, got {value}.");
//         }

//         Guess { value }
//     }
// }

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {value}."
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {value}."
            );
        }

        Guess { value }
    }
}

#[cfg(test)]// it tells rust to use cargo test to compile the code not to compile using cargo build
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    // #[test]
    // fn another(){
    // panic!("Make another test fail");

    // }
   #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
        // assert_eq! is used to prove equality 
        //assert_ne! is used to prove inequality
        assert_eq!(larger.can_hold(&smaller),true);
        assert_ne!(larger.can_hold(&smaller),false);

        

    }
    //     #[test]
    // fn greeting_contains_name() {
    //     let result = greeting("Carol");
    //     assert!(
    //         result.contains("Carol"),
    //         "Greeting did not contain name, value was `{result}`"
    //     );
    // }// here if we want to customize the reason for failure of the test

    //assert! macro is used when you want to ensure that certain condition in a macro is for sure true
   // should panic is used when you want to confirm that the test failed

    #[test]
    #[should_panic(expected = "less than or equal to 100")]//if you want to print a customized message if the test fails

    fn greater_than_100() {
        Guess::new(200);
    }




    
}

