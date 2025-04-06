mod basic_dsa;
use basic_dsa::stack::Stack;

fn main() {
    basic();

    fn basic() {
        let mut s: Stack<i32> = Stack::new();
        s.push(1);
        s.push(2);
        s.push(3);

        println!("Size: {}, {:?}", s.len(), s);
    }
}
