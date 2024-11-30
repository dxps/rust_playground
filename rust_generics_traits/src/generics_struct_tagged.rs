// ----------------------------------------------------------------------------
// This example shows a structure named `Tagged` using `T` is a generic type.
// The `tag()` method is implemented in a `impl` block that also declares `T`
// as a generic type. Without it (just `impl Tagged<T>`), rustc would consider
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
        tag: "comm".into(),
        value: "Communication related",
    };
    println!(
        "\n>>> Item tagged with '{}' and having value '{}'.\n",
        tagged_item.tag(),
        tagged_item.value
    )
}
