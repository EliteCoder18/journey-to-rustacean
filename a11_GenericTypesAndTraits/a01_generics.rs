

fn largest<T: std::cmp::PartialOrd>(list:&[T]) -> &T{
    let mut largest = &list[0];
    for item in list {
        if item>largest{
            largest = item;
        }
    }largest
}



//here we needed to write two different functions if we had to change 
//the type like one for integer and other for charcter we needed two different types of functions
//but by using generics you need not to 
fn largest_i32(list: &[i32])->&i32{
    let mut largest = &list[0];
    
    for item in list {
        if item> largest {
            largest = item;
        }
    }
    largest
}

fn largest_char(list: &[char])->&char{
    let mut largest = &list[0];

    for item in list{
        if item>largest{
            largest = item;
        }
    }
    largest
}


struct Point<T> {
    x: T,
    y: T,
}
impl<T> Point<T> {
    // here if you want to define methods you can define it by puting
    // T right after impl 
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {//you can also define a method by using a particular type
// although the main struct is generic
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Points<X1,Y1>
{
    x:X1,
    y:Y1,
}
impl<X1, Y1> Points<X1, Y1> {
    fn mixup<X2, Y2>(self, other: Points<X2, Y2>) -> Points<X1, Y2> {
        Points{
            x: self.x,
            y: other.y,
        }
    }
}//here x1 and y1 are written after impl to demonstrate they go with the struct definition
// while x2 and y2 are written after mixup to demonstrate they go with the mixup definition
fn main(){
    let number_list = vec![34,50,25,100,65];

    let result = largest_i32(&number_list);
    println!("The largest number is {result}");

    let char_list = vec!['y', 'm', 'a' , 'q'];

    let result = largest_char(&char_list);
    println!("the largest char is {result}");


    let result = largest(&number_list);
    println!("The largest number is {result}");
    let result = largest(&char_list);
    println!("The largest number is {result}");

     let p = Point { x: 5, y: 10 };

    println!("p.x = {}", p.x());
}
// ............................the concept of monomorphization 
// i.e. if you use generics there is no lack in performance it acts same as if 
// you duplicated the code like 
// you wrote 
//              let float = Some(5.0);
//              let integer = Some(5);
// then compiler will make the code specific to 
//                  enum Option_i32 {
//                      Some(i32),
//                      None,
//                  }

//                  enum Option_f64 {
//                      Some(f64),
//                         None,
//                  }

//                  fn main() {
//                      let integer = Option_i32::Some(5);
//                      let float = Option_f64::Some(5.0);
//                  }
