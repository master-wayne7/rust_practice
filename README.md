# My Rust Learning Journey

![Rust Logo](https://raw.githubusercontent.com/lecepin/rust-logo/5bcf63a2e8ad83cacdfe37880c060f71e608d2db/images/1660286946670.svg)

## Day 1

Today, I started my journey in Rust by learning the basics of the language. I learned about Rust syntax, basic data types, and how to write simple programs.

One of the first programs I created is a guessing game. In this game, the user enters a number, and the program generates a random number. The user's number is then compared with the randomly generated number to determine if they've guessed correctly.


---

## Day 2

Today, I delved into one of the core concepts of Rust programming: ownership and borrowing.

In Rust, ownership rules ensure memory safety without the need for a garbage collector. I learned that every value in Rust has a variable that's called its owner. There can only be one owner at a time, and when the owner goes out of scope, the value will be dropped.

I also learned about borrowing in Rust, which allows multiple parts of the code to access data without needing to copy it into multiple places. Borrowing can be done either immutably or mutably, and Rust's compiler ensures that there are no data races or dangling pointers.

Understanding ownership and borrowing is crucial for writing safe and efficient Rust code. I practiced creating programs that leverage these concepts to manage memory effectively and prevent common pitfalls like memory leaks and data races.


---

## Day 3

Today, I explored Rust's rich type system by learning about enums and structs.

Enums, short for enumerations, allow me to define a type by enumerating its possible variants. This is particularly useful for representing concepts that can take on a limited set of values. I learned how to define and use enums in Rust, including pattern matching to handle different variants.

Additionally, I delved into structs, which are used to create custom data types by bundling together multiple pieces of data into a single compound type. Structs allow me to define the structure of my data and provide methods to operate on that data.

By mastering enums and structs, I gained a deeper understanding of how to model real-world problems and organize data effectively in Rust programs.

---

## Day 4

Today, I delved into two important topics in Rust: pattern matching and traits.

### Creating a Simple Markup Language

I created a simple markup language in Rust that performs text transformation based on certain symbols:
- Text within `^` is converted to uppercase.
- Text within `_` is converted to lowercase.
- Text within `#` is ignored.

### Learning About Traits
I also learned about traits in Rust, which define shared behavior for types. Traits allow me to define methods that types must implement, enabling code reuse and polymorphism.

By understanding pattern matching and traits, I gained powerful tools for writing expressive and reusable code in Rust.

---