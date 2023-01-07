use rust_data_structures::queues::simple_queue::Queue;
fn main() {
    let mut q = Queue::new();
    q.add("one");
    q.add("two");
    q.add("three");

    assert!(!q.is_empty());
    assert_eq!(q.depth(), 3);

    assert_eq!(q.remove().unwrap(), "one");
}
