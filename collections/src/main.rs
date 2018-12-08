use std::collections::HashMap;

fn main() {
    //vector declarations

    //empty vector:
    let mut v: Vec<i32> = Vec::new();

    //predefined vector (containing data type is unnecessary if we predefined values)
    v = vec![1, 2, 3];

    //adding elements to vector:

    v.push(4);
    v.push(5);
    v.push(6);

    //pop() method turn the last element and removing it from the vector:
    match v.pop() {
        Some(value) => {
            println!("popped element is {}", value);
        }
        None => {
            //nothing was in vector
        }
    }

    //reading vector elements:
    let third = &v[3];
    //will cause the program panic if referenced to non-existing element
    //also, this borrow is immutable, which prevent adding new elements, because adding
    //new elements to vector might require allocate more memory and copy the old elements
    //to the new space. So compiler doesn't allow us to add ellements after immutable borrow.

    match v.get(3) {
        Some(&element) => {
            //return value at index 100 if exists
            println!("vector returns {}", element);
        }
        None => {
            //get() method returns 'None' without panicking
            //if number isn't exists or to large
            println!("vector returns None");
        }
    }

    //iterating through vector items:
    for i in &v {
        println!("i == {}", i);
    }
    let mut newVector = vec![100, 32, 57];
    //iterating through mutable references in order to make changes to elements:
    for i in &mut newVector {
        // To change the value that the mutable reference refers to, we have to use the dereference
        // operator (*) to get to the value in i before we can use the += operator .
        *i += 50;
    }

    //to store different data-types in vector we can use Enum:
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    //creating String:
    let mut s = String::new();
    let data = "initial contents";
    s = data.to_string();
    // the method also works on a literal directly:
    s = "initial contents".to_string();
    //We can also use the function String::from to create a String from a string literal.
    s = String::from("initial contents");

    //appending string:
    let mut s = String::from("foo");
    s.push_str("bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    //'push_str' taking slice of s2 and not the actual value,
    //so s2 is still valid
    s1.push_str(s2);
    println!("s2 is {}", s2);
    //The push method takes a single character as a parameter and adds it to the String.
    s1.push('l');
    //concentration with '+':

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // Note s1 has been moved here and can no longer be used

    //formating string:
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3); //format! macro doesn't take ownership
                                             //of any of it's parameters

    //indexing through the string, to get speciffic char is
    //not allowed in Rust. we just can iterate through string
    //using chars() or bytes() metod:
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() {
        println!("{}", b);
    }
    // valid Unicode scalar values may be made up of more than 1 byte

    //hasMaps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    //we can collect items from two vectorslet teams  = vec![String::from("Blue"), String::from("Yellow")];
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    //it will iterate until the shortest vector length.
    let mut scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    // to check if hasMap contains the value, associated with speciffic key
    //we can call 'entry' method:
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    //modify value in hashMap if it exists:
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
}
