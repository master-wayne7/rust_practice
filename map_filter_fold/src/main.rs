fn main() {
    let items = (1..10).into_iter();
    // map function on an iter
    let other_items: Vec<i32> = items.clone().map(|x| x + 1).collect();

    // filter function on an iter
    let filter_items: Vec<_> = items.clone().filter(|x| x % 2 == 0).collect();

    // fold function on an iter
    let sum: i32 = items.clone().fold(0, |acc, x| acc + x);

    // combined
    let combined = (1..100)
        .into_iter()
        .filter(|x| x % 2 == 1)
        .map(|x| x * x)
        .filter(|x| x % 5 != 0)
        .fold(0, |acc, x| acc + x);

    println!("map {:?}", other_items);
    println!("filter {:?}", filter_items);
    println!("fold {:?}", sum);

    println!("combined {:?}", combined);
}
