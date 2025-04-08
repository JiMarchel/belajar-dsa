use super::stack::Stack;

pub fn base_converter(mut dec_num: i32, base: i32) -> String {
    //Save the remainder in the stack
    let digits = [
        '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F',
    ];
    let mut rem_stack = Stack::new();

    //push rem into the stack
    while dec_num > 0 {
        let rem = dec_num % base;
        rem_stack.push(rem);
        dec_num /= base;
    }

    //pop out elems from the stack to form a string
    let mut bin_str = "".to_string();
    while !rem_stack.is_empty() {
        let rem = rem_stack.pop().unwrap() as usize;
        bin_str += &digits[rem].to_string();
    }

    bin_str
}
