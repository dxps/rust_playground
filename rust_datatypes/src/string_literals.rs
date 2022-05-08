fn main() {
    let s1 = "hello";
    let s2 = s1; // s2 gets a copy of it, not ownership.

    let s1 = s1.to_uppercase();

    println!("\ts1={}", &s1);
    println!("\ts2={}", &s2);
}
