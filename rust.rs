fn main() {
    println!("hello world!");

    let mut s = String::from("hello");
    let immus1 = &s;
    let immus2 = &s;
    let immus3 = &s;
    println!("immus1 holds: {}", *immus1);
    println!("immus2 holds: {}", *immus2);
    println!("immus3 holds: {}", *immus3);
    s = take_ownership(s);
    println!("got ownership of string {} back", s); // compiler error: value moved
}

fn take_ownership(some_s: String) -> String {
    println!("took ownership of string {}", some_s);
    some_s // return statement w/o 'return' keyword
}