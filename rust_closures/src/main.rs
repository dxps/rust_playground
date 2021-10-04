fn math(a: i32, b: i32, op: Box<dyn Fn(i32, i32) -> i32>) -> i32 {
    op(a, b)
}

fn main() {
    let add = Box::new(|a, b| a + b);
    let sub = Box::new(|a, b| a - b);
    println!("add: {}", math(2, 2, add));
    println!("sub: {}", math(2, 2, sub));

    // Closures can also take data from outside their environment.
    // In this example, "move" must be used to "capture" the flag internally.
    let flag = "dbg";
    let mul = Box::new(move |a, b| {
        println!("[{}] on {} and {}", flag, a, b);
        a * b
    });
    println!("mul: {}", math(2, 2, mul));
}
