use std::mem::swap;

pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

impl<T> Queue<T> {
    fn new() -> Self {
        Self {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.younger.push(item);
    }

    fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    fn pop(&mut self) -> Option<T> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        self.older.pop()
    }
}

fn main() {
    let mut q = Queue::new();
    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));
    assert!(!q.is_empty());
    q.push('∞');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('∞'));
    assert_eq!(q.pop(), None);
    assert!(q.is_empty());

    let mut bq = Box::new(Queue::new());
    bq.push('0');

    let mut r = Queue::<char>::new();
    r.push('0');
}
