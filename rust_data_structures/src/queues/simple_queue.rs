use std::{borrow::BorrowMut, mem::take};

/// This is the implementation of a simple queue.
///
/// Note that the standard library offers a queue implementation as `VecDequeue`.
///

struct Queue<T> {
    end: Option<QueueItem<T>>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue { end: None }
    }

    pub fn is_empty(&self) -> bool {
        match self.end {
            Some(_) => false,
            None => true,
        }
    }

    pub fn add(&mut self, value: T) {
        let item = QueueItem::new(value);
        if let Some(end) = &mut self.end {
            let mut last = end;
            loop {
                if let Some(_) = &last.next {
                    last = last.next.as_mut().unwrap().borrow_mut();
                } else {
                    break;
                }
            }
            last.next = Some(Box::new(item));
        } else {
            self.end = Some(item);
        }
    }

    pub fn remove(&mut self) -> Option<T> {
        if !self.is_empty() {
            let last = take(&mut self.end).unwrap();
            if let Some(next) = last.next {
                self.end = Some(*next)
            }
            Some(last.value)
        } else {
            None
        }
    }
}

struct QueueItem<T> {
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
