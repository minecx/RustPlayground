fn main() {
    println!("Hello, world!");
    sample_func(55, 127);

    // let x = (let y = 6); // compiler error - `let` does NOT return values

    let x = 100;
    let y = {
        let x = 1;
        x + 1 // adding `;` here will turn this line into 
    };
    println!("y is: {}.", y); // 2 since inside y = {}, local definition "shadows" global definition

    println!("{}", return_func(300));

    /*
    if x { // not ok - there are no truthy/falsy values in Rust
        println!("x is good.");
    }
    */

    if (x == 100) { // it's ok to () if conditions but usually they are not necessary (like Python)
        println!("x is 100.");
    }

    // ternary operator
    // condition ? val_if_true : val_if_false
    let z = if x > 100 { 200 } else { 50 };
    // again both must have the same type
    
    let mut exp = 1;
    let mut res: u128 = 1;
    let two_to_tenth = loop {
        res = res * 2;
        if exp == 10 {
            break res * 2
        }
    };
    println!("res is: {}", res);
}

fn sample_func(i1: i32, i2: u32) { // returns void
    println!("i1 is: {}, i2 is: {}.", i1, i2);
}

fn return_func(in1: i32) -> i32 { // similar to Apple Swift
    in1 + 10
}

fn inf_print(s: &str) {
    loop {
        println!("{}", s);
    }
}