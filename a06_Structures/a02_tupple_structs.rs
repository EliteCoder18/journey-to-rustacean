#[derive(Debug)]//here it adds addition functionality to the below mentioned struct that is debug
struct Rect{
    width:u32,
    height:u32,
}

fn main(){
            let width1 = 30;
            let height1 = 50;
            println!("the area of the rectangle is {} square pixles",area(width1,height1));
        //.......instead
        let rect1=(30,50);
            println!("the area of the rectangle using tupple is {} square pixles",areat(rect1));


        let rect3=Rect{
            width:30,
            height:50,
        };
            println!("the area of rectangle using struct is {}",areas(&rect3));
           
           // println!("printing the structure of rectangle{}",rect3)//here this line would put a error
            // as println isnt aware wht part of struct to show 
           println!("by using debug we gonna print struct {rect3:?}");// here we also need to add debug functionality or 
           // a trait to the struct to use this print struct {rect3:?}

           println!("by using debug we gonna print struct {rect3:#?}");// more improved view for print

           //println macro takes only refrence to print something but here is a catch
           // debug macro takes ownership to print something and then returns the ownership
           dbg!(&rect3);

           let scale = 2;
           let rect5 = Rect {
               width: dbg!(30 * scale),//dbg also returns the value
               height: 50,
           };
       
           dbg!(&rect5);




}


fn area(height:u32,width:u32)->u32{
    height*width
}         //it seems unreadable as if we want to add so many charactersics to a rectangle the
        // it seems tedious as for a rect it takes tow seperate comp
        //......hence we use tupple struct

//......................tupple
fn areat(dimensions:(u32,u32))->u32{
    dimensions.0*dimensions.1
}       
        //here also dimensions never labels like what is width and what is height
        //although it has no effect here to calc area but still seems tedious


fn areas(rectangle:&Rect)->u32{//here we have taken immutable refrence ownership isnt taken
    //it is better rather than using 0 and 1 index
    rectangle.width*rectangle.height
}