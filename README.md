# RustPlayground
Bob's Playground with Rust. 

Update Feb 18th: mostly following [the official documentation](https://doc.rust-lang.org/book/)

### Feb 18th
Followed chapter 2 in the book, finished guessing game.

Liked:
- The `mut` keyword which allows me to control whether each individual variable is mutable or not.
- Auto type guessing but I can also manually enforce a datatype (like `let guess: u32`). Similar syntax with 
Apple Swift.
- Pattern matching (likely from Haskell) and I can even do error handling from there!
- And the Cargo - package manager, builder and runner ALL IN ONE!

Disliked:
- Used `::` a lot... my right little finger hurts
- Not necessarily disliked, but sometimes I can walk away without a `;` when there's only one line in a `{}` block 
while adding more lines will generate compiler error.

### Feb 22th
Followed chapter 3.1 & 3.2 in the book, learnt some datatypes in Rust.

Questions:
- Can't seem to declare a `const str` in global scope? E.g. if I do `const s: str = "something";` the compiler
will complain that it cannot determine the size of this variable at compile time.

Liked:
- Able to specify bits used to store integers and floats
- `char` type supports char from other languages, Chinese ,Japanese, Korean, emoji, you name it! But more on this
in later chapters and I could be wrong...
- Able to initialize arrays with datatype and **SIZE** at compile time! Useful compile time optimize I guess

Worth noting:
- `30/7` will perform integer division (truncating decimal points), `30.0/7.0` will perform float division
(4.28...), while `30.0/7` or `30/7.0` will generate compiler error saying cannot do `{Int}/{Float}` or vice versa.
- We use `tuple.index` to access items in tuples, but `array[index]` to access items in arrays. Very curious about
what's the point of differentiating these two...