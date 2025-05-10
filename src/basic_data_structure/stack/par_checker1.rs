// Matching parenthesis

use super::stack::Stack;

pub fn par_checker1(par: &str) -> bool {
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

        if '(' == c {
            //if it's '(', put data into stack
            stack.push(c);
        } else {
            //if it's ')', determine if stack is empty
            if stack.is_empty() {
                //empty stack, matched
                balance = false;
            } else {
                let _r = stack.pop();
            }
        }

        index += 1;
    }

    // parenthesis matched: balanced and empty stack
    balance && stack.is_empty()
}
