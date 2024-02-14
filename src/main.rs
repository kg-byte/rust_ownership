fn main() {
    // s is not valid here - it's not yet declared
    let s = "hello"; // s is valid from this point forward
    // do stuff with s
    // this scope is now over and s is no longer valid
    // above is string literal &str:
    //  they're immutable
    //  not every value is known at compile time (hard codeds) because they're immutable
    // stored on stack


    //String
    let mut s = String::from(s); // requests memory it needs
    s.push_str(", Rust!"); // appends a literal to a String
    println!("{s}");
    variables_and_data_interact_with_move();
    return_values_and_scope();
    // String can be mutated but &str (literal) cannot
    // allocated on heapn order to support a mutable, growable piece of text
    // size unknown at compile time
    // memory must be requested from the memory allocator at runtime
    // we need a way of returning this momery to allocator when done

    // language with GC - done automatically 
    //   - in C++ deallocation resources at the eol
    //   - known as RAII (Resource Acquisition is Initialization)
    //   - similar to Rust

    // language with GC - manual (difficult)
    //   - one allocage with exact one free - more than one - bug
    //   - done too early - invalid var
    //   - done too late - waste of mamry
    // Rust:
    //   automatically call 'drop': memory is automatically returned once the variable goes out of scope
    //

    variables_and_data_interact_with_clone();
    ownership_and_functions();

    //referecens/borrowing etc
    reference_example_calculate_length(&s);
    reference_example_mutable(&mut s);
    println!("{s}");
    reference_mutable_twice_with_scope();
}

fn variables_and_data_interact_with_move() {
    let x = 5; // bind value 5 t to x
    let y = x; // bind value in x to y
    // integers are simple values with known fixed size
    // these two 5 values are pushed onto the stack
    // stack only 'copied' without calling clone
    println!("x = {}, y = {}", x, y);
    // the reason is types as such have a known size at compile time
    // and are stored entirely on the stack. so copies of actual values are quick to make
    // there’s no reason we would want to prevent x from being valid after we create the variable y
    // there's no difference between deep vs shallow copying
    // Rust has a special annotation called the Copy trait that we can place on types that are stored on the stack, as integers are (we’ll talk more about traits in Chapter 10). If a type implements the Copy trait, variables that use it do not move, but rather are trivially copied, making them still valid after assignment to another variable.
    // e.g.
    //       All the integer types, such as u32.
    //       The Boolean type, bool, with values true and false.
    //       All the floating-point types, such as f64.
    //       The character type, char.
    //       Tuples, if they only contain types that also implement Copy. For example, (i32, i32) implements Copy, but (i32, String) does not.
 
 
    let s1 = String::from("hello");
    // on stack s1 contains 
    //             a pointer to the memory that holds the content of string (on heap)
    //             a length (5)  - how much memory in bytes the contents of string are currently using
    //             a capacity (5) - how much memory in bytes that string has received from the allocator
    let s2 = s1;
    // we copy the pointer, the length, and the capaity athat are on the stack
    // we do not copy the data that are on the heap that the pointer refers to
    
    // to avoid double free/deallocation error now that both s1 and s2 point to
    // the same memory on heap that stores the string value
    // s1 goes out of scope once s2 is created
    // println!("{}, world", s1); value borrowed here after move
    println!("{} again, Rust!", s2);
    // move(not shallow copy because s1 is invalidated): copying the pointer, length and capacity
    // s1 was moved to s2
    // Rust will never automatcally create deep copies of your data
    // therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance
}

fn variables_and_data_interact_with_clone() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    // if we do want to deeply copy the heap data of the String
    // (not just the stcack data,) we can use 'clone'
    // when we call to clone - we know some arbitrary code
    // is being executed and that code may be expensive
    // visual indicator of something is going on
    println!("s1 = {}, s2 = {}", s1, s2);
}

fn ownership_and_functions() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into takes_ownership
                        // s is no longer available here
    
    let x = 5;        // x comes into scope
    makes_copy(x);    // x would move into the function,
                      // but i32 is Copy, so it's okay to still
                      // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.


fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.


fn return_values_and_scope() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 was moved, so nothing
  // happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("yours"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// This function takes a String and returns one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}

// references and borrowing - immutable
// a reference is like a pointer in that it's an address leading to the data stored
// the data is owned by some other variable. 
// unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference

fn reference_example_calculate_length(s: &String) -> usize {
     // s is a reference to a String
    s.len()
    // s.push_str("add more") // &s reference is not mutable
}// Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.

fn reference_example_mutable(s: &mut String) {
    // big restriction: if you have a mutable reference to a value, you cannot have other references
    // The restriction preventing multiple mutable references to the same data at the same time allows for mutation but in a very controlled fashion.
    // prevent data races at compile time! data race occurs when:
    //      - two or more pointers access the same data at the same time
    //      - at least one of the pointers is being used to write to the data
    //      - there's no mechanism being used to synchronize access to the data
    s.push_str("add more");
  }

  fn reference_mutable_twice_with_scope() {
    let mut s = String::from("hello");
    println!("s original: {s}");

    {
        let r1 = &mut s;
        r1.push_str(", world");
        println!("r1 mutated in scope: {r1}");

    } // r1 goes out of scope here, so we can make a new reference with no problems.
    println!("s after r1 mutation: {s}");
    
    let r2 = &mut s;
    r2.push_str("moreeeeee!");
    println!("r2 mutated out of scope: {r2}");
    println!("s after r2 mutation: {s}");

    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    // let r3 = &mut s; // big problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);

  }