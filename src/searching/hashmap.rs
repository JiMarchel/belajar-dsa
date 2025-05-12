use core::panic;

#[derive(Debug, Clone, PartialEq)]
pub struct HashMap<T> {
    cap: usize,       //capacity
    slot: Vec<usize>, //store data adress (index)
    data: Vec<T>,     //store elements
}
impl<T: Clone + PartialEq + Default> HashMap<T> {
    pub fn new(cap: usize) -> Self {
        //init slow and data
        let mut slot = Vec::with_capacity(cap);
        let mut data = Vec::with_capacity(cap);
        for _i in 0..cap {
            slot.push(0);
            data.push(Default::default());
        }

        HashMap { cap, slot, data }
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        for &d in self.slot.iter() {
            //if slot not empty, then increase len by 1
            if 0 != d {
                len += 1;
            }
        }

        len
    }

    pub fn is_empty(&self) -> bool {
        let mut empty = true;
        for &d in self.slot.iter() {
            if 0 != d {
                empty = false;
                break;
            }
        }

        empty
    }

    pub fn clear(&mut self) {
        let mut slot = Vec::with_capacity(self.cap);
        let mut data = Vec::with_capacity(self.cap);
        for _i in 0..self.cap {
            slot.push(0);
            data.push(Default::default());
        }

        self.slot = slot;
        self.data = data;
    }

    pub fn hash(&self, key: usize) -> usize {
        key % self.cap
    }

    pub fn rehash(&self, pos: usize) -> usize {
        (pos + 1) % self.cap
    }

    pub fn insert(&mut self, key: usize, value: T) {
        if 0 == key {
            panic!("Error: key must > 0")
        }

        let pos = self.hash(key);
        if 0 == self.slot[pos] {
            self.slot[pos] = key;
            self.data[pos] = value;
        } else {
            let mut next = self.rehash(pos);
            while 0 != self.slot[next] && key != self.slot[next] {
                next = self.rehash(next);

                if next == pos {
                    println!("Error: Slot if full");
                    return;
                }
            }

            if 0 == self.slot[next] {
                self.slot[next] = key;
                self.data[next] = value;
            } else {
                self.data[next] = value;
            }
        }
    }

    pub fn remove(&mut self, key: usize) -> Option<T> {
        if 0 == key {
            panic!("Error: key must > 0")
        }

        let pos = self.hash(key);
        if 0 == self.slot[pos] {
            None
        } else if key == self.slot[pos] {
            self.slot[pos] = 0;
            let data = Some(self.data[pos].clone());
            self.data[pos] = Default::default();
            data
        } else {
            let mut data: Option<T> = None;
            let mut stop = false;
            let mut found = false;
            let mut curr = pos;

            while 0 != self.slot[curr] && !found && !stop {
                if key == self.slot[curr] {
                    found = true;
                    self.slot[curr] = 0;
                    data = Some(self.data[curr].clone());
                    self.data[curr] = Default::default();
                } else {
                    curr = self.rehash(curr);
                    if curr == pos {
                        stop = true;
                    }
                }
            }
            data
        }
    }

    pub fn get_pos(&self, key: usize) -> usize {
        if 0 == key {
            panic!("Error: key must > 0")
        }

        //Calculate data position
        let pos = self.hash(key);
        let mut stop = false;
        let mut found = false;
        let mut curr = pos;

        while 0 != self.slot[curr] && !found && !stop {
            if key == self.slot[curr] {
                found = true;
            } else {
                curr = self.rehash(curr);
                if curr == pos {
                    stop = true;
                }
            }
        }
        curr
    }

    pub fn get(&self, key: usize) -> Option<&T> {
        let curr = self.get_pos(key);
        self.data.get(curr)
    }

    pub fn get_mut(&mut self, key: usize) -> Option<&mut T> {
        let curr = self.get_pos(key);
        self.data.get_mut(curr)
    }

    pub fn contains(&self, key: usize) -> bool {
        if 0 == key {
            panic!("Error: key must > 0");
        }
        self.slot.contains(&key)
    }

    pub fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { stack: Vec::new() };
        for item in self.data.iter() {
            iterator.stack.push(item);
        }
        iterator
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { stack: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.stack.push(item);
        }
        iterator
    }
}

pub struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

pub struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}
