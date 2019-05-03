fn main() {
    // ----- LOOP ---------------------------------
    // the counter must be mutable to be updated!
    let mut counter = 0;

    // result doesn't need to be mutable because we're only
    // assigning it once
    let result = loop {
        counter += 1;

        if counter == 10 {
            // this is how you return a value from a loop
            break counter * 2; 
        }
    }; // semicolon ends the "let result" statement

    println!("The result is {}", result);


    // ----- WHILE LOOP ---------------------------
    // loops over the elements on an array;
    // this implementation is slow and error-prone:
    // could panic if index is out of bounds, and also
    // has to do a check on every element
    let array = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", array[index]);

        index = index + 1;
    }


    // ----- FOR LOOP -----------------------------
    // a for loop implementation for looping over elements
    // in an array is better
    for element in array.iter() {
        println!("the value is: {}", element);
    }

    // ----- RANGE --------------------------------
    // the first number is inclusive, the last exclusive
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");
}
