fn main() {
    //'match' comparing every value until founds the match.
    //if we don't care about values other then 1, 3, 5, 7, we can use
    //placeholder '_' to process the other incoming values.
    //The () is just the unit value, so nothing will happen in the _ case.
    //As a result, we can say that we want to do nothing for all the possible values
    //that we don’t list before the _ placeholder.
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}
