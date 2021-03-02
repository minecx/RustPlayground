# RustPlayground

Bob's Playground with Rust. Updates are in chronological order (*not* reversed).

Update Feb 18th: mostly following [the official documentation](https://doc.rust-lang.org/book/)

## Feb 18th

Followed chapter 2 in the book, finished guessing game.

Liked:

- The `mut` keyword which allows me to control whether each individual variable is mutable or not.
- Auto type guessing but I can also manually enforce a datatype (like `let guess: u32`). Similar syntax with
Apple Swift.
- Pattern matching (likely from Haskell) and I can even do error handling from there!
- And the Cargo - package manager, builder and runner ALL IN ONE!

Disliked:

- Used `::` a lot - and my right little finger hurts
- Not necessarily disliked, but sometimes I can walk away without a `;` when there's only one line in a `{}` block
while adding more lines will generate compiler error. I think requiring `;` for every line within `{}` block could
relieve this confusion.

## Feb 22th

Followed chapter 3.1 & 3.2 in the book, learnt some datatypes in Rust.

Questions:

- I can't seem to declare a `const str` in global scope? E.g. if I do `const s: str = "something";` the compiler
will complain that it cannot determine the size of this variable at compile time.
- We use `tuple.index` to access items in tuples, but `array[index]` to access items in arrays. Very curious about
what's the point of differentiating these two?

Liked:

- Able to specify bits used to store integers and floats
- `char` type supports char from other languages, Chinese ,Japanese, Korean, emoji, you name it! But more on this
in later chapters and I could be wrong...
- Able to initialize arrays with datatype and **SIZE** at compile time! Useful compile time optimize I guess

Worth noting:

- `30/7` will perform integer division (truncating decimal points), `30.0/7.0` will perform float division
(4.28...), while `30.0/7` or `30/7.0` will generate compiler error saying cannot do `{Int}/{Float}` or vice versa.

## Fed 25

Finished chapter 3 about data types, functions and control flows (loop and
conditions).

Liked:

- Ternary operator follows a more intuitive format `if _condition_ {true}
  else {false}`.
- Similar to Apple Swift, need to specify input and output datatype when
  writing functions. Used `->` for return type in function.

Worth noting:

- The `loop` keyword is somewhat counterintuitive if Rust is not the first
  language you get into touch (most other languages like JavaScipt, Java,
  Python, C, C++, you name it) use `for` and `while` for loops while Rust use
  the literal `loop`.

To-do:
  
- Figure out how to write functions with multiple lines of calculation and then
  return some value though (since there is no `return` keyword in Rust)
- Be more familiar with passing strings as function parameters (cannot use
  strings directly since they are not of fixed size, so have to use pointers
  like in C/C++)

Random thoughts

- I heard the next chapter (Ownership) is the most unique and the hardest
  concept of Rust so wish me luck `:(`

## Feb 26

Ownership

- A variable is valid when it's scope is valid
- And becomes invalid once it's out of scope
- So that the memory is automatically returned when something is out of scope
- At the end of each scope, Rust adds a function for us called `drop` which
  essentially wipes all variables that **only** lives in this scope

Copying pointer values in Rust

```Rust
fn main() {
    let x = 5;
    let y = x;
    println!("x is: {}, y is: {}", x, y);
    // get 5 and 5

    let a = String::from("abc");
    let b = a;
    // NOT OK - "Value borrowed after move"
    println!("a is: {}, b is: {}", a, b);
}
```

With things on the heap, we don't want to simply copy over the content since
that'll be too slow (especially if the data is large), but we also don't want
multiple pointers to be pointing to the same position and cause thing like
multiple `free` operation in C.

\
&nbsp;

Different ways of handling `string` in Rust

- If not mutable (aka `String Literal`), hardcode directly into executable
- Else (aka `String` type) use pointer and allocate on heap

## Mar 1 (Feb 26 Cont'd)

The following stack-only data are *deep-copied* (ownership didn't get
transferred):

- Integers
- Boolean
- Floats
- Chars
- Tuples, if all of the data within the tuple are the 4 above types

`Copy` trait: we can still use `a` after `let a = something; let b = a;`
`Drop` trait: we cannot use `a` after `let a = something; let b = a;`

Returning in functions also transfers ownership! In the following program

``` Rust
fn main() {
  println!("hello world!");

  let mut s = String::from("hello");
  s = take_ownership(s);
  // without the return statement below this will be compiler error
  println!("got ownership of string {} back", s);
}

fn take_ownership(some_s: String) -> String {
  println!("took ownership of string {}", some_s);
  some_s // return statement w/o 'return' keyword
}
```

Similar to Python, it's possible to return multiple values with tuples.
