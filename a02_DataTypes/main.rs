fn main() {


//...................for int....................
    // Length	Signed	Unsigned
    // 8-bit	i8	    u8
    // 16-bit	i16 	u16
    // 32-bit	i32	    u32
    // 64-bit	i64	    u64
    // 128-bit	i128	u128
    // arch	    isize	usize
    
    let z:i16 = 10;// int type

    let x = 2.0; // float   {f64} that is 64 bits by default
    let y: f32 = 3.0; // float {f32} that is 32 bits by default

    let t = true;
    
    let f: bool = false; // with explicit type annotation
        
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
    

    // .......................Tuple....................
    //A tupple can store values with different data type

    let tup: (i32, f64, u8) = (500,6.4,1);
    let (x, y, z) = tup;
    
   
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
            
    let months = ["January", "February", "March", "April", "May", "June", "July",
    "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];

    
}