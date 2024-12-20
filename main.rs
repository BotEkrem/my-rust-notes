// An attribute to hide warnings for unused code.
#![allow(dead_code)]

// a module for formatting and printing Strings.
use std::fmt;

// Importing a local file
mod another_file_for_import;

fn main() {
    println!("Hello, world!");

    // Using a function from a different local file
    another_file_for_import::function_from_another_file();

    let formatted_string: String = format!("{} days", 31);
    // You can also directly use these formats to printing functions...
    println!("{}", formatted_string);

    // Like this:
    println!("{} Ekrem", "Bot");

    // Also you can print a string to stderr (for errors)
    eprintln!("{} Ekrem", "Bot");

    /*
        Scalar Types
            Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
            Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
            Floating point: f32, f64
            char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
            bool either true or false
            The unit type (), whose only possible value is an empty tuple: ()
        Compound Types
            Arrays like [1, 2, 3]
            Tuples like (1, true)
    */

    // For operators and symbols, you can visit: https://doc.rust-lang.org/book/appendix-02-operators.html

    // A basic struct
    #[derive(Debug)] /*
     derive(Debug) asks the compiler to auto-generate a suitable implementation of the Debug trait.
     (https://doc.rust-lang.org/std/fmt/trait.Debug.html)
    */
    struct Person {
        name: String,
        age: u8,
    }

    // A unit struct
    struct Unit;

    // A tuple struct
    struct Pair(i32, f32);

    // A struct with two fields
    struct PointTest {
        x: f32,
        y: f32,
    }

    // Structs can be reused as fields of another struct
    struct Rectangle {
        // A rectangle can be specified by where the top left and bottom right
        // corners are in space.
        top_left: PointTest,
        bottom_right: PointTest,
    }

    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // Print debug struct
    println!("{:?}", peter);

    // Instantiate a `Point`
    let point: PointTest = PointTest { x: 5.2, y: 0.4 };
    let another_point: PointTest = PointTest { x: 10.3, y: 0.2 };

    // Access the fields of the point
    println!("point coordinates: ({}, {})", point.x, point.y);

    // Make a new point by using struct update syntax to use the fields of our
    // other one
    let bottom_right = PointTest { x: 10.3, ..another_point };

    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // Destructure the point using a `let` binding
    let PointTest { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // struct instantiation is an expression too
        top_left: PointTest { x: left_edge, y: top_edge },
        bottom_right,
    };

    // Instantiate a unit struct
    let _unit = Unit;

    // Enums

    enum WebEvent {
        // An `enum` variant may either be `unit-like`,
        PageLoad,
        PageUnload,
        // like tuple structs,
        KeyPress(char),
        Paste(String),
        // or c-like structures.
        Click { x: i64, y: i64 },
    }

    // Type aliases

    enum VeryVerboseEnumOfThingsToDoWithNumbers {
        Add,
        Subtract,
    }

    // Creates a type alias
    type Operations = VeryVerboseEnumOfThingsToDoWithNumbers;

    let _x = Operations::Add;

    // Implementing

    impl VeryVerboseEnumOfThingsToDoWithNumbers {
        fn run(&self, x: i32, y: i32) -> i32 {
            match self {
                Self::Add => x + y,
                Self::Subtract => x - y,
            }
        }
    }

    let add = Operations::Add.run(23, 24);
    println!("{}", add);

    /*
    use:
        The use declaration can be used so manual scoping isn't needed
    */

    use crate::Stage::{Beginner, Advanced};
    // Automatically `use` each name inside `Role`.
    use crate::Role::*;

    // Equivalent to `Stage::Beginner`.
    let stage = Beginner;
    // Equivalent to `Role::Student`.
    let role = Student;

    match stage {
        // Note the lack of scoping because of the explicit `use` above.
        Beginner => println!("Beginners are starting their learning journey!"),
        Advanced => println!("Advanced learners are mastering their subjects..."),
    }

    match role {
        // Note again the lack of scoping.
        Student => println!("Students are acquiring knowledge!"),
        Teacher => println!("Teachers are spreading knowledge!"),
    }

    // enum with implicit discriminator (starts at 0)
    enum NumberEnum {
        Zero,
        One,
        Two,
    }

    // enum with explicit discriminator
    enum Color {
        Red = 0xff0000,
        Green = 0x00ff00,
        Blue = 0x0000ff,
    }

    // `enums` can be cast as integers.
    println!("zero is {}", NumberEnum::Zero as i32);
    println!("one is {}", NumberEnum::One as i32);

    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Blue as i32);

    // Constants

    static LANGUAGE: &str = "Rust";
    const THRESHOLD: i32 = 10;

    println!("This is {}", LANGUAGE);
    println!("The threshold is {}", THRESHOLD);

    // The compiler warns about unused variable bindings; these warnings can
    // be silenced by prefixing the variable name with an underscore
    let _unused_variable = 3u32;

    /*
    Mutability:
        Variable bindings are immutable by default, but this can be overridden using the mut modifier.
    */

    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    // Ok
    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // Casting examples

    let decimal = 65.4321_f32;
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    // Literals examples

    // Suffixed literals, their types are known at initialization
    let x = 1u8;
    let y = 2u32;
    let z = 3f32;

    // Unsuffixed literals, their types depend on how they are used
    let i = 1;
    let f = 1.0;

    // `size_of_val` returns the size of a variable in bytes
    println!("size of `x` in bytes: {}", std::mem::size_of_val(&x));
    println!("size of `y` in bytes: {}", std::mem::size_of_val(&y));
    println!("size of `z` in bytes: {}", std::mem::size_of_val(&z));
    println!("size of `i` in bytes: {}", std::mem::size_of_val(&i));
    println!("size of `f` in bytes: {}", std::mem::size_of_val(&f));

    // Inference

    // Because of the annotation, the compiler knows that `elem` has type u8.
    let elem = 5u8;

    // Create an empty vector (a growable array).
    let mut vec = Vec::new();
    // At this point the compiler doesn't know the exact type of `vec`, it
    // just knows that it's a vector of something (`Vec<_>`).

    // Insert `elem` in the vector.
    vec.push(elem);
    // Aha! Now the compiler knows that `vec` is a vector of `u8`s (`Vec<u8>`)

    println!("{:?}", vec);

    // Aliasing

    type NanoSecond = u64;
    type Inch = u64;
    type U64 = u64;

    // `NanoSecond` = `Inch` = `U64` = `u64`.
    let nanoseconds: NanoSecond = 5 as u64;
    let inches: Inch = 2 as U64;

    // Note that type aliases *don't* provide any extra type safety, because
    // aliases are *not* new types
    println!("{} nanoseconds + {} inches = {} unit?",
             nanoseconds,
             inches,
             nanoseconds + inches);

    // Conversion

    let my_str = "hello";
    let _my_string = String::from(my_str);

    // For custom struct (from & into)

    #[derive(Debug)]
    struct Number {
        value: i32,
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item }
        }
    }

    // impl Into<Number> for i32 {
    //     fn into(self) -> Number {
    //         Number { value: self }
    //     }
    // }

    let num = Number::from(30);
    println!("My number is {:?}", num);

    // let int = 5;
    // let num: Number = int.into();
    // println!("My number is {:?}", num);

    // TryFrom & TryInto

    #[derive(Debug, PartialEq)]
    struct EvenNumber(i32);

    impl TryFrom<i32> for EvenNumber {
        type Error = ();

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value % 2 == 0 {
                Ok(EvenNumber(value))
            } else {
                Err(())
            }
        }
    }

    // TryFrom

    assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    assert_eq!(EvenNumber::try_from(5), Err(()));

    // TryInto

    let result: Result<EvenNumber, ()> = 8i32.try_into();
    assert_eq!(result, Ok(EvenNumber(8)));
    let result: Result<EvenNumber, ()> = 5i32.try_into();
    assert_eq!(result, Err(()));

    // Strings

    struct Circle {
        radius: i32
    }

    impl fmt::Display for Circle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Circle of radius {}", self.radius)
        }
    }

    let circle = Circle { radius: 6 };
    println!("{}", circle.to_string());

    // Parsing a string

    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();

    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);

    // Expressions

    let x = 5u32;

    let y = {
        let x_squared = x * x;
        let x_cube = x_squared * x;

        // This expression will be assigned to `y`
        x_cube + x_squared + x
    };

    println!("y is {:?}", y);

    // Flow of control

    // if/else

    let n = 5;

    if n < 0 {
        print!("{} is negative", n);
    } else if n > 0 {
        print!("{} is positive", n);
    } else {
        print!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");

            // This expression returns an `i32`.
            10 * n
        } else {
            println!(", and is a big number, halve the number");

            // This expression must return an `i32` as well.
            n / 2
        };
    //   ^ Don't forget to put a semicolon here! All `let` bindings need it.

    println!("{} -> {}", n, big_n);

    // Loop

    let mut count = 0u32;

    println!("Let's count until infinity!");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            // Skip the rest of this iteration
            continue;
        }

        println!("{}", count);

        if count == 5 {
            println!("OK, that's enough");

            // Exit this loop
            break;
        }
    }

    // Nested

    'outer: loop {
        println!("Entered the outer loop");

        'inner: loop {
            println!("Entered the inner loop");

            // This would break only the inner loop
            //break;

            // This breaks the outer loop
            break 'outer;
        }

        println!("This point will never be reached");
    }

    // Returning from loops

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);

    // While

    let mut n = 1;

    // Loop while `n` is less than 20
    while n < 20 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }

        // Increment counter
        n += 1;
    }

    // For and range

    for n in 1..20 {
        if n % 15 == 0 {
            println!("fizzbuzz");
        } else if n % 3 == 0 {
            println!("fizz");
        } else if n % 5 == 0 {
            println!("buzz");
        } else {
            println!("{}", n);
        }
    }

    // Match (switch)

    let number = 20;

    println!("Tell me about {}", number);
    match number {
        // Match a single value
        1 => println!("One!"),
        // Match several values
        2 | 3 | 5 | 7 | 11 => println!("This is a prime"),
        // Match an inclusive range
        13..=19 => println!("A teen"),
        // Handle the rest of cases
        _ => println!("Ain't special"),
    }

    // https://doc.rust-lang.org/rust-by-example/flow_control/match/guard.html

    // if let

    // All have type `Option<i32>`
    let number = Some(7);
    let letter: Option<i32> = None;

    // The `if let` construct reads: "if `let` destructures `number` into
    // `Some(i)`, evaluate the block (`{}`).
    if let Some(i) = number {
        println!("Matched {:?}!", i);
    }

    // If you need to specify a failure, use an else:
    if let Some(i) = letter {
        println!("Matched {:?}!", i);
    } else {
        // Destructure failed. Change to the failure case.
        println!("Didn't match a number. Let's go with a letter!");
    }

    // let else
    let s = "a b c d";

    let mut it = s.split(' ');
    let (Some(count_str), Some(item)) = (it.next(), it.next()) else {
        panic!("Can't segment count item pair: '{s}'");
    };

    println!("{}, {}", count_str, item);

    // while let

    let mut optional = Some(0);

    // This reads: "while `let` destructures `optional` into
    // `Some(i)`, evaluate the block (`{}`). Else `break`.
    while let Some(i) = optional {
        if i > 9 {
            println!("Greater than 9, quit!");
            optional = None;
        } else {
            println!("`i` is `{:?}`. Try again.", i);
            optional = Some(i + 1);
        }
        // ^ Less rightward drift and doesn't require
        // explicitly handling the failing case.
    }

    println!("Optional: {:?}", optional);

    // Functions

    fizzbuzz_to(20);

    // Associated functions & Methods

    struct Test {
        name: String
    }

    impl Test {
        // a method
        fn say_name(&self) {
            println!("Hello, my name is {}", self.name)
        }

        // an associated function
        fn greet_someone(name: &str) {
            println!("Hello {}, how are you?", name)
        }
    }

    let test = Test { name: "Ekrem".parse().unwrap() };
    test.say_name();

    Test::greet_someone("John Doe");

    // Closures

    let sum_closure = |number: i32| -> i32 { number * 2 };

    println!("sum_closure: {}", sum_closure(20));

    // https://doc.rust-lang.org/rust-by-example/fn/closures.html

    // Higher order functions: https://doc.rust-lang.org/rust-by-example/fn/hof.html

    // super and self: https://doc.rust-lang.org/rust-by-example/mod/super.html

    // File hierarchy: https://doc.rust-lang.org/rust-by-example/mod/split.html

    // Crates: https://doc.rust-lang.org/rust-by-example/crates.html

    // Cargo: https://doc.rust-lang.org/rust-by-example/cargo.html

    // Tests: https://doc.rust-lang.org/rust-by-example/cargo/test.html

    // Attributes: https://doc.rust-lang.org/rust-by-example/attribute.html

    // !!!!!!!!!!!!!!!!!!!! https://doc.rust-lang.org/rust-by-example/scope.html !!!!!!!!!!!!!!!!!!!!!!!!!!!

    // !!!!!!!!!!!!!!!!!!!! RAII (Resource Acquisition Is Initialization), https://en.wikipedia.org/wiki/Resource_acquisition_is_initialization !!!!!!!!!!!!!!!!!!!!

    /*
    Variables in Rust do more than just hold data in the stack: they also own resources, e.g. Box<T> owns memory in the heap. Rust enforces RAII (Resource Acquisition Is Initialization),
    so whenever an object goes out of scope, its destructor is called and its owned resources are freed.

    This behavior shields against resource leak bugs, so you'll never have to manually free memory or worry about memory leaks again!
    */

    fn create_box() {
        // Allocate an integer on the heap
        let _box1 = Box::new(3i32);

        // `_box1` is destroyed here, and memory gets freed
    }

    fn boxes_init() {
        // Allocate an integer on the heap
        let _box2 = Box::new(5i32);

        // A nested scope:
        {
            // Allocate an integer on the heap
            let _box3 = Box::new(4i32);

            // `_box3` is destroyed here, and memory gets freed
        }

        // Creating lots of boxes just for fun
        // There's no need to manually free memory!
        for _ in 0u32..1_000 {
            create_box();
        }

        // `_box2` is destroyed here, and memory gets freed
    }

    boxes_init();

    // !!!!!!!!!!!!!!!!!!!! Ownership and moves !!!!!!!!!!!!!!!!!!!!

    /*
    Because variables are in charge of freeing their own resources, resources can only have one owner. This prevents resources from being freed more than once. Note that not all variables
    own resources (e.g. references).

    When doing assignments (let x = y) or passing function arguments by value (foo(x)), the ownership of the resources is transferred. In Rust-speak, this is known as a move.

    After moving resources, the previous owner can no longer be used. This avoids creating dangling pointers.
     */

    fn destroy_box(c: Box<i32>) {
        println!("Destroying a box that contains {}", c);

        // `c` is destroyed and the memory freed
    }

    // _Stack_ allocated integer
    let x = 5u32;

    // *Copy* `x` into `y` - no resources are moved
    let y = x;

    // Both values can be independently used
    println!("x is {}, and y is {}", x, y);

    // `a` is a pointer to a _heap_ allocated integer
    let a = Box::new(5i32);

    println!("a contains: {}", a);

    // *Move* `a` into `b`
    let b = a;
    // The pointer address of `a` is copied (not the data) into `b`.
    // Both are now pointers to the same heap allocated data, but
    // `b` now owns it.

    // Error! `a` can no longer access the data, because it no longer owns the
    // heap memory
    // println!("a contains: {}", a);
    // TODO ^ Try uncommenting this line

    // This function takes ownership of the heap allocated memory from `b`
    destroy_box(b);

    // Since the heap memory has been freed at this point, this action would
    // result in dereferencing freed memory, but it's forbidden by the compiler
    // Error! Same reason as the previous Error
    //println!("b contains: {}", b);
    // TODO ^ Try uncommenting this line

    // !!!!!!!!!!!!!!!!!!!! Ownership and moves - Mutability !!!!!!!!!!!!!!!!!!!!

    let immutable_box = Box::new(5u32);

    println!("immutable_box contains {}", immutable_box);

    // Mutability error
    //*immutable_box = 4;

    // *Move* the box, changing the ownership (and mutability)
    let mut mutable_box = immutable_box;

    println!("mutable_box contains {}", mutable_box);

    // Modify the contents of the box
    *mutable_box = 4;

    println!("mutable_box now contains {}", mutable_box);

    // https://doc.rust-lang.org/rust-by-example/scope/move/partial_move.html

    // !!!!!!!!!!!!!!!!!!!! Borrowing !!!!!!!!!!!!!!!!!!!!
    // https://doc.rust-lang.org/rust-by-example/scope/borrow.html

    fn eat_box_i32(boxed_i32: Box<i32>) {
        println!("Destroying box that contains {}", boxed_i32);
    }

    // This function borrows an i32
    fn borrow_i32(borrowed_i32: &i32) {
        println!("This int is: {}", borrowed_i32);
    }

    // Create a boxed i32 in the heap, and a i32 on the stack
    // Remember: numbers can have arbitrary underscores added for readability
    // 5_i32 is the same as 5i32
    let boxed_i32 = Box::new(5_i32);
    let stacked_i32 = 6_i32;

    // Borrow the contents of the box. Ownership is not taken,
    // so the contents can be borrowed again.
    borrow_i32(&boxed_i32);
    borrow_i32(&stacked_i32);

    {
        // Take a reference to the data contained inside the box
        let _ref_to_i32: &i32 = &boxed_i32;

        // Error!
        // Can't destroy `boxed_i32` while the inner value is borrowed later in scope.
        // eat_box_i32(boxed_i32);
        // TODO ^ Try uncommenting this line

        // Attempt to borrow `_ref_to_i32` after inner value is destroyed
        borrow_i32(_ref_to_i32);
        // `_ref_to_i32` goes out of scope and is no longer borrowed.
    }

    // `boxed_i32` can now give up ownership to `eat_box_i32` and be destroyed
    eat_box_i32(boxed_i32);

    // !!!!!!!!!!!!!!!!!!!! https://doc.rust-lang.org/rust-by-example/scope/borrow/mut.html !!!!!!!!!!!!!!!!!!!!

    /*
    Data can be immutably borrowed any number of times, but while immutably borrowed, the original data can't be mutably borrowed. On the other hand, only one mutable borrow is allowed
    at a time. The original data can be borrowed again only after the mutable reference has been used for the last time.
    */

    // https://doc.rust-lang.org/rust-by-example/scope/borrow/ref.html
    // When doing pattern matching or destructuring via the let binding, the ref keyword can be used to take references to the fields of a struct/tuple.

    struct Point { x: i32, y: i32 }
    let c = 'Q';

    // A `ref` borrow on the left side of an assignment is equivalent to
    // an `&` borrow on the right side.
    let ref ref_c1 = c;
    let ref_c2 = &c;

    println!("ref_c1 equals ref_c2: {}", *ref_c1 == *ref_c2);

    // !!!!!!!!!!!!!!!!!!!! https://doc.rust-lang.org/rust-by-example/scope/lifetime.html !!!!!!!!!!!!!!!!!!!!
    /*
    A lifetime is a construct the compiler (or more specifically, its borrow checker) uses to ensure all borrows are valid. Specifically, a variable's lifetime begins when it is
    created and ends when it is destroyed. While lifetimes and scopes are often referred to together, they are not the same.
    */

    // !!!!!!!!!!!!!!!!!!!! Traits https://doc.rust-lang.org/rust-by-example/trait.html

    struct Sheep { naked: bool, name: &'static str }

    trait Animal {
        // Associated function signature; `Self` refers to the implementor type.
        fn new(name: &'static str) -> Self;

        // Method signatures; these will return a string.
        fn name(&self) -> &'static str;
        fn noise(&self) -> &'static str;

        // Traits can provide default method definitions.
        fn talk(&self) {
            println!("{} says {}", self.name(), self.noise());
        }
    }

    impl Sheep {
        fn is_naked(&self) -> bool {
            self.naked
        }

        fn shear(&mut self) {
            if self.is_naked() {
                // Implementor methods can use the implementor's trait methods.
                println!("{} is already naked...", self.name());
            } else {
                println!("{} gets a haircut!", self.name);

                self.naked = true;
            }
        }
    }

    // Implement the `Animal` trait for `Sheep`.
    impl Animal for Sheep {
        // `Self` is the implementor type: `Sheep`.
        fn new(name: &'static str) -> Sheep {
            Sheep { name: name, naked: false }
        }

        fn name(&self) -> &'static str {
            self.name
        }

        fn noise(&self) -> &'static str {
            if self.is_naked() {
                "baaaaah?"
            } else {
                "baaaaah!"
            }
        }

        // Default trait methods can be overridden.
        fn talk(&self) {
            // For example, we can add some quiet contemplation.
            println!("{} pauses briefly... {}", self.name, self.noise());
        }
    }

    let mut dolly: Sheep = Animal::new("Dolly");
    // TODO ^ Try removing the type annotations.

    dolly.talk();
    dolly.shear();
    dolly.talk();

    // !!!!!!!!!!!!!!!!!!!! https://doc.rust-lang.org/rust-by-example/trait/derive.html
    // !!!!!!!!!!!!!!!!!!!! https://doc.rust-lang.org/rust-by-example/trait/dyn.html

    // !!!!! https://doc.rust-lang.org/rust-by-example/trait/drop.html
    // !!!!! https://doc.rust-lang.org/rust-by-example/trait/clone.html
    // !!!!! https://doc.rust-lang.org/rust-by-example/trait/supertraits.html

    // !!!!!!!!!!!!!!!!!!!! https://doc.rust-lang.org/rust-by-example/macros.html

    // https://doc.rust-lang.org/rust-by-example/testing.html
    // https://doc.rust-lang.org/rust-by-example/unsafe.html
}


// Sample enum for "use"
enum Stage {
    Beginner,
    Advanced,
}

// Sample enum for "use"
enum Role {
    Student,
    Teacher,
}

// Functions

// Function that returns a boolean value
fn is_divisible_by(lhs: u32, rhs: u32) -> bool {
    // Corner case, early return
    if rhs == 0 {
        return false;
    }

    // This is an expression, the `return` keyword is not necessary here
    lhs % rhs == 0
}

// Functions that "don't" return a value, actually return the unit type `()`
fn fizzbuzz(n: u32) -> () {
    if is_divisible_by(n, 15) {
        println!("fizzbuzz");
    } else if is_divisible_by(n, 3) {
        println!("fizz");
    } else if is_divisible_by(n, 5) {
        println!("buzz");
    } else {
        println!("{}", n);
    }
}

// When a function returns `()`, the return type can be omitted from the
// signature
fn fizzbuzz_to(n: u32) {
    for n in 1..=n {
        fizzbuzz(n);
    }
}
