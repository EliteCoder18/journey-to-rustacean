#[derive(Debug)]
struct Rectangle{
    width:u32,
    height:u32,
}

impl Rectangle{// impl means implementation here everything written after the start will be associated to only
    // rectangle struct
    fn area(&self)->u32{
        self.width*self.height
    }
    fn width(&self)->bool{
        self.width>0
    }
//........................................METHODS WITH MORE PARAMETERS......................................
fn can_hold(&self, other:&Rectangle)->bool{
    self.width>other.width && self.height>other.height
}
//..........................................USED AS A CONSTRUCTOR.......................
fn square(size:u32)->Self{// here self is a alias for what is written after impl
    Self{
    width:size,
    height:size,
    }
}


}
fn main(){
    let rect1=Rectangle{
        width:32,
        height:20,
    };
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    println!("The width of the rectangle is {} square pixels.", rect1.width);// here it is without parenthesis so it is being used as a field
    println!("The width of the rectangle is {} square pixels.", rect1.width());//here there is parenthesis
    // so here it is being used as width method call
    
    //........................................METHODS WITH MORE PARAMETERS......................................
    let rect2 = Rectangle{
        width:40,
        height:50,
    };
    let rect3 = Rectangle{
        width:30,
        height:20,
    };
    println!("can rectangle 2 hold rect 1 {}",rect2.can_hold(&rect1));
    println!("can rectangle 3 hold rect 2 {}",rect3.can_hold(&rect2));


    //..........................................USED AS A CONSTRUCTOR.......................

    let square = Rectangle::square(10);//here square is a associated function for Rectangle
    // double colon is used call a assciated function 
    println!("square is {:?}",square)

}