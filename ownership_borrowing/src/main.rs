// fn take_ownership_sum(v: Vec<i32>) -> i32 {
//     let mut sum = 0;
//     for value in v {
//         sum += value;
//     }
//     sum
// }

// fn borrow_sum(v: &Vec<i32>) -> i32 {
//     let mut sum = 0;
//     for value in v {
//         sum += *value;
//     }
//     sum
// }

// fn cap_values_owned(max: i32, mut v: Vec<i32>) -> Vec<i32> {
//     for index in 0..v.len() {
//         if v[index] > max {
//             v[index] = max;
//         }
//     }
//     v
// }
// fn cap_values_borrowed(max: i32, v: &mut Vec<i32>) {
//     for index in 0..v.len() {
//         if v[index] > max {
//             v[index] = max;
//         }
//     }
// }

fn main() {
    // let values = vec![1, 2, 3, 4, 5];
    // let sum = take_ownership_sum(values);

    // println!("sum of {:?} values {}", values, sum);
    // let sum = borrow_sum(&values);

    // println!("sum of {:?} values {}", values, sum);

    // let mut values = vec![1, 2, 3, 10000, 5];

    // values = cap_values_owned(10, values);
    // for v in values {
    //     print!("{v}");
    // }
    // cap_values_borrowed(10, &mut values);
    // for v in values {
    //     print!("{v}");
    // }

    let mut values = vec![1, 2, 3, 4, 5];

    // let a = &values;
    // let b = &values;

    values[2] = 20000000;
}
