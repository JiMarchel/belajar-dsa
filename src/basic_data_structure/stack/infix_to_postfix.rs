use std::collections::HashMap;

use super::{par_checker3::par_checker3, stack::Stack};

pub fn infix_to_postfix(infix: &str) -> Option<String> {
    //Check parenthesis
    if !par_checker3(infix) {
        return None;
    }

    //set priority of all symbols
    let mut prec = HashMap::new();
    prec.insert("(", 1);
    prec.insert(")", 1);
    prec.insert("+", 2);
    prec.insert("-", 2);
    prec.insert("*", 3);
    prec.insert("/", 3);

    //ops: save operators, postfix: save postfix expression
    let mut ops = Stack::new();
    let mut postfix = Vec::new();

    for token in infix.split_whitespace() {
        if ("A" <= token && token <= "Z") || ("0" <= token && token <= "9") {
            postfix.push(token);
        } else if "(" == token {
            ops.push(token);
        } else if ")" == token {
            let mut top = ops.pop().unwrap();
            while top != "(" {
                postfix.push(top);
                top = ops.pop().unwrap();
            }
        } else {
            while !ops.is_empty() && prec[ops.peek().unwrap()] >= prec[token] {
                postfix.push(ops.pop().unwrap());
            }
            ops.push(token);
        }
    }

    while !ops.is_empty() {
        postfix.push(ops.pop().unwrap());
    }

    let mut postfix_str = "".to_string();
    for c in postfix {
        postfix_str += &c.to_string();
        postfix_str += " ";
    }

    Some(postfix_str)
}
