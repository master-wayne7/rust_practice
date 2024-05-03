use std::thread;

fn main() {
    let closure1 = |x| x + 1;
    println!("{}", closure1(2));

    let val = 10;
    let closure2 = |x| x + val;
    println!("{}", closure2(2));

    let handle = thread::spawn(|| println!("From a thread"));
    println!("Before thread");
    let _ = handle.join();
}

// fn f1() -> impl Fn(i32) -> i32 {
//     |x| x + 1
// }

// fn f2() -> Box<dyn Fn(i32) -> i32> {
//     let f = |x| x + 1;
//     Box::new(f)
// }

// fn print<T: Display>(t: T) {
//     println!("{}", t);
// }
