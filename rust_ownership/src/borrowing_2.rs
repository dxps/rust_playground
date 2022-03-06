fn main() {
    let mut original = String::from("original value");
    println!("\n[main] original: \"{original}\"");

    // An explicit inner scope.
    {
        print_val(&original);
        change_val(&mut original);
        println!("\t[inner scope] value=\"{original}\"")
    }

    println!("[main] original: \"{original}\"");
}

fn print_val(value: &String) {
    println!("\t[print_val] value=\"{value}\"")
}

fn change_val(value: &mut String) {
    let next = value;
    *next = "changed value".to_string();
    println!("\t[change_val] value=\"{next}\"")
}
