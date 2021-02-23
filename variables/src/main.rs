// const STACKSIZE: u32 = 65536; // good way to declare const along with its type

// const STACKSIZE = 65536; // not ok

fn main() {
    /*
    let x = 5;
    x = 6; // not ok
    */
    let x = 5;
    println!("x is: {}", x); // ok to overwrite, just like redifining tuples in Python
    let x = x + 1;
    println!("x is: {}", x);
    let x = x * 2;
    println!("x is: {}", x);

    let some_spaces = "     ";
    println!("someSpaces is: {}", some_spaces);
    let some_spaces = some_spaces.len();
    println!("someSpaces is: {}", some_spaces);

    /*
    let mut someSpaces = "     ";
    println!("someSpaces is: {}", someSpaces);
    someSpaces = someSpaces.len(); // not ok - cannot change variable type
    println!("someSpaces is: {}", someSpaces);
    */

    // not ok - many types are possible, need to specify
    // let g = "42".parse().expect("Not a number");

    // integer types: i = signed, u = unsigned, 8 to 128 bits + arch (machine dependent)

    let _sample_byte = b'a'; // b for byte, only ok with u8
    // let sampleByte: u128 = b'a'; -- compiler error

    // integer overflow - compiler error if debug mode, 2's comp wrap if production mode

    // let sampleFloat: f64 = 30 * 1.0 / 7; // compiler error - cannot do {int} * {float}
    let integer_division = 30 / 7;
    let float_division   = 30.0 / 7.0;
    println!("integerDivision is: {}, floatDivision is: {}", integer_division, float_division);

    /*
    let tuple: (i32, f64, u8) = (500, 6.4, 1); // don't have to be the same type
    tuple.0 = 1000; // compiler error - this tuple is immutable
    */

    let mut tuple: (i32, f64, u8) = (500, 6.4, 1);
    // println!("Second item in tuple is: ", tuple.1); // compiler error
    println!("Second item in tuple is: {}", tuple.1);
    tuple.0 = 1000; // we can make mutable tuples though
    
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];
    println!("Second month is: {}", months[1]);
    // months[1] = "random"; // again compiler error. `let mut months` solves it

    // init array with datatype AND SIZE
    let one_to_five: [i32; 5] = [1, 2, 3, 4, 5];

    // init array with number 3 repeated 5 times
    let five_threes = [3; 5];
}
