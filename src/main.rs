use chapter4::postfix_eval::postfix_eval;

mod chapter4;

fn main() {
    let postfix = "1 2 + 1 2 + *";
    let res = postfix_eval(postfix);
    match res {
        Some(val) => println!("res = {val}"),
        None => println!("{postfix} isn't a valid prefix"),
    }
}
