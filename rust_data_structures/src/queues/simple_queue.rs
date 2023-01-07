use std::{borrow::BorrowMut, mem::take};

/// This is the implementation of a simple queue.
/// It's designed like a single linked list, intentionally no `Rc` or `RefCell` are used.
/// Otherwise, these would be used at least for having references to the items in the queue.
///
/// Note that the standard library offers `VecDequeue` as a queue implementation.
pub struct Queue<T> {
    head: Option<QueueItem<T>>,
    depth: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            head: None,
            depth: 0,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.depth == 0
    }

    pub fn add(&mut self, value: T) {
        let item = QueueItem::new(value);
        if let Some(head) = &mut self.head {
            let mut tail = head;
            loop {
                if let Some(_) = &tail.next {
                    tail = tail.next.as_mut().unwrap().borrow_mut();
                } else {
                    break;
                }
            }
            tail.next = Some(Box::new(item));
        } else {
            self.head = Some(item);
        }
        self.depth += 1
    }

    pub fn remove(&mut self) -> Option<T> {
        if !self.is_empty() {
            let head = take(&mut self.head).unwrap();
            if let Some(next) = head.next {
                self.head = Some(*next)
            }
            self.depth -= 1;
            Some(head.value)
        } else {
            None
        }
    }

    pub fn depth(&self) -> usize {
        self.depth
    }
}

pub struct QueueItem<T> {
    value: T,
    next: Option<Box<QueueItem<T>>>,
}

impl<T> QueueItem<T> {
    pub fn new(value: T) -> Self {
        QueueItem { value, next: None }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut q = Queue::<u32>::new();
        q.add(1);
        q.add(2);
        q.add(3);

        assert_eq!(q.remove(), Some(1));
        assert_eq!(q.remove(), Some(2));
        assert_eq!(q.remove(), Some(3));

        assert!(q.is_empty());
    }
}
