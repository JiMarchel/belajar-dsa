use chapter4::{base_converter::base_converter, divide_by_two::divide_by_two};

mod chapter4;

fn main() {
    let num1 = 10;
    let num2 = 43;
    let bin_str = base_converter(num1, 2);
    let hex_str = base_converter(num2, 16);
    println!("{num1} : b{bin_str}, {num2} : x{hex_str}")
}
