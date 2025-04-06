#[derive(Debug)]
pub struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    //init a stack
    pub fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    //check if vec empty
    pub fn is_empty(&self) -> bool {
        0 == self.size
    }

    //check length of vec
    pub fn len(&self) -> usize {
        self.size
    }

    //clear vec
    pub fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    //put the item into the tail of vec
    pub fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if 0 == self.size {
            return None;
        }
    }
}
