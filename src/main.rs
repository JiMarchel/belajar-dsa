use chapter4::divide_by_two::divide_by_two;

mod chapter4;

fn main() {
    let num = 10;
    let bin_str = divide_by_two(num);
    println!("{num} : b{bin_str}")
}
