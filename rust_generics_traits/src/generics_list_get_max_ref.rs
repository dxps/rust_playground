// ----------------------------------------------------------------------------
// `get_max` function is using a generic `T` type that implements `PartialOrd`
// trait. This is using the `Trait Bounds` feature of the language.
// Also, it returns a reference to that type instead of the value itself, as
// returning a value would imply that `T` to implement also the `Copy` trait
// in order to allow `max = &list[0]` statement.
// ----------------------------------------------------------------------------

fn get_max<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut max = &list[0];
    for item in list.iter() {
        if item > max {
            max = item;
        }
    }
    &max
}

fn main() {
    let numbers = vec![32, 24, 65, 78, 56];
    let max = get_max(&numbers);
    println!("The max number is {}", max);
}
