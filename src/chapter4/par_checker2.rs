// Matching all 3 parenthesis
use super::stack::Stack;

pub fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closers = ")]}";

    opens.find(open) == closers.find(close)
}

pub fn par_checker2(par: &str) -> bool {
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
        } else {
            if stack.is_empty() {
                balance = false;
            } else {
                //determine if match
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    balance = false;
                }
            }
        }

        index += 1;
    }

    // parenthesis matched: balanced and empty stack
    balance && stack.is_empty()
}
