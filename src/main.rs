use chapter4::dequeue::palindrome_checker::palindrome_checker;

mod chapter4;

fn main() {
    let pal = "rustsur";
    let is_pal = palindrome_checker(pal);
    println!("{pal} is palindrome string: {is_pal}");

    let pal = "panda";
    let is_pal = palindrome_checker(pal);
    println!("{pal} is palindrome string: {is_pal}");
}
