fn main() {
    let mut iterator = (1..10).into_iter();

    println!("{:?}", iterator.next());
    let mut taken = iterator.take(2);
    println!("{:?} here", taken.next());
    println!("{:?}", taken.next());
    println!("{:?}", taken.next());
    println!("{:?}", taken.next());
    println!("{:?}", taken.next());

    let iterator = vec!['A', 'B', 'C'].into_iter();
    let mut enumerated = iterator.enumerate();
    println!("{:?}", enumerated.next());
    println!("{:?}", enumerated.next());
}
