// ----------------------------------------------------------------------------
// This example shows a structure named `Tagged` using `T` is a generic type.
// The `tag()` method is implemented in a `impl` block that also declares `T`
// as a generic type. Without it (just `impl Tagged<T>`), Rustc would consider
// `T` as a concrete type, which is not.
// ----------------------------------------------------------------------------

struct Tagged<T> {
    tag: String,
    value: T,
}

impl<T> Tagged<T> {
    fn tag(&self) -> String {
        self.tag.clone()
    }
}

fn main() {
    let tagged_item = Tagged {
        tag: String::from("tag1"),
        value: "#something",
    };
    println!(
        "\n>>> Item tagged with '{}' and value '{}'.\n",
        tagged_item.tag(),
        tagged_item.value
    )
}
