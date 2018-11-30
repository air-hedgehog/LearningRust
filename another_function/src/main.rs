use std::io;

fn main() {
    another_function(5, 6);

    //statements does not return values, so let x = 5 = 6
    //won't compile. To assign expression without creating
    //another function in Rust we using curly brackets:
    let x = {
        let y = 3;
        y + 1 //the last statement in expression is without semycolon
    };

    println!("value of x is: {}", x);

    println!("Enter the first value...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("_");

    let first_int: i32 = input.trim().parse().expect("_");

    println!("Enter the second value...");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("_");

    let second_int: i32 = input.trim().parse().expect("_");
    println!("Sum of {} and {} is: {}", first_int, second_int,
        sumarize(first_int, second_int));

    //'if' statement can be used in 'let' statements
    let condition = true;
    let number = if condition {
        5 //no semicolon
    } else {
        6 //no semicolon
    };

    //besides of 'loop' and 'while' we can use more
    //flexible iterator 'for'
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    //iteration also might be reverse:
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32, y: i32) {
    println!("{} {}", x, y);
}

fn sumarize(x: i32, y: i32) -> i32 {
    //return statement can be finished without semicolon
    //or with just expression without 'return' statement
    // 'x + y' || 'return x + y' || 'return x + y;' exact same thing here
    x + y
}
