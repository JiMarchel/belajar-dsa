use super::stack::Stack;

pub fn postfix_eval(postfix: &str) -> Option<i32> {
    // the expression needs at least two operands and
    // one operator, and two spaces to separate them.
    if postfix.len() < 5 {
        return None;
    }

    let mut ops = Stack::new();
    for token in postfix.split_whitespace() {
        // Strings can be compared directly
        if "0" <= token && token <= "9" {
            ops.push(token.parse::<i32>().unwrap());
        } else {
            // For substraction and division, the order matters
            let op2 = ops.pop().unwrap();
            let op1 = ops.pop().unwrap();
            let res = do_calc(token, op1, op2);
            ops.push(res);
        }
    }

    Some(ops.pop().unwrap())
}
pub fn do_calc(op: &str, op1: i32, op2: i32) -> i32 {
    if "+" == op {
        op1 + op2
    } else if "-" == op {
        op1 - op2
    } else if "*" == op {
        op1 * op2
    } else if "/" == op {
        if 0 == op2 {
            panic!("ZeroDivisionError: Invalid operation!");
        }
        op1 / op2
    } else {
        panic!("OperatorError: Invalid Operator: {:?}", op);
    }
}
