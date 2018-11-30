//unsigned 32 bit, which can be only positive
const Y: u32 = 5;

fn main() {
    let x = Y;
    println!("The value of x is : {}", x);
    let x = x + 1;
    println!("The value of x is : {}", x);

    //floating point types: f32, f64. Default is f64.
    let y = 2.0; //f64
    let x: f32 = 2.0; //f32
    println!("2.0 f64 is: {}", y);
    println!("2.0 f32 is: {}", x);

    //tuple
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //to get items from defined tuple we use destructuring:
    let (tup_x, tup_y, tup_z) = tup;
    println!("The value of tup_x is {}", tup_x);
    println!("The value of tup_y is {}", tup_y);
    println!("The value of tup_z is {}", tup_z);
    //or we can access dirrectly to element by using "." followed
    //by the index:
    println!("The value of tup_x is: {}", tup.0);

    //arrays always fixed size (unlike the vector)
    //first parameter is type, second is length:
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    //items is accessable by index:
    println!("second element of arr is: {}", arr[1]);
}
