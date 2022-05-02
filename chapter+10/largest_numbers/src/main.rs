fn main() {
    let largest = largest(&vec![10, 20, 66, 12, 14, 256]);
    println!("the largest number is {}", largest);
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest;
}