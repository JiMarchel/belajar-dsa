use learn_macro::macroo::{four, multiply_add, times_five, vec_strs};

mod basic_data_structure;
mod learn_macro;
mod searching;
mod sorting;

fn main() {
    let x = four!();
    println!("x: {}", x);

    let a = times_five!(x);
    println!("a: {}", a);

    let b = multiply_add!(5, 2, 3);
    println!("b: {}", b);

    let c = vec_strs![1, 45, true, 9.212];
    println!("c: {:?}", c);
}
