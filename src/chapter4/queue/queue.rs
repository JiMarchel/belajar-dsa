use std::collections::VecDeque;

#[derive(Debug)]
pub struct Queue<T> {
    data: VecDeque<T>,
    cap: usize,
}

impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Self {
            data: VecDeque::with_capacity(size),
            cap: size,
        }
    }

    pub fn enqueue(&mut self, val: T) -> Result<(), String> {
        if self.data.len() == self.cap {
            Err("No space available".to_string())
        } else {
            self.data.push_front(val);
            Ok(())
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.data.pop_back()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn clear(&mut self) {
        self.data = VecDeque::with_capacity(self.cap)
    }

    pub fn is_full(&self) -> bool {
        self.len() == self.cap
    }
}
