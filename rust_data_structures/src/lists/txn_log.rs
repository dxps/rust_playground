// ----------------------------------------------------------------------------
// The reason `Rc<RefCell<Node>` is used instead of `Box<Node>` is that it
// provides the "interior mutability" and this allows us to properly set
// the `.next` references, whereas `Box`'s `clone()` would make some distinct
// and isolated copies of data.
// ----------------------------------------------------------------------------

use std::{cell::RefCell, rc::Rc};

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone, Debug)]
struct Node {
    value: String,
    next: Link,
}

#[derive(Debug)]
pub struct TransactionLog {
    head: Link,
    tail: Link,
    length: u64,
}

impl Node {
    fn new(value: String) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { value, next: None }))
    }
}

impl TransactionLog {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }

    pub fn append(&mut self, value: String) {
        let new = Node::new(value);

        match self.tail.take() {
            Some(old) => old.borrow_mut().next = Some(new.clone()),
            None => self.head = Some(new.clone()),
        };
        self.length += 1;
        self.tail = Some(new);
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
}

#[cfg(test)]
mod tests {
    use crate::lists::txn_log::TransactionLog;

    #[test]
    fn transaction_log_append() {
        let mut txn_log = TransactionLog::new();
        assert_eq!(txn_log.length, 0);
        txn_log.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        txn_log.append("INSERT INTO mytable VALUES (2,3,4)".to_owned());
        txn_log.append("INSERT INTO mytable VALUES (3,4,5)".to_owned());
        assert_eq!(txn_log.length(), 3);

        assert_eq!(
            txn_log.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            txn_log.pop(),
            Some("INSERT INTO mytable VALUES (2,3,4)".to_owned())
        );
        assert_eq!(
            txn_log.pop(),
            Some("INSERT INTO mytable VALUES (3,4,5)".to_owned())
        );
        assert_eq!(txn_log.pop(), None);
    }

    #[test]
    fn transaction_log_pop() {
        let mut list = TransactionLog::new();
        assert_eq!(list.pop(), None);
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        list.append("INSERT INTO mytable VALUES (1,2,3)".to_owned());
        assert_eq!(
            list.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            list.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(
            list.pop(),
            Some("INSERT INTO mytable VALUES (1,2,3)".to_owned())
        );
        assert_eq!(list.pop(), None);
    }
}
