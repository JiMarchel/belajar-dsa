use super::stack::Stack;

pub fn divide_by_two(mut dec_num: i32) -> String {
    //Save the remainder in the stack
    let mut rem_stack = Stack::new();

    //push rem into the stack
    while dec_num > 0 {
        let rem = dec_num % 2;
        rem_stack.push(rem);
        dec_num /= 2;
    }

    //pop out elems from the stack to form a string
    let mut bin_str = "".to_string();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap().to_string();
        bin_str += &rem;
    }

    bin_str
}
