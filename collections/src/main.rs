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
}
