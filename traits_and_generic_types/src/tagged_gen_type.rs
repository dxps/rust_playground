
struct Tagged<T> {
    value: T,
    tag: String
}

impl<T> Tagged<T>  {
    fn tag(&self) -> String {
        self.tag.clone()
    }
}

fn main() {

    let tagged_item = Tagged {
        tag: String::from("tag1"),
        value: "#something",
    };
    println!("\n>>> Item tagged with '{}' and value '{}'.\n", tagged_item.tag(), tagged_item.value)

}
