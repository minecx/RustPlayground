fn main() {
    println!("hello world!");

    let mut s = String::from("hello");
    s = take_ownership(s);
    println!("got ownership of string {} back", s); // compiler error: value moved
}

fn take_ownership(some_s: String) -> String {
    println!("took ownership of string {}", some_s);
    some_s // return statement w/o 'return' keyword
}