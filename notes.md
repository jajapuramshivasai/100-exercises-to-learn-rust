/*
# Comprehensive Rust Notes

This document covers the fundamental concepts of the Rust programming language, structured to follow a typical learning path.

---

## 1. The Basics

### 1.1. Variables and Mutability
- Variables are immutable by default.
- Use the `mut` keyword to make a variable mutable.
- Rust can often infer types, but you can declare them explicitly.

```rust
let immutable_var = 5; // Immutable
let mut mutable_var = 10; // Mutable
mutable_var = 11;

let typed_var: i32 = 20; // With explicit type
```

### 1.2. Basic Data Types
- **Integers**: `i8`, `u8`, `i32`, `u32`, `i64`, `u64`, `isize`, `usize`. Signed (`i`) or unsigned (`u`). `isize`/`usize` depend on the computer's architecture (32-bit or 64-bit).
- **Floating-Point**: `f32`, `f64`.
- **Booleans**: `bool` (`true` or `false`).
- **Characters**: `char`. Represents a single Unicode scalar value (e.g., 'a', '🚀').

### 1.3. Control Flow
- **`if-else`**: Conditions must be `bool`. `if` is an expression, so it can be used on the right side of a `let` statement.

```rust
let number = 6;
if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
}

let condition = true;
let x = if condition { 5 } else { 6 }; // `if` as an expression
```

- **Loops**: `loop`, `while`, and `for`.
    - `loop`: An infinite loop, broken with `break`.
    - `while`: Loops as long as a condition is true.
    - `for`: The most common and safe loop. Iterates over items from an iterator.

```rust
// for loop
for i in 0..5 { // 0 to 4
    println!("{}", i);
}

// for loop over an array
let a = [10, 20, 30];
for element in a.iter() {
    println!("the value is: {}", element);
}
```

---

## 2. Compound Data Types

### 2.1. Structs
- A custom data type that lets you package together and name multiple related values.

```rust
struct User {
    username: String,
    email: String,
    active: bool,
}

let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
};
```

### 2.2. Enums
- A type that can be one of several different variants.

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

let msg = Message::Write(String::from("hello"));
```

### 2.3. `match` Control Flow
- A powerful control flow construct that allows you to compare a value against a series of patterns and execute code based on which pattern matches.
- `match` expressions must be **exhaustive**, meaning all possibilities must be handled.

```rust
match msg {
    Message::Quit => println!("Quit"),
    Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
    Message::Write(text) => println!("Text message: {}", text),
    Message::ChangeColor(r, g, b) => println!("Change color to r: {}, g: {}, b: {}", r, g, b),
}
```

---

## 3. Ownership: Rust's Core Principle

Ownership is how Rust achieves memory safety without a garbage collector.

### 3.1. The Three Rules of Ownership
1.  Each value in Rust has a variable that’s called its **owner**.
2.  There can only be **one owner at a time**.
3.  When the owner goes out of scope, the value will be **dropped** (memory is freed).

### 3.2. Move vs. Copy
- When you assign a complex type (like `String`, `Vec`, or a `struct`) to another variable, the ownership is **moved**. The original variable is no longer valid.

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is moved to s2.
// println!("{}", s1); // This would cause a compile error!
```

- Simple types (like integers, bools, chars) that are stored entirely on the stack are **copied**. The original variable is still valid. These types implement the `Copy` trait.

```rust
let x = 5;
let y = x; // x is copied to y.
println!("x = {}, y = {}", x, y); // This is fine.
```

### 3.3. Borrowing and References
- Instead of transferring ownership, you can create a **reference** to a value, which is called **borrowing**.
- A reference is like a pointer, but with safety guarantees.
- References are immutable by default.

```rust
let s1 = String::from("hello");
let len = calculate_length(&s1); // Pass a reference to s1

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // s goes out of scope, but because it does not own the value, nothing happens.
```

### 3.4. The Rules of References
1.  At any given time, you can have either **one mutable reference** OR **any number of immutable references**.
2.  References must always be valid (they cannot be dangling).

This is the core of Rust's "fearless concurrency" promise, as it prevents data races at compile time.

---

## 4. Error Handling

Rust uses two primary enums for handling states that might be absent or might fail.

### 4.1. `Option<T>` for Nullability
- Used when a value could be something or it could be nothing.
- Variants: `Some(T)` and `None`.
- Prevents "null pointer" errors.

```rust
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

match find_user(1) {
    Some(name) => println!("User found: {}", name),
    None => println!("User not found."),
}
```

### 4.2. `Result<T, E>` for Fallibility
- Used for operations that can fail, like I/O, parsing, etc.
- Variants: `Ok(T)` for success and `Err(E)` for an error.

```rust
use std::fs::File;

fn open_file() -> Result<File, std::io::Error> {
    let f = File::open("hello.txt")?; // The `?` operator propagates errors.
    Ok(f)
}
```
- **The `?` Operator**: A shortcut for propagating errors. If a `Result` is `Err`, `?` will immediately return that `Err` from the function. If it's `Ok`, it will unwrap the value and continue.

---

## 5. Collections

### 5.1. `Vec<T>` (Vector)
- A growable, heap-allocated list of items of the same type.

```rust
let mut v: Vec<i32> = Vec::new();
v.push(5);
v.push(6);

let third: Option<&i32> = v.get(2); // Safe access, returns Option
// let third: &i32 = &v[2]; // Direct access, will panic if out of bounds
```

### 5.2. `String`
- A growable, heap-allocated, UTF-8 encoded string. It's essentially a `Vec<u8>`.

### 5.3. `HashMap<K, V>`
- A collection of key-value pairs, stored on the heap.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name); // Returns Option<&i32>
```

---

## 6. Traits

A **trait** tells the Rust compiler about functionality a type must provide. It's similar to an interface in other languages.

### 6.1. Defining and Implementing a Trait

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub author: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}
```

### 6.2. Common Traits
- `Debug`: Allows for printing a struct with `{:?}`.
- `Clone`: For explicitly creating a deep copy of a value.
- `Copy`: For types that can be trivially copied (like integers).
- `Drop`: For running cleanup code when a value goes out of scope.
- `From`/`Into`: For conversions between types.

---

## 7. Concurrency (Threads)

Rust's ownership model makes concurrent programming safer.

### 7.1. `thread::spawn`
- Creates a new thread.
- The `move` keyword is often used with closures in threads to transfer ownership of variables to the thread.

```rust
use std::thread;
use std::time::Duration;

let handle = thread::spawn(move || {
    for i in 1..5 {
        println!("hi number {} from the spawned thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
});

// Wait for the thread to finish
handle.join().unwrap();
```

### 7.2. `thread::scope` (Scoped Threads)
- A safer way to spawn threads that need to borrow data from the parent thread.
- Guarantees all threads finish before the scope ends, eliminating lifetime issues.

```rust
let my_data = vec![1, 2, 3];

thread::scope(|s| {
    s.spawn(|| {
        // This thread can safely BORROW my_data
        println!("Data: {:?}", &my_data);
    });
});
```

### 7.3. Channels for Message Passing
- A way for threads to communicate by sending each other messages. "Do not communicate by sharing memory; instead, share memory by communicating."
- `mpsc` stands for "multiple producer, single consumer".

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel(); // tx = transmitter, rx = receiver

thread::spawn(move || {
    let val = String::from("hi");
    tx.send(val).unwrap();
});

let received = rx.recv().unwrap();
println!("Got: {}", received);
```
*/
