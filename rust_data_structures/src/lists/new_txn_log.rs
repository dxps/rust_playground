// ----------------------------------------------------------------------------
// The reason `Rc<RefCell<Node>` is used (in `Link`) instead of `Box<Node>`
// is that it provides the "interior mutability" and this allows us to properly
// set the `.next` references, whereas `Box`'s `clone()` would make a distinct
// and isolated copy of data.
// ----------------------------------------------------------------------------

use std::{cell::RefCell, rc::Rc};

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug)]
struct Node {
    value: String,
    next: Link,
    prev: Link,
}

#[derive(Debug)]
pub struct NewTxnLog {
    head: Link,
    tail: Link,
    length: u64,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            value,
            next: None,
            prev: None,
        }))
    }
}

impl NewTxnLog {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: String) -> &mut Self {
        let new = Node::new(value);

        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
        self
    }

    pub fn pop(&mut self) -> Option<String> {
        self.head.take().map(|head| {
            if let Some(next) = head.borrow_mut().next.take() {
                self.head = Some(next);
            } else {
                self.tail.take();
            }
            self.length -= 1;
            Rc::try_unwrap(head)
                .ok()
                .expect("Something is terribly wrong")
                .into_inner()
                .value
        })
    }

    pub fn length(&self) -> u64 {
        self.length
    }

    pub fn iter(&self) -> NewTxnLogIterator {
        NewTxnLogIterator::new(self.head.clone())
    }

    pub fn back_iter(&self) -> NewTxnLogIterator {
        NewTxnLogIterator::new(self.tail.clone())
    }
}

impl IntoIterator for NewTxnLog {
    type Item = String;

    type IntoIter = NewTxnLogIterator;

    fn into_iter(self) -> Self::IntoIter {
        NewTxnLogIterator::new(self.head)
    }
}

pub struct NewTxnLogIterator {
    current: Link,
}

impl NewTxnLogIterator {
    fn new(start_at: Link) -> Self {
        Self { current: start_at }
    }
}

impl Iterator for NewTxnLogIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let mut result = None;
        self.current = match &self.current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.next.clone()
            }
            None => None,
        };
        result
    }
}

impl DoubleEndedIterator for NewTxnLogIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        let mut result = None;
        self.current = match &self.current {
            Some(ref current) => {
                let current = current.borrow();
                result = Some(current.value.clone());
                current.prev.clone()
            }
            None => None,
        };
        result
    }
}

//
// -------------------------------------
//

#[cfg(test)]
mod tests {
    use crate::lists::NewTxnLog;

    #[test]
    fn txn_log_append() {
        let mut log = NewTxnLog::new();
        assert_eq!(log.length, 0);
        let item1 = "INSERT INTO mytable VALUES (1,2,3)".to_owned();
        let item2 = "INSERT INTO mytable VALUES (2,3,4)".to_owned();
        let item3 = "INSERT INTO mytable VALUES (3,4,5)".to_owned();
        log.append(item1.clone())
            .append(item2.clone())
            .append(item3.clone());
        assert_eq!(log.length(), 3);
    }

    #[test]
    fn txn_log_pop() {
        let mut log = NewTxnLog::new();
        assert_eq!(log.length, 0);
        let item1 = "INSERT INTO mytable VALUES (1,2,3)".to_owned();
        let item2 = "INSERT INTO mytable VALUES (2,3,4)".to_owned();
        let item3 = "INSERT INTO mytable VALUES (3,4,5)".to_owned();
        log.append(item1.clone())
            .append(item2.clone())
            .append(item3.clone());
        assert_eq!(log.length(), 3);

        assert_eq!(log.pop(), Some(item1));
        assert_eq!(log.pop(), Some(item2));
        assert_eq!(log.pop(), Some(item3));
        assert_eq!(log.pop(), None);
    }

    #[test]
    fn txn_log_iter() {
        let item1 = "INSERT INTO mytable VALUES (1,2,3)".to_owned();
        let item2 = "INSERT INTO mytable VALUES (1,2,3)".to_owned();
        let item3 = "INSERT INTO mytable VALUES (1,2,3)".to_owned();
        let mut log = NewTxnLog::new();
        log.append(item1).append(item2).append(item3);
        assert_eq!(log.into_iter().count(), 3);
    }
}
