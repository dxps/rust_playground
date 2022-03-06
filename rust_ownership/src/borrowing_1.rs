fn main() {
    let mut original = String::from("original value");
    println!("\noriginal: \"{original}\"");

    // Borrowing the 'original' value and updating it.

    let next = &mut original; // Get a mutable ref to the 'original'.
    next.push_str(" v2"); // Update the value that it refers to.
    *next = String::from("next update"); // Update the value that it refers to, this time using dereference operator.

    // 'next' is out of scope here, so there is no conflict between owning and borrowing at the same time.

    println!("\noriginal: \"{original}\""); // The 'original' variable sees this change.
    original = String::from("changed value"); // Of course, 'original' var owns it and can change it.
    println!("\noriginal: \"{original}\"");
}
