use ::rust_datatypes::slices;

fn main() {
    let arr1 = [123, 234, 345];
    slices::print_array(&arr1);

    let vec1 = vec![10, 20, 30];
    slices::print_vector(&vec1);

    slices::print_slice_array_or_vector(&arr1);
    slices::print_slice_array_or_vector(&vec1);
}
