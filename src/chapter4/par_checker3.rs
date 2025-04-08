// Skipping non parenthesis character
use super::{par_checker2::par_match, stack::Stack};

pub fn par_checker3(par: &str) -> bool {
    //adding character to Vec
    let mut char_list = Vec::new();
    for c in par.chars() {
        char_list.push(c);
    }

    let mut index = 0;
    let mut balance = true; //determine if balance
    let mut stack = Stack::new();

    while index < char_list.len() && balance {
        let c = char_list[index];

        if '(' == c || '[' == c || '{' == c {
            stack.push(c);
        }

        if ')' == c || ']' == c || '}' == c {
            if stack.is_empty() {
                balance = false;
            } else {
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    balance = false;
                }
            }
        }

        index += 1;
    }

    balance && stack.is_empty()
}
