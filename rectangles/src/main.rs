fn main() {
    //creating rectangle tuple:
    //let rect1 = (30, 50);

    //creating rectangle struct:
    let rect1 = Rectangle { width: 30, height: 50 };
    let rect2 = Rectangle { width: 10, height: 40 };
    let rect3 = Rectangle { width: 60, height: 45 };
    // println!(
    //     "The area of the rectangle is {} square pixels.",
    //     //area(rect1)
    //     area(&rect1)
    // );

    println!(
           "The area of the rectangle is {} square pixels.",
           rect1.area()
    );

    println!("{:?}", rect1);

    println!("Can rect1 hold rect2: {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3: {}", rect1.can_hold(&rect3));
    println!("square with side of 54 px == {:?}", Rectangle::square(54));
}

//receiving tuple as an argument:
// fn area(dimensions: (u32, u32)) -> u32 {
//     dimensions.0 * dimensions.1
// }


//to print sruct we need to add derive:
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

//Each struct is allowed to have multiple 'impl' blocks
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    //asociated function isn't method. It doesn't receive 'self' as a parameter:
    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size }
    }
}
