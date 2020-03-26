pub fn print_array(param: &[i32; 3]) {
    println!("this is an array: {:?}", param);
}

pub fn print_vector(param: &Vec<i32>) {
    println!("this is a vector: {:?}", param);
}

// This function allows receiving a reference to either an array or vector
// of i32 values since its input param is a slice of i32.
pub fn print_slice_array_or_vector(param: &[i32]) {
    println!("this is a slice: {:?}", param);
}
