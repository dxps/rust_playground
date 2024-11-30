// ----------------------------------------------------------------------------
// `get_max` function is using a generic `T` type that implements `PartialOrd`
// trait. This is using the `Trait Bounds` feature of the language.
//
// Since it returns the value (not reference to it), each `max = ...` statement
// implies a move, but rustc sees list is a non-copy slice because it doesn't
// know the size of `T` as a generic type. Therefore, `T` needs to implement
// the `Copy` trait. That's why it is added to the trait bounds `T`.
// ----------------------------------------------------------------------------

fn get_max<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut max = list[0];
    for &item in list.iter() {
        if item > max {
            max = item;
        }
    }
    max
}

fn main() {
    let numbers = vec![32, 24, 65, 78, 56];
    let max = get_max(&numbers);
    println!("The max number is {}", max);
}
