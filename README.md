# My Rust Learning Journey

![Rust Logo](https://raw.githubusercontent.com/lecepin/rust-logo/5bcf63a2e8ad83cacdfe37880c060f71e608d2db/images/1660286946670.svg)

This Markdown document serves as a detailed log of my journey learning Rust programming language. Each section corresponds to a day of learning, where I summarize the concepts I've studied, the programs I've built, and the progress I've made. Dive into each day to explore the topics covered and my code commits, providing insights into my Rust learning process.


---

<style>
summary  {
  font-size: 1.5em; 
  margin: 0.5em 0; 
}
</style>

<details>
<summary><strong> Day 1 </strong></summary>

Today, I started my journey in Rust by learning the basics of the language. I learned about Rust syntax, basic data types, and how to write simple programs.

One of the first programs I created is a guessing game. In this game, the user enters a number, and the program generates a random number. The user's number is then compared with the randomly generated number to determine if they've guessed correctly.


</details>

---
<details>
<summary><strong> Day 2 </strong></summary>

Today, I delved into one of the core concepts of Rust programming: ownership and borrowing.

In Rust, ownership rules ensure memory safety without the need for a garbage collector. I learned that every value in Rust has a variable that's called its owner. There can only be one owner at a time, and when the owner goes out of scope, the value will be dropped.

I also learned about borrowing in Rust, which allows multiple parts of the code to access data without needing to copy it into multiple places. Borrowing can be done either immutably or mutably, and Rust's compiler ensures that there are no data races or dangling pointers.

Understanding ownership and borrowing is crucial for writing safe and efficient Rust code. I practiced creating programs that leverage these concepts to manage memory effectively and prevent common pitfalls like memory leaks and data races.

</details>

---
<details>

<summary><strong> Day 3 </strong></summary>

Today, I explored Rust's rich type system by learning about enums and structs.

Enums, short for enumerations, allow me to define a type by enumerating its possible variants. This is particularly useful for representing concepts that can take on a limited set of values. I learned how to define and use enums in Rust, including pattern matching to handle different variants.

Additionally, I delved into structs, which are used to create custom data types by bundling together multiple pieces of data into a single compound type. Structs allow me to define the structure of my data and provide methods to operate on that data.

By mastering enums and structs, I gained a deeper understanding of how to model real-world problems and organize data effectively in Rust programs.
</details>

---
<details>

<summary><strong> Day 4 </strong></summary>

Today, I delved into two important topics in Rust: pattern matching and traits.

### Creating a Simple Markup Language

I created a simple markup language in Rust that performs text transformation based on certain symbols:
- Text within `^` is converted to uppercase.
- Text within `_` is converted to lowercase.
- Text within `#` is ignored.

### Learning About Traits
I also learned about traits in Rust, which define shared behavior for types. Traits allow me to define methods that types must implement, enabling code reuse and polymorphism.

By understanding pattern matching and traits, I gained powerful tools for writing expressive and reusable code in Rust.
</details>

---
<details>

<summary><strong> Day 5 </strong></summary>

Today, I explored two powerful concepts in Rust: generics and dynamic dispatch.

### Understanding Generics

Generics in Rust allow me to write code that can work with any type. By abstracting over types, I can write functions, structs, and enums that are more flexible and reusable.

Here's a simple example of a generic function that swaps the values of two variables:

```rust
fn swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b);
}

fn main() {
    let mut x = 5;
    let mut y = 10;
    println!("Before swap: x = {}, y = {}", x, y);
    swap(&mut x, &mut y);
    println!("After swap: x = {}, y = {}", x, y);
}
```
### Exploring Dynamic Dispatch
Dynamic dispatch allows me to write code that operates on different types without knowing the exact type at compile time. This is achieved through trait objects, which enable runtime polymorphism.

Here's an example of using dynamic dispatch with trait objects:

```rust
trait Animal {
    fn sound(&self);
}

struct Dog;
impl Animal for Dog {
    fn sound(&self) {
        println!("Woof!");
    }
}

struct Cat;
impl Animal for Cat {
    fn sound(&self) {
        println!("Meow!");
    }
}

fn make_sound(animal: &dyn Animal) {
    animal.sound();
}

fn main() {
    let dog = Dog;
    let cat = Cat;
    make_sound(&dog);
    make_sound(&cat);
}
```
By learning about generics and dynamic dispatch, I gained the ability to write more flexible and versatile code in Rust.
</details>

---
<details>

<summary><strong> Day 6 </strong></summary>

Today, I delved into two important concepts in Rust: closures and iterators.

### Understanding Closures

Closures are anonymous functions that can capture variables from their surrounding environment. They are a powerful tool for writing concise and expressive code in Rust.

Here's an example of a closure that adds two numbers together:

```rust
fn main() {
    let add = |a, b| a + b;
    let result = add(3, 5);
    println!("Result: {}", result);
}
```
Closures can capture variables from their environment, allowing for flexible and reusable code. They can also be stored in variables or passed as arguments to other functions.

### Exploring Iterators
Iterators provide a way to traverse and process elements in a collection in a sequential manner. They are a fundamental part of Rust's standard library and are used extensively in Rust code.

Here's an example of using iterators to find the sum of numbers in a vector:

```rust 
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("Sum: {}", sum);
}
```
Iterators in Rust are lazy, meaning they only perform operations when needed. This allows for efficient and composable code.

By learning about closures and iterators, I gained powerful tools for working with data and writing functional-style code in Rust.
</details>

---

<details>
<summary > <strong> Day 7 </strong></summary>

Today, I delved into several important concepts in Rust: `map`, `fold`, `filter`, and parallel iterators using the Rayon crate.

### Exploring map, fold, and filter

`map`, `fold`, and `filter` are fundamental functions provided by iterators in Rust. They allow for efficient and expressive manipulation of collections.

Here's a brief overview of each function:
- `map`: Transforms each element in a collection according to a specified function.
- `fold`: Accumulates the elements of a collection into a single value by applying a combining function.
- `filter`: Selects elements from a collection that satisfy a specified predicate function.

Here's an example of using `map`, `fold`, and `filter` to perform various operations on a vector:

```rust
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Using map to square each number
    let squared_numbers: Vec<_> = numbers.iter().map(|&x| x * x).collect();
    println!("Squared Numbers: {:?}", squared_numbers);

    // Using fold to find the sum of numbers
    let sum: i32 = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {}", sum);

    // Using filter to select even numbers
    let even_numbers: Vec<_> = numbers.iter().filter(|&x| x % 2 == 0).collect();
    println!("Even Numbers: {:?}", even_numbers);
}
```

### Using Rayon for Parallel Iterators
Rayon is a high-performance parallel iterator library for Rust that allows for easy parallelization of computation across multiple threads.

By leveraging Rayon's parallel iterators, I was able to significantly improve the performance of my computation by utilizing multiple CPU cores.

</details>

---

The journey continues as I delve deeper into Rust, exploring more advanced topics and building more complex projects. Stay tuned for further updates as I progress on my Rust learning adventure!
