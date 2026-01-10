//iterator is used to iterate over the data structure and return the some value and 
// return none when the sequence is over
// all iterators implement a loop 
//      pub trait Iterator {
//           type Item;

//          fn next(&mut self) -> Option<Self::Item>;
//      }


fn main(){
    let v1 = vec![1,2,3];
    let v1_iter = v1.iter();// here it takes a immutable refrence to the heap
    for val in v1_iter{
        println!("Got: {val}");
    }

//    println!("{v1_iter}");

   // methods that are defined on the iterator are called iterator adapter 
   let v1:Vec<i32> = vec![1,2,3];
   let v2:Vec<_> = v1.iter().map(|x|x+1).collect();
      assert_eq!(v2, vec![2, 3, 4]);
}
#[test]
fn iterator_demonstration(){
    let v1 = vec![1,2,3];
    let mut v1_iter=v1.iter();// here we have taken mutable iterator because it is going to change
    //as it consumes the value for v1_iter 
    assert_eq!(v1_iter.next(),Some(&1));
    assert_eq!(v1_iter.next(), Some(&2));
    assert_eq!(v1_iter.next(), Some(&3));
    assert_eq!(v1_iter.next(), None);

}

// the methods that call next are called consuming methods as they consume iter on calling
// For Example sum method

#[test]
   fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();// here the sum takes ownership of the v1_iter


        assert_eq!(total, 6);
    }
    
#[derive(PartialEq, Debug)]
struct Shoe{
    size: u32,
    style: String,
}
//into_iter is used to take ownership of the struc shoes
fn shoes_in_size(shoes:Vec<Shoe>, shoe_size:u32)->Vec<Shoe>{
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()// here shoe_size is captured from the environment 
    // and then the closure returns true or false and then filter returns value to collect a consuming iterator method
    // that collects what is returned into a vec
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}