struct User {
  username: String, // name followed by type of field 
  email: String,
  active: bool,
}

struct rgb_color (i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area (&self) -> u32 {
        return self.width * self.height
    }
}

fn main() {
    println!("Hello, world!");
    let mut u1 = User { // immutable. use `let mut u1` to make the whole struct mutable
      username: String::from("xyz123"),
      email: String::from("example1@email.com"),
      active: true,
    };

    // println!("{}", u1.username); // should give us "xyz123"
    // println!("{}", u1.active); // should give us "true"

    let u2 = User {
      username: String::from("abc456"),
      ..u1 // copy the rest of the field from u1
      // can no longer refer to email field from u1 - shallow copy
    };

    // println!("u1 email is {}, u2 email is {}.", u1.email, u2.email);
    println!("u1 username is {}", u1.username);
    println!("u2 email is {}", u2.email);

    let r1 = Rectangle {height: 50, width: 30};
    println!("r1 is {:?}", r1); // debug print
    println!("[FORMATTED] r1 is {:#?}", r1); // debug print
    println!("area of r1 is {}", r1.area());
}

fn build_user(username: String, email: String) -> User {
  User {
    username, // doable if function parameter has the same name as the field
    email,
    active: true,
  }
}

